use std::{cmp::Ordering, error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Nil,
    Logical,
    Integer,
    Float,
    String,
    Array,
}

impl ValueKind {
    pub fn type_name(self) -> &'static str {
        match self {
            Self::Nil => "Nil",
            Self::Logical => "Logical",
            Self::Integer => "Integer",
            Self::Float => "Float",
            Self::String => "String",
            Self::Array => "Array",
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
    Array(Vec<Value>),
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Nil => ValueKind::Nil,
            Self::Logical(_) => ValueKind::Logical,
            Self::Integer(_) => ValueKind::Integer,
            Self::Float(_) => ValueKind::Float,
            Self::String(_) => ValueKind::String,
            Self::Array(_) => ValueKind::Array,
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

    pub fn as_array(&self) -> Result<&[Value], RuntimeError> {
        match self {
            Self::Array(values) => Ok(values),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to array",
                self.kind(),
            )),
        }
    }

    pub fn as_array_mut(&mut self) -> Result<&mut Vec<Value>, RuntimeError> {
        match self {
            Self::Array(values) => Ok(values),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to array",
                self.kind(),
            )),
        }
    }

    pub fn array(values: Vec<Value>) -> Self {
        Self::Array(values)
    }

    pub fn empty_array() -> Self {
        Self::Array(Vec::new())
    }

    pub fn array_with_len(len: usize) -> Self {
        Self::Array(vec![Self::Nil; len])
    }

    pub fn array_len(&self) -> Result<usize, RuntimeError> {
        self.as_array().map(|values| values.len())
    }

    pub fn array_get(&self, index: &Self) -> Result<&Value, RuntimeError> {
        let values = self.as_array()?;
        let zero_based_index = array_index_to_zero_based(index, values.len())?;
        values.get(zero_based_index).ok_or_else(|| {
            RuntimeError::array_index_out_of_bounds(array_index_integer(index), values.len())
        })
    }

    pub fn array_get_path(&self, indices: &[Value]) -> Result<&Value, RuntimeError> {
        let mut current = self;
        for index in indices {
            current = current.array_get(index)?;
        }

        Ok(current)
    }

    pub fn array_get_owned(&self, index: &Self) -> Result<Self, RuntimeError> {
        self.array_get(index).cloned()
    }

    pub fn array_get_mut(&mut self, index: &Self) -> Result<&mut Value, RuntimeError> {
        let values = self.as_array_mut()?;
        let len = values.len();
        let zero_based_index = array_index_to_zero_based(index, len)?;
        values
            .get_mut(zero_based_index)
            .ok_or_else(|| RuntimeError::array_index_out_of_bounds(array_index_integer(index), len))
    }

    pub fn array_set(&mut self, index: &Self, value: Self) -> Result<Self, RuntimeError> {
        let slot = self.array_get_mut(index)?;
        *slot = value.clone();
        Ok(value)
    }

    pub fn array_set_path(&mut self, indices: &[Value], value: Self) -> Result<Self, RuntimeError> {
        let (first, rest) = indices
            .split_first()
            .ok_or_else(RuntimeError::array_assignment_path_empty)?;

        if rest.is_empty() {
            return self.array_set(first, value);
        }

        let nested = self.array_get_mut(first)?;
        nested.array_set_path(rest, value)
    }

    pub fn array_resize(&mut self, len: usize) -> Result<(), RuntimeError> {
        let values = self.as_array_mut()?;
        values.resize(len, Self::Nil);
        Ok(())
    }

    pub fn array_push(&mut self, value: Self) -> Result<Self, RuntimeError> {
        let values = self.as_array_mut()?;
        values.push(value.clone());
        Ok(value)
    }

    pub fn array_clone(&self) -> Result<Self, RuntimeError> {
        self.as_array().map(|values| Self::Array(values.to_vec()))
    }

    pub fn to_output_string(&self) -> String {
        match self {
            Self::Nil => "NIL".to_owned(),
            Self::Logical(true) => ".T.".to_owned(),
            Self::Logical(false) => ".F.".to_owned(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Array(values) => format!("{{ Array({}) }}", values.len()),
        }
    }

    pub fn to_print_string(&self) -> String {
        self.to_output_string()
    }

    pub fn add(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        match (self, rhs) {
            (Self::String(left), Self::String(right)) => {
                let mut value = left.clone();
                value.push_str(right);
                Ok(Self::String(value))
            }
            _ => match self.numeric_pair(rhs, "add")? {
                NumericPair::Integers(left, right) => Ok(Self::Integer(left + right)),
                NumericPair::Floats(left, right) => Ok(Self::Float(left + right)),
            },
        }
    }

    pub fn subtract(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        match self.numeric_pair(rhs, "subtract")? {
            NumericPair::Integers(left, right) => Ok(Self::Integer(left - right)),
            NumericPair::Floats(left, right) => Ok(Self::Float(left - right)),
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        match self.numeric_pair(rhs, "multiply")? {
            NumericPair::Integers(left, right) => Ok(Self::Integer(left * right)),
            NumericPair::Floats(left, right) => Ok(Self::Float(left * right)),
        }
    }

    pub fn divide(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        let (left, right) = self.numeric_pair_as_float(rhs, "divide")?;
        if right == 0.0 {
            return Err(RuntimeError::division_by_zero());
        }

        Ok(Self::Float(left / right))
    }

    pub fn equals(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        let result = match (self, rhs) {
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) | (_, Self::Nil) => false,
            (Self::Logical(left), Self::Logical(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            _ => {
                if let Ok((left, right)) = self.numeric_pair_as_float(rhs, "compare equality") {
                    left == right
                } else {
                    return Err(RuntimeError::binary_operator_mismatch(
                        "compare equality",
                        self.kind(),
                        rhs.kind(),
                    ));
                }
            }
        };

        Ok(Self::Logical(result))
    }

    pub fn exact_equals(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        let result = match (self, rhs) {
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) | (_, Self::Nil) => false,
            (Self::Logical(left), Self::Logical(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Array(_), Self::Array(_)) => std::ptr::eq(self, rhs),
            _ => {
                if let Ok((left, right)) = self.numeric_pair_as_float(rhs, "compare exact equality")
                {
                    left == right
                } else {
                    return Err(RuntimeError::binary_operator_mismatch(
                        "compare exact equality",
                        self.kind(),
                        rhs.kind(),
                    ));
                }
            }
        };

        Ok(Self::Logical(result))
    }

    pub fn not_equals(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        match self.equals(rhs)? {
            Self::Logical(value) => Ok(Self::Logical(!value)),
            _ => unreachable!("equals always returns Value::Logical"),
        }
    }

    pub fn exact_not_equals(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        match self.exact_equals(rhs)? {
            Self::Logical(value) => Ok(Self::Logical(!value)),
            _ => unreachable!("exact_equals always returns Value::Logical"),
        }
    }

    pub fn less_than(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare ordering")
            .map(|ordering| Self::Logical(ordering == Ordering::Less))
    }

    pub fn less_than_or_equal(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare ordering")
            .map(|ordering| Self::Logical(ordering != Ordering::Greater))
    }

    pub fn greater_than(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare ordering")
            .map(|ordering| Self::Logical(ordering == Ordering::Greater))
    }

    pub fn greater_than_or_equal(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare ordering")
            .map(|ordering| Self::Logical(ordering != Ordering::Less))
    }

    fn numeric_pair(&self, rhs: &Self, operation: &str) -> Result<NumericPair, RuntimeError> {
        match (self, rhs) {
            (Self::Integer(left), Self::Integer(right)) => Ok(NumericPair::Integers(*left, *right)),
            _ => self
                .numeric_pair_as_float(rhs, operation)
                .map(|(left, right)| NumericPair::Floats(left, right)),
        }
    }

    fn numeric_pair_as_float(
        &self,
        rhs: &Self,
        operation: &str,
    ) -> Result<(f64, f64), RuntimeError> {
        match (self, rhs) {
            (Self::Integer(left), Self::Integer(right)) => Ok((*left as f64, *right as f64)),
            (Self::Integer(left), Self::Float(right)) => Ok((*left as f64, *right)),
            (Self::Float(left), Self::Integer(right)) => Ok((*left, *right as f64)),
            (Self::Float(left), Self::Float(right)) => Ok((*left, *right)),
            _ => Err(RuntimeError::binary_operator_mismatch(
                operation,
                self.kind(),
                rhs.kind(),
            )),
        }
    }

    fn compare_order(&self, rhs: &Self, operation: &str) -> Result<Ordering, RuntimeError> {
        match (self, rhs) {
            (Self::String(left), Self::String(right)) => Ok(left.cmp(right)),
            _ => {
                let (left, right) = self.numeric_pair_as_float(rhs, operation)?;
                left.partial_cmp(&right)
                    .ok_or_else(RuntimeError::invalid_float_comparison)
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutputBuffer {
    content: String,
}

impl OutputBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub fn into_string(self) -> String {
        self.content
    }

    fn push_qout_line(&mut self, values: &[Value]) {
        if values.is_empty() {
            self.content.push('\n');
            return;
        }

        let mut iter = values.iter();
        if let Some(first) = iter.next() {
            self.content.push_str(&first.to_print_string());
        }

        for value in iter {
            self.content.push(' ');
            self.content.push_str(&value.to_print_string());
        }

        self.content.push('\n');
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeContext {
    output: OutputBuffer,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(&self) -> &OutputBuffer {
        &self.output
    }

    pub fn output_mut(&mut self) -> &mut OutputBuffer {
        &mut self.output
    }

    pub fn into_output(self) -> OutputBuffer {
        self.output
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    QOut,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("QOUT") {
            Some(Self::QOut)
        } else {
            None
        }
    }
}

pub fn qout(values: &[Value], output: &mut OutputBuffer) -> Result<Value, RuntimeError> {
    output.push_qout_line(values);
    Ok(Value::Nil)
}

pub fn call_builtin(
    name: &str,
    arguments: &[Value],
    context: &mut RuntimeContext,
) -> Result<Value, RuntimeError> {
    match Builtin::lookup(name) {
        Some(Builtin::QOut) => qout(arguments, context.output_mut()),
        None => Err(RuntimeError::unknown_builtin(name)),
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

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(value)
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

impl TryFrom<&Value> for Vec<Value> {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.as_array().map(ToOwned::to_owned)
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

    pub fn binary_operator_mismatch(message: &str, left: ValueKind, right: ValueKind) -> Self {
        Self {
            message: format!(
                "{} with {} and {}",
                message,
                left.type_name(),
                right.type_name()
            ),
            expected: None,
            actual: None,
        }
    }

    pub fn division_by_zero() -> Self {
        Self {
            message: "divide by zero".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn invalid_float_comparison() -> Self {
        Self {
            message: "compare ordering with non-orderable Float".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn unknown_builtin(name: &str) -> Self {
        Self {
            message: format!("unknown builtin {}", name),
            expected: None,
            actual: None,
        }
    }

    pub fn array_index_type_mismatch(actual: ValueKind) -> Self {
        Self {
            message: "array index must be Integer".to_owned(),
            expected: Some(ValueKind::Integer),
            actual: Some(actual),
        }
    }

    pub fn array_index_out_of_bounds(index: i64, len: usize) -> Self {
        Self {
            message: format!("array index {} out of bounds for length {}", index, len),
            expected: None,
            actual: None,
        }
    }

    pub fn array_assignment_path_empty() -> Self {
        Self {
            message: "array assignment path must not be empty".to_owned(),
            expected: None,
            actual: None,
        }
    }
}

enum NumericPair {
    Integers(i64, i64),
    Floats(f64, f64),
}

fn array_index_integer(index: &Value) -> i64 {
    match index {
        Value::Integer(value) => *value,
        _ => unreachable!("array index integer helper only called after validation"),
    }
}

fn array_index_to_zero_based(index: &Value, len: usize) -> Result<usize, RuntimeError> {
    let index = match index {
        Value::Integer(value) => *value,
        _ => {
            return Err(RuntimeError::array_index_type_mismatch(index.kind()));
        }
    };

    if index <= 0 {
        return Err(RuntimeError::array_index_out_of_bounds(index, len));
    }

    let zero_based_index = (index - 1) as usize;
    if zero_based_index >= len {
        return Err(RuntimeError::array_index_out_of_bounds(index, len));
    }

    Ok(zero_based_index)
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
    use crate::{OutputBuffer, RuntimeContext, RuntimeError, Value, ValueKind, call_builtin, qout};

    #[test]
    fn value_kind_and_type_name_match_variants() {
        assert_eq!(Value::Nil.kind(), ValueKind::Nil);
        assert_eq!(Value::from(true).kind(), ValueKind::Logical);
        assert_eq!(Value::from(1_i64).kind(), ValueKind::Integer);
        assert_eq!(Value::from(1.5_f64).kind(), ValueKind::Float);
        assert_eq!(Value::from("abc").kind(), ValueKind::String);
        assert_eq!(Value::empty_array().kind(), ValueKind::Array);
        assert_eq!(Value::from("abc").type_name(), "String");
    }

    #[test]
    fn strict_and_promoted_value_conversions_work() {
        assert_eq!(Value::from(true).as_logical(), Ok(true));
        assert_eq!(Value::from(42_i64).as_integer(), Ok(42));
        assert_eq!(Value::from(42_i64).as_float(), Ok(42.0));
        assert_eq!(Value::from(1.5_f64).as_float(), Ok(1.5));
        assert_eq!(Value::from("harbour").as_str(), Ok("harbour"));
        assert_eq!(
            Value::array(vec![Value::from(1_i64)]).as_array(),
            Ok([Value::from(1_i64)].as_slice())
        );
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
        assert_eq!(
            Value::from("nope").as_array(),
            Err(RuntimeError {
                message: "convert value to array".to_owned(),
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
        assert_eq!(
            Value::array(vec![Value::from(1_i64), Value::from(2_i64)]).to_output_string(),
            "{ Array(2) }"
        );
    }

    #[test]
    fn array_constructors_produce_expected_baseline_values() {
        assert_eq!(Value::empty_array(), Value::Array(Vec::new()));
        assert_eq!(
            Value::array_with_len(3),
            Value::Array(vec![Value::Nil, Value::Nil, Value::Nil])
        );
        assert_eq!(
            Value::array(vec![Value::from(1_i64), Value::from("x")]),
            Value::Array(vec![Value::from(1_i64), Value::from("x")])
        );
    }

    #[test]
    fn array_index_helpers_follow_one_based_runtime_baseline() {
        let matrix = Value::array(vec![
            Value::array(vec![Value::from(10_i64), Value::from(20_i64)]),
            Value::array(vec![Value::from(30_i64), Value::from(40_i64)]),
        ]);

        assert_eq!(matrix.array_len(), Ok(2));
        assert_eq!(
            matrix.array_get(&Value::from(1_i64)),
            Ok(&Value::array(vec![
                Value::from(10_i64),
                Value::from(20_i64),
            ]))
        );
        assert_eq!(
            matrix.array_get_path(&[Value::from(2_i64), Value::from(1_i64)]),
            Ok(&Value::from(30_i64))
        );
        assert_eq!(
            matrix.array_get_owned(&Value::from(2_i64)),
            Ok(Value::array(vec![Value::from(30_i64), Value::from(40_i64)]))
        );
    }

    #[test]
    fn array_set_helpers_support_one_based_updates_and_nested_assignment_paths() {
        let mut matrix = Value::array(vec![
            Value::array(vec![Value::from(10_i64), Value::from(20_i64)]),
            Value::array(vec![Value::from(30_i64), Value::from(40_i64)]),
        ]);

        assert_eq!(
            matrix.array_set(&Value::from(1_i64), Value::array(vec![Value::from(99_i64)])),
            Ok(Value::array(vec![Value::from(99_i64)]))
        );
        assert_eq!(
            matrix.array_get(&Value::from(1_i64)),
            Ok(&Value::array(vec![Value::from(99_i64)]))
        );
        assert_eq!(
            matrix.array_set_path(
                &[Value::from(2_i64), Value::from(1_i64)],
                Value::from("updated"),
            ),
            Ok(Value::from("updated"))
        );
        assert_eq!(
            matrix.array_get_path(&[Value::from(2_i64), Value::from(1_i64)]),
            Ok(&Value::from("updated"))
        );
    }

    #[test]
    fn array_collection_helpers_cover_resize_push_and_clone() {
        let mut values = Value::array(vec![Value::from(10_i64)]);

        assert_eq!(
            values.array_push(Value::from("tail")),
            Ok(Value::from("tail"))
        );
        assert_eq!(
            values,
            Value::array(vec![Value::from(10_i64), Value::from("tail")])
        );

        assert_eq!(values.array_resize(4), Ok(()));
        assert_eq!(
            values,
            Value::array(vec![
                Value::from(10_i64),
                Value::from("tail"),
                Value::Nil,
                Value::Nil,
            ])
        );

        let cloned = values.array_clone();
        assert_eq!(cloned, Ok(values.clone()));

        assert_eq!(values.array_resize(1), Ok(()));
        assert_eq!(values, Value::array(vec![Value::from(10_i64)]));
        assert_eq!(
            cloned,
            Ok(Value::array(vec![
                Value::from(10_i64),
                Value::from("tail"),
                Value::Nil,
                Value::Nil,
            ]))
        );
    }

    #[test]
    fn invalid_array_indexing_reports_structured_runtime_errors() {
        let values = Value::array(vec![Value::from(10_i64), Value::from(20_i64)]);

        assert_eq!(
            Value::from("text").array_get(&Value::from(1_i64)),
            Err(RuntimeError {
                message: "convert value to array".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            values.array_get(&Value::from("1")),
            Err(RuntimeError {
                message: "array index must be Integer".to_owned(),
                expected: Some(ValueKind::Integer),
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            values.array_get(&Value::from(0_i64)),
            Err(RuntimeError {
                message: "array index 0 out of bounds for length 2".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.array_get(&Value::from(3_i64)),
            Err(RuntimeError {
                message: "array index 3 out of bounds for length 2".to_owned(),
                expected: None,
                actual: None,
            })
        );

        let mut mutable_values = values.clone();
        assert_eq!(
            mutable_values.array_set_path(&[], Value::from(1_i64)),
            Err(RuntimeError {
                message: "array assignment path must not be empty".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            mutable_values.array_set_path(&[Value::from(1_i64), Value::from(1_i64)], Value::Nil),
            Err(RuntimeError {
                message: "convert value to array".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
    }

    #[test]
    fn arithmetic_operations_cover_integer_float_and_string_cases() {
        assert_eq!(
            Value::from(2_i64).add(&Value::from(3_i64)),
            Ok(Value::from(5_i64))
        );
        assert_eq!(
            Value::from(2_i64).add(&Value::from(0.5_f64)),
            Ok(Value::from(2.5_f64))
        );
        assert_eq!(
            Value::from("har").add(&Value::from("bour")),
            Ok(Value::from("harbour"))
        );
        assert_eq!(
            Value::from(6_i64).subtract(&Value::from(2_i64)),
            Ok(Value::from(4_i64))
        );
        assert_eq!(
            Value::from(4_i64).multiply(&Value::from(2.5_f64)),
            Ok(Value::from(10.0_f64))
        );
        assert_eq!(
            Value::from(9_i64).divide(&Value::from(2_i64)),
            Ok(Value::from(4.5_f64))
        );
    }

    #[test]
    fn comparison_operations_cover_numbers_and_strings() {
        assert_eq!(
            Value::from(2_i64).less_than(&Value::from(3_i64)),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from(3_i64).greater_than_or_equal(&Value::from(3.0_f64)),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from("abc").equals(&Value::from("abc")),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from("abc").less_than(&Value::from("abd")),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::Nil.not_equals(&Value::from(false)),
            Ok(Value::from(true))
        );
    }

    #[test]
    fn exact_comparison_distinguishes_array_identity_from_value_equality() {
        let array = Value::array(vec![Value::from(1_i64), Value::from(2_i64)]);
        let clone = array.clone();

        assert_eq!(array.exact_equals(&array), Ok(Value::from(true)));
        assert_eq!(array.exact_equals(&clone), Ok(Value::from(false)));
        assert_eq!(array.exact_not_equals(&clone), Ok(Value::from(true)));
        assert_eq!(
            Value::from("abc").exact_equals(&Value::from("abc")),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from(2_i64).exact_equals(&Value::from(2.0_f64)),
            Ok(Value::from(true))
        );
    }

    #[test]
    fn invalid_runtime_operations_report_errors() {
        assert_eq!(
            Value::from(true).add(&Value::from(1_i64)),
            Err(RuntimeError {
                message: "add with Logical and Integer".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            Value::from(1_i64).divide(&Value::from(0_i64)),
            Err(RuntimeError {
                message: "divide by zero".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            Value::from(true).less_than(&Value::from(false)),
            Err(RuntimeError {
                message: "compare ordering with Logical and Logical".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn qout_formats_arguments_as_a_single_print_line() {
        let mut output = OutputBuffer::new();

        assert_eq!(
            qout(
                &[
                    Value::from("hello"),
                    Value::from(2_i64),
                    Value::from(true),
                    Value::Nil,
                ],
                &mut output,
            ),
            Ok(Value::Nil)
        );

        assert_eq!(output.as_str(), "hello 2 .T. NIL\n");
    }

    #[test]
    fn qout_without_arguments_emits_blank_line() {
        let mut output = OutputBuffer::new();

        assert_eq!(qout(&[], &mut output), Ok(Value::Nil));
        assert_eq!(output.as_str(), "\n");
    }

    #[test]
    fn builtin_dispatch_invokes_qout_case_insensitively() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "qout",
                &[Value::from("hello"), Value::from(7_i64)],
                &mut context,
            ),
            Ok(Value::Nil)
        );
        assert_eq!(context.output().as_str(), "hello 7\n");
    }

    #[test]
    fn unknown_builtin_reports_runtime_error() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("MissingBuiltin", &[], &mut context),
            Err(RuntimeError {
                message: "unknown builtin MissingBuiltin".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }
}
