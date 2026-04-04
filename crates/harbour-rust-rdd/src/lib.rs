use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

use harbour_rust_runtime::{Value, ValueKind};

pub const DBF_DBASE_III_VERSION: u8 = 0x03;
pub const DBF_FIELD_DESCRIPTOR_SIZE: usize = 32;
pub const DBF_HEADER_BASE_SIZE: usize = 32;
pub const DBF_HEADER_TERMINATOR: u8 = 0x0D;
pub const DBF_DELETED_FLAG: u8 = b'*';
pub const DBF_ACTIVE_FLAG: u8 = b' ';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DbfDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl DbfDate {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, RddError> {
        if !(1..=12).contains(&month) {
            return Err(RddError::invalid_format(format!(
                "invalid DBF month value {month}"
            )));
        }
        if !(1..=31).contains(&day) {
            return Err(RddError::invalid_format(format!(
                "invalid DBF day value {day}"
            )));
        }

        Ok(Self { year, month, day })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Character,
    Numeric,
    Logical,
    Date,
}

impl FieldType {
    pub fn from_code(code: u8) -> Result<Self, RddError> {
        match code {
            b'C' => Ok(Self::Character),
            b'N' => Ok(Self::Numeric),
            b'L' => Ok(Self::Logical),
            b'D' => Ok(Self::Date),
            _ => Err(RddError::unsupported_field_type(code as char)),
        }
    }

    pub fn code(self) -> u8 {
        match self {
            Self::Character => b'C',
            Self::Numeric => b'N',
            Self::Logical => b'L',
            Self::Date => b'D',
        }
    }

    pub fn type_name(self) -> &'static str {
        match self {
            Self::Character => "Character",
            Self::Numeric => "Numeric",
            Self::Logical => "Logical",
            Self::Date => "Date",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: String,
    pub field_type: FieldType,
    pub length: u8,
    pub decimals: u8,
    pub offset: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbfHeader {
    pub version: u8,
    pub last_update: DbfDate,
    pub record_count: u32,
    pub header_length: u16,
    pub record_length: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbfSchema {
    pub header: DbfHeader,
    pub fields: Vec<FieldDescriptor>,
}

impl DbfSchema {
    pub fn field(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields
            .iter()
            .find(|field| field.name.eq_ignore_ascii_case(name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordSnapshot {
    pub recno: usize,
    pub deleted: bool,
    pub values: Vec<(String, Value)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RddError {
    Io {
        path: PathBuf,
        message: String,
    },
    InvalidFormat(String),
    UnsupportedFieldType(char),
    FieldNotFound(String),
    NotPositioned,
    RecordOutOfBounds {
        requested: usize,
        record_count: usize,
    },
    TypeMismatch {
        field: String,
        expected: &'static str,
        found: ValueKind,
    },
    NumericParse {
        field: String,
        raw: String,
    },
}

impl RddError {
    pub fn io(path: &Path, error: impl fmt::Display) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat(message.into())
    }

    pub fn unsupported_field_type(code: char) -> Self {
        Self::UnsupportedFieldType(code)
    }

    pub fn field_not_found(name: impl Into<String>) -> Self {
        Self::FieldNotFound(name.into())
    }

    pub fn type_mismatch(
        field: impl Into<String>,
        expected: &'static str,
        found: ValueKind,
    ) -> Self {
        Self::TypeMismatch {
            field: field.into(),
            expected,
            found,
        }
    }
}

impl fmt::Display for RddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(f, "I/O error for {}: {}", path.display(), message)
            }
            Self::InvalidFormat(message) => write!(f, "invalid DBF format: {}", message),
            Self::UnsupportedFieldType(code) => {
                write!(f, "unsupported DBF field type `{}`", code)
            }
            Self::FieldNotFound(name) => write!(f, "field `{}` was not found", name),
            Self::NotPositioned => f.write_str("record cursor is not positioned"),
            Self::RecordOutOfBounds {
                requested,
                record_count,
            } => write!(
                f,
                "record {} is out of bounds for table with {} records",
                requested, record_count
            ),
            Self::TypeMismatch {
                field,
                expected,
                found,
            } => write!(
                f,
                "field `{}` expected {}, found {}",
                field,
                expected,
                found.type_name()
            ),
            Self::NumericParse { field, raw } => write!(
                f,
                "failed to parse numeric field `{}` from raw value `{}`",
                field, raw
            ),
        }
    }
}

impl Error for RddError {}

pub trait Rdd {
    fn schema(&self) -> &DbfSchema;
    fn close(&mut self) -> Result<(), RddError>;
    fn go_to(&mut self, recno: usize) -> Result<(), RddError>;
    fn skip(&mut self, count: i32) -> Result<(), RddError>;
    fn bof(&self) -> bool;
    fn eof(&self) -> bool;
    fn recno(&self) -> usize;
    fn rec_count(&self) -> usize;
    fn field_get(&self, name: &str) -> Result<Value, RddError>;
    fn field_put(&mut self, name: &str, value: Value) -> Result<(), RddError>;
    fn append_blank(&mut self) -> Result<(), RddError>;
    fn deleted(&self) -> Result<bool, RddError>;
    fn delete(&mut self) -> Result<(), RddError>;
    fn recall(&mut self) -> Result<(), RddError>;
    fn snapshot(&self) -> Result<RecordSnapshot, RddError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbfTable {
    path: PathBuf,
    schema: DbfSchema,
    current_record: usize,
    bof: bool,
    eof: bool,
    is_closed: bool,
}

impl DbfTable {
    pub fn new(path: PathBuf, schema: DbfSchema) -> Self {
        let eof = schema.header.record_count == 0;
        Self {
            path,
            schema,
            current_record: 0,
            bof: true,
            eof,
            is_closed: false,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn schema(&self) -> &DbfSchema {
        &self.schema
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DBF_ACTIVE_FLAG, DBF_DBASE_III_VERSION, DBF_DELETED_FLAG, DbfDate, FieldType, RddError,
    };

    #[test]
    fn field_type_roundtrip_matches_supported_subset() {
        for (code, expected) in [
            (b'C', FieldType::Character),
            (b'N', FieldType::Numeric),
            (b'L', FieldType::Logical),
            (b'D', FieldType::Date),
        ] {
            let parsed = FieldType::from_code(code).expect("field type");
            assert_eq!(parsed, expected);
            assert_eq!(parsed.code(), code);
        }
    }

    #[test]
    fn rejects_unsupported_field_type_code() {
        let error = FieldType::from_code(b'M').expect_err("unsupported field type");
        assert_eq!(error, RddError::unsupported_field_type('M'));
    }

    #[test]
    fn validates_dbf_date_ranges() {
        let date = DbfDate::new(2026, 4, 3).expect("valid date");
        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 4);
        assert_eq!(date.day, 3);

        assert!(matches!(
            DbfDate::new(2026, 13, 1),
            Err(RddError::InvalidFormat(_))
        ));
        assert!(matches!(
            DbfDate::new(2026, 4, 0),
            Err(RddError::InvalidFormat(_))
        ));
    }

    #[test]
    fn dbf_constants_match_expected_dbase_values() {
        assert_eq!(DBF_DBASE_III_VERSION, 0x03);
        assert_eq!(DBF_ACTIVE_FLAG, b' ');
        assert_eq!(DBF_DELETED_FLAG, b'*');
    }
}
