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
        let values = array_for_access(self)?;
        let zero_based_index =
            array_index_to_zero_based(index, values.len(), ArrayOperation::Access)?;
        values
            .get(zero_based_index)
            .ok_or_else(|| RuntimeError::array_access_out_of_bounds(array_index_integer(index)))
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
        let values = array_for_assign_mut(self)?;
        let len = values.len();
        let zero_based_index = array_index_to_zero_based(index, len, ArrayOperation::Assign)?;
        values
            .get_mut(zero_based_index)
            .ok_or_else(|| RuntimeError::array_assign_out_of_bounds(array_index_integer(index)))
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
        let values = array_for_assign_mut(self)?;
        values.resize(len, Self::Nil);
        Ok(())
    }

    pub fn array_push(&mut self, value: Self) -> Result<Self, RuntimeError> {
        let values = array_for_assign_mut(self)?;
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
        if self.is_array_comparison_target(rhs) {
            return Err(RuntimeError::array_comparison_equals());
        }

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
        if self.is_array_comparison_target(rhs) {
            return Err(RuntimeError::array_comparison_not_equals());
        }

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
        self.compare_order(rhs, "compare less-than")
            .map(|ordering| Self::Logical(ordering == Ordering::Less))
    }

    pub fn less_than_or_equal(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare less-than-or-equal")
            .map(|ordering| Self::Logical(ordering != Ordering::Greater))
    }

    pub fn greater_than(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare greater-than")
            .map(|ordering| Self::Logical(ordering == Ordering::Greater))
    }

    pub fn greater_than_or_equal(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        self.compare_order(rhs, "compare greater-than-or-equal")
            .map(|ordering| Self::Logical(ordering != Ordering::Less))
    }

    fn is_array_comparison_target(&self, rhs: &Self) -> bool {
        matches!(self, Self::Array(_)) || matches!(rhs, Self::Array(_))
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
        if self.is_array_comparison_target(rhs) {
            return Err(match operation {
                "compare less-than" => RuntimeError::array_comparison_less_than(),
                "compare less-than-or-equal" => RuntimeError::array_comparison_less_than_or_equal(),
                "compare greater-than" => RuntimeError::array_comparison_greater_than(),
                "compare greater-than-or-equal" => {
                    RuntimeError::array_comparison_greater_than_or_equal()
                }
                _ => RuntimeError::binary_operator_mismatch(operation, self.kind(), rhs.kind()),
            });
        }

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
    Len,
    SubStr,
    Left,
    Right,
    Upper,
    Lower,
    AAdd,
    ASize,
    AClone,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("QOUT") {
            Some(Self::QOut)
        } else if name.eq_ignore_ascii_case("LEN") {
            Some(Self::Len)
        } else if name.eq_ignore_ascii_case("SUBSTR") {
            Some(Self::SubStr)
        } else if name.eq_ignore_ascii_case("LEFT") {
            Some(Self::Left)
        } else if name.eq_ignore_ascii_case("RIGHT") {
            Some(Self::Right)
        } else if name.eq_ignore_ascii_case("UPPER") {
            Some(Self::Upper)
        } else if name.eq_ignore_ascii_case("LOWER") {
            Some(Self::Lower)
        } else if name.eq_ignore_ascii_case("AADD") {
            Some(Self::AAdd)
        } else if name.eq_ignore_ascii_case("ASIZE") {
            Some(Self::ASize)
        } else if name.eq_ignore_ascii_case("ACLONE") {
            Some(Self::AClone)
        } else {
            None
        }
    }
}

pub fn qout(values: &[Value], output: &mut OutputBuffer) -> Result<Value, RuntimeError> {
    output.push_qout_line(values);
    Ok(Value::Nil)
}

pub fn len(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::len_argument_error(None));
    };

    match value {
        Value::String(text) => Ok(Value::from(text.len() as i64)),
        Value::Array(values) => Ok(Value::from(values.len() as i64)),
        other => Err(RuntimeError::len_argument_error(Some(other.kind()))),
    }
}

pub fn substr(
    source: Option<&Value>,
    start: Option<&Value>,
    count: Option<&Value>,
) -> Result<Value, RuntimeError> {
    let Some(source) = source else {
        return Err(RuntimeError::substr_argument_error(None));
    };
    let Value::String(text) = source else {
        return Err(RuntimeError::substr_argument_error(Some(source.kind())));
    };

    let Some(start) = start else {
        return Err(RuntimeError::substr_argument_error(None));
    };
    let mut start = match start {
        Value::Integer(value) => *value,
        other => return Err(RuntimeError::substr_argument_error(Some(other.kind()))),
    };

    let mut count = match count {
        Some(Value::Integer(value)) => *value,
        Some(other) => return Err(RuntimeError::substr_argument_error(Some(other.kind()))),
        None => text.chars().count() as i64,
    };

    let characters: Vec<char> = text.chars().collect();
    let size = characters.len() as i64;

    if start > 0 {
        start -= 1;
        if start > size {
            count = 0;
        }
    }

    if count <= 0 {
        return Ok(Value::from(""));
    }

    if start < 0 {
        start += size;
    }

    let mut start_index = 0_i64;
    let mut available = size;
    if start > 0 {
        start_index = start;
        available = size - start;
    }

    if count > available {
        count = available;
    }

    if count <= 0 {
        return Ok(Value::from(""));
    }

    let start_index = start_index as usize;
    Ok(Value::from(string_slice(
        &characters,
        start_index,
        count as usize,
    )))
}

pub fn left(source: Option<&Value>, count: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(source) = source else {
        return Err(RuntimeError::left_argument_error(None));
    };
    let Value::String(text) = source else {
        return Err(RuntimeError::left_argument_error(Some(source.kind())));
    };

    let Some(count) = count else {
        return Err(RuntimeError::left_argument_error(None));
    };
    let count = match count {
        Value::Integer(value) => *value,
        other => return Err(RuntimeError::left_argument_error(Some(other.kind()))),
    };

    if count <= 0 {
        return Ok(Value::from(""));
    }

    let characters: Vec<char> = text.chars().collect();
    let count = usize::min(count as usize, characters.len());
    Ok(Value::from(string_slice(&characters, 0, count)))
}

pub fn right(source: Option<&Value>, count: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(source) = source else {
        return Ok(Value::from(""));
    };
    let Value::String(text) = source else {
        return Ok(Value::from(""));
    };

    let Some(count) = count else {
        return Ok(Value::from(""));
    };
    let count = match count {
        Value::Integer(value) => *value,
        _ => return Ok(Value::from("")),
    };

    if count <= 0 {
        return Ok(Value::from(""));
    }

    let characters: Vec<char> = text.chars().collect();
    if count as usize >= characters.len() {
        return Ok(Value::from(text.clone()));
    }

    let start = characters.len() - count as usize;
    Ok(Value::from(string_slice(
        &characters,
        start,
        count as usize,
    )))
}

pub fn upper(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::upper_argument_error(None));
    };
    let Value::String(text) = value else {
        return Err(RuntimeError::upper_argument_error(Some(value.kind())));
    };

    Ok(Value::from(text.to_ascii_uppercase()))
}

pub fn lower(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::lower_argument_error(None));
    };
    let Value::String(text) = value else {
        return Err(RuntimeError::lower_argument_error(Some(value.kind())));
    };

    Ok(Value::from(text.to_ascii_lowercase()))
}

pub fn aadd(array: &mut Value, value: Value) -> Result<Value, RuntimeError> {
    if matches!(array, Value::Array(_)) {
        array.array_push(value)
    } else {
        Ok(Value::Nil)
    }
}

pub fn asize(array: &mut Value, len: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(len) = len else {
        return Ok(Value::Nil);
    };

    if !matches!(array, Value::Array(_)) {
        return Ok(Value::Nil);
    }

    let target_len = match len {
        Value::Integer(value) if *value <= 0 => 0,
        Value::Integer(value) => *value as usize,
        _ => return Ok(Value::Nil),
    };

    array.array_resize(target_len)?;
    array.array_clone()
}

pub fn aclone(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Ok(Value::Nil);
    };

    if matches!(value, Value::Array(_)) {
        value.array_clone()
    } else {
        Ok(Value::Nil)
    }
}

pub fn call_builtin(
    name: &str,
    arguments: &[Value],
    context: &mut RuntimeContext,
) -> Result<Value, RuntimeError> {
    match Builtin::lookup(name) {
        Some(Builtin::QOut) => qout(arguments, context.output_mut()),
        Some(Builtin::Len) => len(arguments.first()),
        Some(Builtin::SubStr) => substr(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Left) => left(arguments.first(), arguments.get(1)),
        Some(Builtin::Right) => right(arguments.first(), arguments.get(1)),
        Some(Builtin::Upper) => upper(arguments.first()),
        Some(Builtin::Lower) => lower(arguments.first()),
        Some(Builtin::AClone) => aclone(arguments.first()),
        Some(Builtin::AAdd | Builtin::ASize) => {
            Err(RuntimeError::builtin_requires_mutable_dispatch(name))
        }
        None => Err(RuntimeError::unknown_builtin(name)),
    }
}

pub fn call_builtin_mut(
    name: &str,
    arguments: &mut [Value],
    context: &mut RuntimeContext,
) -> Result<Value, RuntimeError> {
    match Builtin::lookup(name) {
        Some(Builtin::QOut) => qout(arguments, context.output_mut()),
        Some(Builtin::Len) => len(arguments.first()),
        Some(Builtin::SubStr) => substr(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Left) => left(arguments.first(), arguments.get(1)),
        Some(Builtin::Right) => right(arguments.first(), arguments.get(1)),
        Some(Builtin::Upper) => upper(arguments.first()),
        Some(Builtin::Lower) => lower(arguments.first()),
        Some(Builtin::AClone) => aclone(arguments.first()),
        Some(Builtin::AAdd) => {
            let Some((array, rest)) = arguments.split_first_mut() else {
                return Ok(Value::Nil);
            };
            let Some(value) = rest.first() else {
                return Ok(Value::Nil);
            };
            aadd(array, value.clone())
        }
        Some(Builtin::ASize) => {
            let Some((array, rest)) = arguments.split_first_mut() else {
                return Ok(Value::Nil);
            };
            asize(array, rest.first())
        }
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

    pub fn builtin_requires_mutable_dispatch(name: &str) -> Self {
        Self {
            message: format!("builtin {} requires mutable dispatch", name),
            expected: None,
            actual: None,
        }
    }

    pub fn len_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1111 Argument error (LEN)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn substr_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1110 Argument error (SUBSTR)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn left_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1124 Argument error (LEFT)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn upper_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1102 Argument error (UPPER)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn lower_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1103 Argument error (LOWER)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn array_access_type_mismatch(actual: ValueKind) -> Self {
        Self {
            message: "BASE 1068 Argument error (array access)".to_owned(),
            expected: Some(ValueKind::Integer),
            actual: Some(actual),
        }
    }

    pub fn array_assign_type_mismatch(actual: ValueKind) -> Self {
        Self {
            message: "BASE 1069 Argument error (array assign)".to_owned(),
            expected: Some(ValueKind::Integer),
            actual: Some(actual),
        }
    }

    pub fn array_access_out_of_bounds(_index: i64) -> Self {
        Self {
            message: "BASE 1132 Bound error (array access)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_assign_out_of_bounds(_index: i64) -> Self {
        Self {
            message: "BASE 1133 Bound error (array assign)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_equals() -> Self {
        Self {
            message: "BASE 1071 Argument error (=)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_not_equals() -> Self {
        Self {
            message: "BASE 1072 Argument error (<>)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_less_than() -> Self {
        Self {
            message: "BASE 1073 Argument error (<)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_less_than_or_equal() -> Self {
        Self {
            message: "BASE 1074 Argument error (<=)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_greater_than() -> Self {
        Self {
            message: "BASE 1075 Argument error (>)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn array_comparison_greater_than_or_equal() -> Self {
        Self {
            message: "BASE 1076 Argument error (>=)".to_owned(),
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

#[derive(Debug, Clone, Copy)]
enum ArrayOperation {
    Access,
    Assign,
}

fn array_index_integer(index: &Value) -> i64 {
    match index {
        Value::Integer(value) => *value,
        _ => unreachable!("array index integer helper only called after validation"),
    }
}

fn array_for_access(value: &Value) -> Result<&[Value], RuntimeError> {
    match value {
        Value::Array(values) => Ok(values),
        _ => Err(RuntimeError::array_access_type_mismatch(value.kind())),
    }
}

fn array_for_assign_mut(value: &mut Value) -> Result<&mut Vec<Value>, RuntimeError> {
    match value {
        Value::Array(values) => Ok(values),
        _ => Err(RuntimeError::array_assign_type_mismatch(value.kind())),
    }
}

fn array_index_to_zero_based(
    index: &Value,
    len: usize,
    operation: ArrayOperation,
) -> Result<usize, RuntimeError> {
    let index = match index {
        Value::Integer(value) => *value,
        _ => {
            return Err(match operation {
                ArrayOperation::Access => RuntimeError::array_access_type_mismatch(index.kind()),
                ArrayOperation::Assign => RuntimeError::array_assign_type_mismatch(index.kind()),
            });
        }
    };

    if index <= 0 {
        return Err(match operation {
            ArrayOperation::Access => RuntimeError::array_access_out_of_bounds(index),
            ArrayOperation::Assign => RuntimeError::array_assign_out_of_bounds(index),
        });
    }

    let zero_based_index = (index - 1) as usize;
    if zero_based_index >= len {
        return Err(match operation {
            ArrayOperation::Access => RuntimeError::array_access_out_of_bounds(index),
            ArrayOperation::Assign => RuntimeError::array_assign_out_of_bounds(index),
        });
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

fn string_slice(characters: &[char], start: usize, count: usize) -> String {
    characters[start..start + count].iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        OutputBuffer, RuntimeContext, RuntimeError, Value, ValueKind, aadd, aclone, asize,
        call_builtin, call_builtin_mut, len, qout,
    };

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
                message: "BASE 1068 Argument error (array access)".to_owned(),
                expected: Some(ValueKind::Integer),
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            values.array_get(&Value::from("1")),
            Err(RuntimeError {
                message: "BASE 1068 Argument error (array access)".to_owned(),
                expected: Some(ValueKind::Integer),
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            values.array_get(&Value::from(0_i64)),
            Err(RuntimeError {
                message: "BASE 1132 Bound error (array access)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.array_get(&Value::from(3_i64)),
            Err(RuntimeError {
                message: "BASE 1132 Bound error (array access)".to_owned(),
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
            mutable_values.array_set(&Value::from("1"), Value::Nil),
            Err(RuntimeError {
                message: "BASE 1069 Argument error (array assign)".to_owned(),
                expected: Some(ValueKind::Integer),
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            mutable_values.array_set(&Value::from(3_i64), Value::Nil),
            Err(RuntimeError {
                message: "BASE 1133 Bound error (array assign)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            mutable_values.array_set_path(&[Value::from(1_i64), Value::from(1_i64)], Value::Nil),
            Err(RuntimeError {
                message: "BASE 1069 Argument error (array assign)".to_owned(),
                expected: Some(ValueKind::Integer),
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
    fn array_comparison_operations_follow_xbase_baseline_errors() {
        let values = Value::array(vec![Value::from(1_i64)]);

        assert_eq!(
            values.equals(&values),
            Err(RuntimeError {
                message: "BASE 1071 Argument error (=)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.not_equals(&values),
            Err(RuntimeError {
                message: "BASE 1072 Argument error (<>)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.less_than(&values),
            Err(RuntimeError {
                message: "BASE 1073 Argument error (<)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.less_than_or_equal(&values),
            Err(RuntimeError {
                message: "BASE 1074 Argument error (<=)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.greater_than(&values),
            Err(RuntimeError {
                message: "BASE 1075 Argument error (>)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            values.greater_than_or_equal(&values),
            Err(RuntimeError {
                message: "BASE 1076 Argument error (>=)".to_owned(),
                expected: None,
                actual: None,
            })
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
                message: "compare less-than with Logical and Logical".to_owned(),
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
    fn len_supports_strings_and_arrays_with_xbase_style_errors() {
        assert_eq!(len(Some(&Value::from("123"))), Ok(Value::from(3_i64)));
        assert_eq!(
            len(Some(&Value::array(vec![
                Value::from(1_i64),
                Value::from(2_i64),
                Value::from(3_i64),
            ]))),
            Ok(Value::from(3_i64))
        );
        assert_eq!(
            len(Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1111 Argument error (LEN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            len(Some(&Value::from(123_i64))),
            Err(RuntimeError {
                message: "BASE 1111 Argument error (LEN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
    }

    #[test]
    fn len_dispatches_through_the_immutable_builtin_surface() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("len", &[Value::from("abcd")], &mut context),
            Ok(Value::from(4_i64))
        );

        let mut mutable_arguments = [Value::array(vec![Value::from(1_i64), Value::from(2_i64)])];
        assert_eq!(
            call_builtin_mut("LEN", &mut mutable_arguments, &mut context),
            Ok(Value::from(2_i64))
        );

        assert_eq!(
            call_builtin("LEN", &[], &mut context),
            Err(RuntimeError {
                message: "BASE 1111 Argument error (LEN)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn array_builtins_mutate_the_first_argument_through_mutable_dispatch() {
        let mut context = RuntimeContext::new();
        let mut add_arguments = [Value::empty_array(), Value::from("tail")];

        assert_eq!(
            call_builtin_mut("AADD", &mut add_arguments, &mut context),
            Ok(Value::from("tail"))
        );
        assert_eq!(add_arguments[0], Value::array(vec![Value::from("tail")]));

        let mut size_arguments = [add_arguments[0].clone(), Value::from(3_i64)];
        assert_eq!(
            call_builtin_mut("ASIZE", &mut size_arguments, &mut context),
            Ok(Value::array(vec![
                Value::from("tail"),
                Value::Nil,
                Value::Nil,
            ]))
        );
        assert_eq!(
            size_arguments[0],
            Value::array(vec![Value::from("tail"), Value::Nil, Value::Nil])
        );
    }

    #[test]
    fn aclone_follows_the_current_lenient_runtime_baseline() {
        let values = Value::array(vec![
            Value::from(1_i64),
            Value::array(vec![Value::from("nested")]),
        ]);

        assert_eq!(aclone(None), Ok(Value::Nil));
        assert_eq!(aclone(Some(&Value::Nil)), Ok(Value::Nil));
        assert_eq!(aclone(Some(&Value::from("nope"))), Ok(Value::Nil));
        assert_eq!(aclone(Some(&values)), Ok(values.clone()));
    }

    #[test]
    fn aadd_and_asize_follow_the_current_lenient_runtime_baseline() {
        let mut values = Value::empty_array();

        assert_eq!(aadd(&mut values, Value::Nil), Ok(Value::Nil));
        assert_eq!(values, Value::array(vec![Value::Nil]));

        assert_eq!(
            asize(&mut values, Some(&Value::from(-1_i64))),
            Ok(Value::empty_array())
        );
        assert_eq!(values, Value::empty_array());

        let mut not_array = Value::from("nope");
        assert_eq!(aadd(&mut not_array, Value::from(1_i64)), Ok(Value::Nil));
        assert_eq!(
            asize(&mut not_array, Some(&Value::from(3_i64))),
            Ok(Value::Nil)
        );
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

    #[test]
    fn mutable_array_builtins_report_when_called_through_immutable_dispatch() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "AADD",
                &[Value::empty_array(), Value::from(1_i64)],
                &mut context
            ),
            Err(RuntimeError {
                message: "builtin AADD requires mutable dispatch".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            call_builtin(
                "ASIZE",
                &[Value::empty_array(), Value::from(2_i64)],
                &mut context
            ),
            Err(RuntimeError {
                message: "builtin ASIZE requires mutable dispatch".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn aclone_dispatches_through_the_immutable_builtin_surface() {
        let mut context = RuntimeContext::new();
        let values = Value::array(vec![
            Value::from(1_i64),
            Value::array(vec![Value::from("nested")]),
        ]);

        assert_eq!(
            call_builtin("ACLONE", std::slice::from_ref(&values), &mut context),
            Ok(values.clone())
        );
        assert_eq!(call_builtin("ACLONE", &[], &mut context), Ok(Value::Nil));

        let mut mutable_arguments = [values.clone()];
        assert_eq!(
            call_builtin_mut("ACLONE", &mut mutable_arguments, &mut context),
            Ok(values.clone())
        );
        assert_eq!(mutable_arguments[0], values);
    }
}
