use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Nil,
    Logical,
    Integer,
    Float,
    String,
}

impl ValueKind {
    pub fn type_name(self) -> &'static str {
        match self {
            Self::Nil => "Nil",
            Self::Logical => "Logical",
            Self::Integer => "Integer",
            Self::Float => "Float",
            Self::String => "String",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
    Nil,
    Logical(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Nil => ValueKind::Nil,
            Self::Logical(_) => ValueKind::Logical,
            Self::Integer(_) => ValueKind::Integer,
            Self::Float(_) => ValueKind::Float,
            Self::String(_) => ValueKind::String,
        }
    }

    pub fn type_name(&self) -> &'static str {
        self.kind().type_name()
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn as_logical(&self) -> Result<bool, RuntimeError> {
        match self {
            Self::Logical(value) => Ok(*value),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to logical",
                self.kind(),
            )),
        }
    }

    pub fn as_integer(&self) -> Result<i64, RuntimeError> {
        match self {
            Self::Integer(value) => Ok(*value),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to integer",
                self.kind(),
            )),
        }
    }

    pub fn as_float(&self) -> Result<f64, RuntimeError> {
        match self {
            Self::Integer(value) => Ok(*value as f64),
            Self::Float(value) => Ok(*value),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to float",
                self.kind(),
            )),
        }
    }

    pub fn as_str(&self) -> Result<&str, RuntimeError> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to string",
                self.kind(),
            )),
        }
    }

    pub fn to_output_string(&self) -> String {
        match self {
            Self::Nil => "NIL".to_owned(),
            Self::Logical(true) => ".T.".to_owned(),
            Self::Logical(false) => ".F.".to_owned(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::String(value) => value.clone(),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Nil
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Logical(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl TryFrom<&Value> for bool {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_logical()
    }
}

impl TryFrom<&Value> for i64 {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_integer()
    }
}

impl TryFrom<&Value> for f64 {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_float()
    }
}

impl TryFrom<&Value> for String {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_str().map(ToOwned::to_owned)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
    pub message: String,
    pub expected: Option<ValueKind>,
    pub actual: Option<ValueKind>,
}

impl RuntimeError {
    pub fn type_mismatch(message: &str, actual: ValueKind) -> Self {
        Self {
            message: message.to_owned(),
            expected: None,
            actual: Some(actual),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.expected, self.actual) {
            (Some(expected), Some(actual)) => write!(
                f,
                "{} (expected {}, found {})",
                self.message,
                expected.type_name(),
                actual.type_name()
            ),
            (None, Some(actual)) => {
                write!(f, "{} (found {})", self.message, actual.type_name())
            }
            (Some(expected), None) => {
                write!(f, "{} (expected {})", self.message, expected.type_name())
            }
            (None, None) => f.write_str(&self.message),
        }
    }
}

impl Error for RuntimeError {}

#[cfg(test)]
mod tests {
    use crate::{RuntimeError, Value, ValueKind};

    #[test]
    fn value_kind_and_type_name_match_variants() {
        assert_eq!(Value::Nil.kind(), ValueKind::Nil);
        assert_eq!(Value::from(true).kind(), ValueKind::Logical);
        assert_eq!(Value::from(1_i64).kind(), ValueKind::Integer);
        assert_eq!(Value::from(1.5_f64).kind(), ValueKind::Float);
        assert_eq!(Value::from("abc").kind(), ValueKind::String);
        assert_eq!(Value::from("abc").type_name(), "String");
    }

    #[test]
    fn strict_and_promoted_value_conversions_work() {
        assert_eq!(Value::from(true).as_logical(), Ok(true));
        assert_eq!(Value::from(42_i64).as_integer(), Ok(42));
        assert_eq!(Value::from(42_i64).as_float(), Ok(42.0));
        assert_eq!(Value::from(1.5_f64).as_float(), Ok(1.5));
        assert_eq!(Value::from("harbour").as_str(), Ok("harbour"));
    }

    #[test]
    fn invalid_conversion_reports_runtime_error() {
        assert_eq!(
            Value::from("nope").as_integer(),
            Err(RuntimeError {
                message: "convert value to integer".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
    }

    #[test]
    fn output_format_uses_clipper_style_primitives() {
        assert_eq!(Value::Nil.to_output_string(), "NIL");
        assert_eq!(Value::from(true).to_output_string(), ".T.");
        assert_eq!(Value::from(false).to_output_string(), ".F.");
        assert_eq!(Value::from(12_i64).to_output_string(), "12");
        assert_eq!(Value::from("abc").to_output_string(), "abc");
    }
}
