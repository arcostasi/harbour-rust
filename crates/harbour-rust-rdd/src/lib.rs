use std::{
    error::Error,
    fmt, fs,
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
    pub fn read_from_path(path: &Path) -> Result<Self, RddError> {
        let bytes = fs::read(path).map_err(|error| RddError::io(path, error))?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RddError> {
        let header = parse_header(bytes)?;
        let fields = parse_field_descriptors(bytes, &header)?;
        Ok(Self { header, fields })
    }

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
    ValueEncoding {
        field: String,
        message: String,
    },
    UnsupportedOperation(&'static str),
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
            Self::ValueEncoding { field, message } => {
                write!(f, "failed to encode field `{}`: {}", field, message)
            }
            Self::UnsupportedOperation(operation) => {
                write!(f, "RDD operation `{}` is not implemented yet", operation)
            }
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
    bytes: Vec<u8>,
    schema: DbfSchema,
    current_record: usize,
    bof: bool,
    eof: bool,
    is_closed: bool,
}

impl DbfTable {
    pub fn open(path: &Path) -> Result<Self, RddError> {
        let bytes = fs::read(path).map_err(|error| RddError::io(path, error))?;
        Self::from_bytes(path.to_path_buf(), bytes)
    }

    pub fn from_bytes(path: PathBuf, bytes: Vec<u8>) -> Result<Self, RddError> {
        let schema = DbfSchema::from_bytes(&bytes)?;
        validate_record_storage(&bytes, &schema.header)?;
        Ok(Self::new(path, bytes, schema))
    }

    pub fn new(path: PathBuf, bytes: Vec<u8>, schema: DbfSchema) -> Self {
        let eof = schema.header.record_count == 0;
        Self {
            path,
            bytes,
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

    pub fn raw_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn schema(&self) -> &DbfSchema {
        &self.schema
    }

    pub fn bof(&self) -> bool {
        self.bof
    }

    pub fn eof(&self) -> bool {
        self.eof
    }

    pub fn recno(&self) -> usize {
        self.current_record
    }

    pub fn rec_count(&self) -> usize {
        self.schema.header.record_count as usize
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn ensure_open(&self) -> Result<(), RddError> {
        if self.is_closed {
            return Err(RddError::invalid_format(
                "operation attempted on closed DBF table",
            ));
        }
        Ok(())
    }

    fn ensure_positioned(&self) -> Result<usize, RddError> {
        self.ensure_open()?;
        if self.current_record == 0 {
            return Err(RddError::NotPositioned);
        }
        Ok(self.current_record)
    }

    fn record_offset(&self, recno: usize) -> Result<usize, RddError> {
        if recno == 0 || recno > self.rec_count() {
            return Err(RddError::RecordOutOfBounds {
                requested: recno,
                record_count: self.rec_count(),
            });
        }
        Ok(self.schema.header.header_length as usize
            + (recno - 1) * self.schema.header.record_length as usize)
    }

    fn record_bytes(&self, recno: usize) -> Result<&[u8], RddError> {
        let offset = self.record_offset(recno)?;
        let end = offset + self.schema.header.record_length as usize;
        Ok(&self.bytes[offset..end])
    }

    fn current_record_bytes(&self) -> Result<&[u8], RddError> {
        let recno = self.ensure_positioned()?;
        self.record_bytes(recno)
    }

    fn current_record_bytes_mut(&mut self) -> Result<&mut [u8], RddError> {
        let recno = self.ensure_positioned()?;
        let offset = self.record_offset(recno)?;
        let end = offset + self.schema.header.record_length as usize;
        Ok(&mut self.bytes[offset..end])
    }

    fn field_descriptor(&self, name: &str) -> Result<&FieldDescriptor, RddError> {
        self.schema
            .field(name)
            .ok_or_else(|| RddError::field_not_found(name))
    }

    fn decode_field(&self, field: &FieldDescriptor, record: &[u8]) -> Result<Value, RddError> {
        let start = field.offset as usize;
        let end = start + field.length as usize;
        let raw = &record[start..end];

        match field.field_type {
            FieldType::Character => Ok(Value::from(trim_right_ascii_spaces(raw))),
            FieldType::Numeric => parse_numeric_field(field, raw),
            FieldType::Logical => parse_logical_field(raw),
            FieldType::Date => parse_date_field(raw),
        }
    }

    fn move_before_first(&mut self) {
        self.current_record = 0;
        self.bof = true;
        self.eof = false;
    }

    fn move_after_last(&mut self) {
        self.current_record = 0;
        self.bof = false;
        self.eof = true;
    }

    fn sync_to_disk(&self) -> Result<(), RddError> {
        fs::write(&self.path, &self.bytes).map_err(|error| RddError::io(&self.path, error))
    }

    fn update_record_count_header(&mut self) {
        let bytes = self.schema.header.record_count.to_le_bytes();
        self.bytes[4..8].copy_from_slice(&bytes);
    }

    fn blank_record(&self) -> Vec<u8> {
        let mut record = vec![b' '; self.schema.header.record_length as usize];
        record[0] = DBF_ACTIVE_FLAG;
        record
    }
}

impl Rdd for DbfTable {
    fn schema(&self) -> &DbfSchema {
        &self.schema
    }

    fn close(&mut self) -> Result<(), RddError> {
        self.ensure_open()?;
        self.is_closed = true;
        Ok(())
    }

    fn go_to(&mut self, recno: usize) -> Result<(), RddError> {
        self.ensure_open()?;
        if recno == 0 || recno > self.rec_count() {
            return Err(RddError::RecordOutOfBounds {
                requested: recno,
                record_count: self.rec_count(),
            });
        }

        self.current_record = recno;
        self.bof = false;
        self.eof = false;
        Ok(())
    }

    fn skip(&mut self, count: i32) -> Result<(), RddError> {
        self.ensure_open()?;
        if self.rec_count() == 0 {
            self.move_after_last();
            return Ok(());
        }

        let base = if self.current_record == 0 {
            if self.bof {
                1
            } else if self.eof {
                self.rec_count()
            } else {
                1
            }
        } else {
            self.current_record
        };

        let target = base as i64 + count as i64;
        if target < 1 {
            self.move_before_first();
        } else if target > self.rec_count() as i64 {
            self.move_after_last();
        } else {
            self.current_record = target as usize;
            self.bof = false;
            self.eof = false;
        }
        Ok(())
    }

    fn bof(&self) -> bool {
        self.bof
    }

    fn eof(&self) -> bool {
        self.eof
    }

    fn recno(&self) -> usize {
        self.current_record
    }

    fn rec_count(&self) -> usize {
        self.rec_count()
    }

    fn field_get(&self, name: &str) -> Result<Value, RddError> {
        self.ensure_open()?;
        let field = self.field_descriptor(name)?;
        let record = self.current_record_bytes()?;
        self.decode_field(field, record)
    }

    fn field_put(&mut self, _name: &str, _value: Value) -> Result<(), RddError> {
        self.ensure_open()?;
        let field = self.field_descriptor(_name)?.clone();
        let encoded = encode_field_value(&field, &_value)?;
        let start = field.offset as usize;
        let end = start + field.length as usize;
        let record = self.current_record_bytes_mut()?;
        record[start..end].copy_from_slice(&encoded);
        self.sync_to_disk()
    }

    fn append_blank(&mut self) -> Result<(), RddError> {
        self.ensure_open()?;
        let record = self.blank_record();
        self.bytes.extend_from_slice(&record);
        self.schema.header.record_count += 1;
        self.update_record_count_header();
        self.current_record = self.schema.header.record_count as usize;
        self.bof = false;
        self.eof = false;
        self.sync_to_disk()
    }

    fn deleted(&self) -> Result<bool, RddError> {
        self.ensure_open()?;
        let record = self.current_record_bytes()?;
        Ok(record[0] == DBF_DELETED_FLAG)
    }

    fn delete(&mut self) -> Result<(), RddError> {
        self.ensure_open()?;
        let record = self.current_record_bytes_mut()?;
        record[0] = DBF_DELETED_FLAG;
        self.sync_to_disk()
    }

    fn recall(&mut self) -> Result<(), RddError> {
        self.ensure_open()?;
        let record = self.current_record_bytes_mut()?;
        record[0] = DBF_ACTIVE_FLAG;
        self.sync_to_disk()
    }

    fn snapshot(&self) -> Result<RecordSnapshot, RddError> {
        self.ensure_open()?;
        let record = self.current_record_bytes()?;
        let values = self
            .schema
            .fields
            .iter()
            .map(|field| {
                self.decode_field(field, record)
                    .map(|value| (field.name.clone(), value))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RecordSnapshot {
            recno: self.current_record,
            deleted: record[0] == DBF_DELETED_FLAG,
            values,
        })
    }
}

fn parse_header(bytes: &[u8]) -> Result<DbfHeader, RddError> {
    if bytes.len() < DBF_HEADER_BASE_SIZE {
        return Err(RddError::invalid_format(format!(
            "header is shorter than {} bytes",
            DBF_HEADER_BASE_SIZE
        )));
    }

    let version = bytes[0];
    if version != DBF_DBASE_III_VERSION {
        return Err(RddError::invalid_format(format!(
            "unsupported DBF version byte 0x{version:02X}"
        )));
    }

    let last_update = DbfDate::new(1900 + bytes[1] as u16, bytes[2], bytes[3])?;
    let record_count = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let header_length = u16::from_le_bytes([bytes[8], bytes[9]]);
    let record_length = u16::from_le_bytes([bytes[10], bytes[11]]);

    if header_length as usize > bytes.len() {
        return Err(RddError::invalid_format(format!(
            "header length {} exceeds file size {}",
            header_length,
            bytes.len()
        )));
    }
    if record_length == 0 {
        return Err(RddError::invalid_format("record length cannot be zero"));
    }
    if header_length < DBF_HEADER_BASE_SIZE as u16 + 1 {
        return Err(RddError::invalid_format(format!(
            "header length {} is too small for field terminator",
            header_length
        )));
    }

    Ok(DbfHeader {
        version,
        last_update,
        record_count,
        header_length,
        record_length,
    })
}

fn parse_field_descriptors(
    bytes: &[u8],
    header: &DbfHeader,
) -> Result<Vec<FieldDescriptor>, RddError> {
    let header_length = header.header_length as usize;
    let terminator_index = bytes[DBF_HEADER_BASE_SIZE..header_length]
        .iter()
        .position(|byte| *byte == DBF_HEADER_TERMINATOR)
        .map(|relative| DBF_HEADER_BASE_SIZE + relative)
        .ok_or_else(|| {
            RddError::invalid_format("DBF header does not contain a field terminator byte")
        })?;

    let field_bytes = terminator_index
        .checked_sub(DBF_HEADER_BASE_SIZE)
        .ok_or_else(|| RddError::invalid_format("field descriptor area underflowed"))?;

    if field_bytes % DBF_FIELD_DESCRIPTOR_SIZE != 0 {
        return Err(RddError::invalid_format(format!(
            "field descriptor section size {} is not aligned to {}",
            field_bytes, DBF_FIELD_DESCRIPTOR_SIZE
        )));
    }

    let mut fields = Vec::new();
    let mut offset = 1u16;
    for descriptor in
        bytes[DBF_HEADER_BASE_SIZE..terminator_index].chunks_exact(DBF_FIELD_DESCRIPTOR_SIZE)
    {
        let name = parse_field_name(&descriptor[..11])?;
        let field_type = FieldType::from_code(descriptor[11])?;
        let length = descriptor[16];
        let decimals = descriptor[17];

        if length == 0 {
            return Err(RddError::invalid_format(format!(
                "field `{}` has zero length",
                name
            )));
        }

        fields.push(FieldDescriptor {
            name,
            field_type,
            length,
            decimals,
            offset,
        });
        offset = offset
            .checked_add(length as u16)
            .ok_or_else(|| RddError::invalid_format("field offsets overflowed u16"))?;
    }

    if fields.is_empty() {
        return Err(RddError::invalid_format(
            "DBF table must contain at least one field",
        ));
    }

    let computed_record_length = offset;
    if computed_record_length != header.record_length {
        return Err(RddError::invalid_format(format!(
            "record length mismatch: header says {}, computed {}",
            header.record_length, computed_record_length
        )));
    }

    Ok(fields)
}

fn validate_record_storage(bytes: &[u8], header: &DbfHeader) -> Result<(), RddError> {
    let data_length = bytes.len().saturating_sub(header.header_length as usize);
    let required_length = header.record_count as usize * header.record_length as usize;
    if data_length < required_length {
        return Err(RddError::invalid_format(format!(
            "file does not contain all declared records: need {} bytes of record data, found {}",
            required_length, data_length
        )));
    }

    Ok(())
}

fn parse_field_name(raw: &[u8]) -> Result<String, RddError> {
    let end = raw.iter().position(|byte| *byte == 0).unwrap_or(raw.len());
    let name = std::str::from_utf8(&raw[..end])
        .map_err(|_| RddError::invalid_format("field name is not valid ASCII/UTF-8"))?
        .trim();

    if name.is_empty() {
        return Err(RddError::invalid_format("field name cannot be empty"));
    }

    Ok(name.to_owned())
}

fn trim_right_ascii_spaces(raw: &[u8]) -> String {
    let text = String::from_utf8_lossy(raw);
    text.trim_end_matches(' ').to_owned()
}

fn parse_numeric_field(field: &FieldDescriptor, raw: &[u8]) -> Result<Value, RddError> {
    let text = String::from_utf8_lossy(raw);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(Value::Nil);
    }

    if field.decimals == 0 && !trimmed.contains('.') {
        return trimmed
            .parse::<i64>()
            .map(Value::Integer)
            .map_err(|_| RddError::NumericParse {
                field: field.name.clone(),
                raw: trimmed.to_owned(),
            });
    }

    trimmed
        .parse::<f64>()
        .map(Value::Float)
        .map_err(|_| RddError::NumericParse {
            field: field.name.clone(),
            raw: trimmed.to_owned(),
        })
}

fn parse_logical_field(raw: &[u8]) -> Result<Value, RddError> {
    match raw.first().copied().unwrap_or(b' ') {
        b'T' | b't' | b'Y' | b'y' => Ok(Value::Logical(true)),
        b'F' | b'f' | b'N' | b'n' => Ok(Value::Logical(false)),
        b' ' | b'?' => Ok(Value::Nil),
        other => Err(RddError::invalid_format(format!(
            "invalid logical DBF byte 0x{other:02X}"
        ))),
    }
}

fn parse_date_field(raw: &[u8]) -> Result<Value, RddError> {
    let text = String::from_utf8_lossy(raw);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(Value::Nil);
    }
    if trimmed.len() != 8 || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(RddError::invalid_format(format!(
            "invalid DBF date payload `{trimmed}`"
        )));
    }
    Ok(Value::from(trimmed.to_owned()))
}

fn encode_field_value(field: &FieldDescriptor, value: &Value) -> Result<Vec<u8>, RddError> {
    let encoded = match field.field_type {
        FieldType::Character => encode_character_field(field, value)?,
        FieldType::Numeric => encode_numeric_field(field, value)?,
        FieldType::Logical => encode_logical_field(field, value)?,
        FieldType::Date => encode_date_field(field, value)?,
    };

    debug_assert_eq!(encoded.len(), field.length as usize);
    Ok(encoded)
}

fn encode_character_field(field: &FieldDescriptor, value: &Value) -> Result<Vec<u8>, RddError> {
    let text = match value {
        Value::String(value) => value.as_str(),
        Value::Nil => "",
        other => {
            return Err(RddError::type_mismatch(
                field.name.clone(),
                "Character/String",
                other.kind(),
            ));
        }
    };

    if text.len() > field.length as usize {
        return Err(RddError::ValueEncoding {
            field: field.name.clone(),
            message: format!(
                "string length {} exceeds field width {}",
                text.len(),
                field.length
            ),
        });
    }

    let mut bytes = vec![b' '; field.length as usize];
    bytes[..text.len()].copy_from_slice(text.as_bytes());
    Ok(bytes)
}

fn encode_numeric_field(field: &FieldDescriptor, value: &Value) -> Result<Vec<u8>, RddError> {
    let text = match value {
        Value::Nil => String::new(),
        Value::Integer(number) if field.decimals == 0 => number.to_string(),
        Value::Integer(number) => format!("{:.*}", field.decimals as usize, *number as f64),
        Value::Float(number) => format!("{:.*}", field.decimals as usize, number),
        other => {
            return Err(RddError::type_mismatch(
                field.name.clone(),
                "Numeric/Integer/Float",
                other.kind(),
            ));
        }
    };

    if text.len() > field.length as usize {
        return Err(RddError::ValueEncoding {
            field: field.name.clone(),
            message: format!(
                "numeric representation `{}` exceeds field width {}",
                text, field.length
            ),
        });
    }

    let mut bytes = vec![b' '; field.length as usize];
    let start = field.length as usize - text.len();
    bytes[start..].copy_from_slice(text.as_bytes());
    Ok(bytes)
}

fn encode_logical_field(field: &FieldDescriptor, value: &Value) -> Result<Vec<u8>, RddError> {
    let byte = match value {
        Value::Logical(true) => b'T',
        Value::Logical(false) => b'F',
        Value::Nil => b' ',
        other => {
            return Err(RddError::type_mismatch(
                field.name.clone(),
                "Logical",
                other.kind(),
            ));
        }
    };

    Ok(vec![byte])
}

fn encode_date_field(field: &FieldDescriptor, value: &Value) -> Result<Vec<u8>, RddError> {
    let text = match value {
        Value::String(value) => value.as_str(),
        Value::Nil => "",
        other => {
            return Err(RddError::type_mismatch(
                field.name.clone(),
                "Date/String",
                other.kind(),
            ));
        }
    };

    if !text.is_empty() && (text.len() != 8 || !text.bytes().all(|byte| byte.is_ascii_digit())) {
        return Err(RddError::ValueEncoding {
            field: field.name.clone(),
            message: "date values must use YYYYMMDD".to_owned(),
        });
    }

    let mut bytes = vec![b' '; field.length as usize];
    bytes[..text.len()].copy_from_slice(text.as_bytes());
    Ok(bytes)
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
