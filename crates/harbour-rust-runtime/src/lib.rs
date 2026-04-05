use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering as AtomicOrdering},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Nil,
    Logical,
    Integer,
    Float,
    String,
    Array,
    Codeblock,
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
            Self::Codeblock => "Codeblock",
        }
    }
}

static CODEBLOCK_ID_SEED: AtomicUsize = AtomicUsize::new(1);

type CodeblockImpl =
    dyn Fn(&[Value], &mut RuntimeContext) -> Result<Value, RuntimeError> + Send + Sync;

#[derive(Clone)]
pub struct CodeblockValue {
    id: usize,
    repr: String,
    implementation: Arc<CodeblockImpl>,
}

impl CodeblockValue {
    pub fn new<F>(repr: &str, implementation: F) -> Self
    where
        F: Fn(&[Value], &mut RuntimeContext) -> Result<Value, RuntimeError> + Send + Sync + 'static,
    {
        Self {
            id: CODEBLOCK_ID_SEED.fetch_add(1, AtomicOrdering::Relaxed),
            repr: repr.to_owned(),
            implementation: Arc::new(implementation),
        }
    }

    pub fn call(
        &self,
        arguments: &[Value],
        context: &mut RuntimeContext,
    ) -> Result<Value, RuntimeError> {
        (self.implementation)(arguments, context)
    }

    pub fn repr(&self) -> &str {
        &self.repr
    }
}

impl fmt::Debug for CodeblockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodeblockValue")
            .field("id", &self.id)
            .field("repr", &self.repr)
            .finish()
    }
}

impl PartialEq for CodeblockValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CodeblockValue {}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
    Nil,
    Logical(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Codeblock(CodeblockValue),
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
            Self::Codeblock(_) => ValueKind::Codeblock,
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

    pub fn as_codeblock(&self) -> Result<&CodeblockValue, RuntimeError> {
        match self {
            Self::Codeblock(value) => Ok(value),
            _ => Err(RuntimeError::type_mismatch(
                "convert value to codeblock",
                self.kind(),
            )),
        }
    }

    pub fn array(values: Vec<Value>) -> Self {
        Self::Array(values)
    }

    pub fn codeblock<F>(repr: &str, implementation: F) -> Self
    where
        F: Fn(&[Value], &mut RuntimeContext) -> Result<Value, RuntimeError> + Send + Sync + 'static,
    {
        Self::Codeblock(CodeblockValue::new(repr, implementation))
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

    pub fn array_insert(&mut self, index: usize) -> Result<(), RuntimeError> {
        let values = array_for_assign_mut(self)?;
        if index >= values.len() {
            return Ok(());
        }

        for position in (index + 1..values.len()).rev() {
            values[position] = values[position - 1].clone();
        }
        values[index] = Self::Nil;
        Ok(())
    }

    pub fn array_delete(&mut self, index: usize) -> Result<(), RuntimeError> {
        let values = array_for_assign_mut(self)?;
        if index >= values.len() {
            return Ok(());
        }

        for position in index..values.len().saturating_sub(1) {
            values[position] = values[position + 1].clone();
        }

        if let Some(last) = values.last_mut() {
            *last = Self::Nil;
        }

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
            Self::Codeblock(value) => value.repr().to_owned(),
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
            (Self::String(left), Self::String(right)) => string_equals_exact_off(left, right),
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
            (Self::Codeblock(left), Self::Codeblock(right)) => left == right,
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuntimeContext {
    output: OutputBuffer,
    private_frames: Vec<HashMap<String, Value>>,
    public_memvars: HashMap<String, Value>,
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

    pub fn push_private_frame(&mut self) {
        self.private_frames.push(HashMap::new());
    }

    pub fn pop_private_frame(&mut self) {
        self.private_frames.pop();
    }

    pub fn define_private(&mut self, name: &str, value: Value) -> Value {
        if self.private_frames.is_empty() {
            self.push_private_frame();
        }

        self.private_frames
            .last_mut()
            .expect("private frame")
            .insert(normalize_name(name), value.clone());
        value
    }

    pub fn define_public(&mut self, name: &str, value: Value) -> Value {
        self.public_memvars
            .insert(normalize_name(name), value.clone());
        value
    }

    pub fn read_memvar(&self, name: &str) -> Value {
        let key = normalize_name(name);

        for frame in self.private_frames.iter().rev() {
            if let Some(value) = frame.get(&key) {
                return value.clone();
            }
        }

        self.public_memvars.get(&key).cloned().unwrap_or(Value::Nil)
    }

    pub fn assign_memvar(&mut self, name: &str, value: Value) -> Value {
        let key = normalize_name(name);

        for frame in self.private_frames.iter_mut().rev() {
            if let Some(existing) = frame.get_mut(&key) {
                *existing = value.clone();
                return value;
            }
        }

        if let Some(existing) = self.public_memvars.get_mut(&key) {
            *existing = value.clone();
            return value;
        }

        if let Some(frame) = self.private_frames.last_mut() {
            frame.insert(key, value.clone());
            return value;
        }

        self.public_memvars.insert(key, value.clone());
        value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    QOut,
    Eval,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Exp,
    Log,
    Int,
    Round,
    Mod,
    Max,
    Min,
    Len,
    Str,
    Val,
    ValType,
    Type,
    Empty,
    SubStr,
    Left,
    Right,
    Upper,
    Lower,
    Trim,
    LTrim,
    RTrim,
    At,
    Replicate,
    Space,
    AAdd,
    ASize,
    AClone,
    ADel,
    AIns,
    AScan,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("QOUT") {
            Some(Self::QOut)
        } else if name.eq_ignore_ascii_case("EVAL") {
            Some(Self::Eval)
        } else if name.eq_ignore_ascii_case("ABS") {
            Some(Self::Abs)
        } else if name.eq_ignore_ascii_case("SQRT") {
            Some(Self::Sqrt)
        } else if name.eq_ignore_ascii_case("SIN") {
            Some(Self::Sin)
        } else if name.eq_ignore_ascii_case("COS") {
            Some(Self::Cos)
        } else if name.eq_ignore_ascii_case("TAN") {
            Some(Self::Tan)
        } else if name.eq_ignore_ascii_case("EXP") {
            Some(Self::Exp)
        } else if name.eq_ignore_ascii_case("LOG") {
            Some(Self::Log)
        } else if name.eq_ignore_ascii_case("INT") {
            Some(Self::Int)
        } else if name.eq_ignore_ascii_case("ROUND") {
            Some(Self::Round)
        } else if name.eq_ignore_ascii_case("MOD") {
            Some(Self::Mod)
        } else if name.eq_ignore_ascii_case("MAX") {
            Some(Self::Max)
        } else if name.eq_ignore_ascii_case("MIN") {
            Some(Self::Min)
        } else if name.eq_ignore_ascii_case("LEN") {
            Some(Self::Len)
        } else if name.eq_ignore_ascii_case("STR") {
            Some(Self::Str)
        } else if name.eq_ignore_ascii_case("VAL") {
            Some(Self::Val)
        } else if name.eq_ignore_ascii_case("VALTYPE") {
            Some(Self::ValType)
        } else if name.eq_ignore_ascii_case("TYPE") {
            Some(Self::Type)
        } else if name.eq_ignore_ascii_case("EMPTY") {
            Some(Self::Empty)
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
        } else if name.eq_ignore_ascii_case("TRIM") {
            Some(Self::Trim)
        } else if name.eq_ignore_ascii_case("LTRIM") {
            Some(Self::LTrim)
        } else if name.eq_ignore_ascii_case("RTRIM") {
            Some(Self::RTrim)
        } else if name.eq_ignore_ascii_case("AT") {
            Some(Self::At)
        } else if name.eq_ignore_ascii_case("REPLICATE") {
            Some(Self::Replicate)
        } else if name.eq_ignore_ascii_case("SPACE") {
            Some(Self::Space)
        } else if name.eq_ignore_ascii_case("AADD") {
            Some(Self::AAdd)
        } else if name.eq_ignore_ascii_case("ASIZE") {
            Some(Self::ASize)
        } else if name.eq_ignore_ascii_case("ACLONE") {
            Some(Self::AClone)
        } else if name.eq_ignore_ascii_case("ADEL") {
            Some(Self::ADel)
        } else if name.eq_ignore_ascii_case("AINS") {
            Some(Self::AIns)
        } else if name.eq_ignore_ascii_case("ASCAN") {
            Some(Self::AScan)
        } else {
            None
        }
    }
}

pub fn qout(values: &[Value], output: &mut OutputBuffer) -> Result<Value, RuntimeError> {
    output.push_qout_line(values);
    Ok(Value::Nil)
}

pub fn eval(
    codeblock: Option<&Value>,
    arguments: &[Value],
    context: &mut RuntimeContext,
) -> Result<Value, RuntimeError> {
    let Some(codeblock) = codeblock else {
        return Err(RuntimeError::eval_argument_error(None));
    };

    let codeblock = codeblock
        .as_codeblock()
        .map_err(|_| RuntimeError::eval_argument_error(Some(codeblock.kind())))?;

    codeblock.call(arguments, context)
}

pub fn abs(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::abs_argument_error(None));
    };

    match value {
        Value::Integer(number) => {
            if let Some(absolute) = number.checked_abs() {
                Ok(Value::from(absolute))
            } else {
                Ok(Value::from((*number as i128).abs() as f64))
            }
        }
        Value::Float(number) => Ok(Value::from(number.abs())),
        other => Err(RuntimeError::abs_argument_error(Some(other.kind()))),
    }
}

pub fn sqrt_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::sqrt_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::sqrt_argument_error(Some(other.kind()))),
    };

    if number <= 0.0 {
        return Ok(Value::from(0.0_f64));
    }

    Ok(Value::from(number.sqrt()))
}

pub fn sin_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::sin_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::sin_argument_error(Some(other.kind()))),
    };

    Ok(Value::from(number.sin()))
}

pub fn cos_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::cos_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::cos_argument_error(Some(other.kind()))),
    };

    Ok(Value::from(number.cos()))
}

pub fn tan_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::tan_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::tan_argument_error(Some(other.kind()))),
    };

    Ok(Value::from(number.tan()))
}

pub fn exp_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::exp_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::exp_argument_error(Some(other.kind()))),
    };

    Ok(Value::from(number.exp()))
}

pub fn log_value(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::log_argument_error(None));
    };

    let number = match value {
        Value::Integer(number) => *number as f64,
        Value::Float(number) => *number,
        other => return Err(RuntimeError::log_argument_error(Some(other.kind()))),
    };

    if number <= 0.0 {
        return Ok(Value::from(f64::NEG_INFINITY));
    }

    Ok(Value::from(number.ln()))
}

pub fn int(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::int_argument_error(None));
    };

    match value {
        Value::Integer(number) => Ok(Value::from(*number)),
        Value::Float(number) => {
            let truncated = number.trunc();
            if truncated >= i64::MIN as f64 && truncated <= i64::MAX as f64 {
                Ok(Value::from(truncated as i64))
            } else {
                Ok(Value::from(truncated))
            }
        }
        other => Err(RuntimeError::int_argument_error(Some(other.kind()))),
    }
}

pub fn round_value(
    number: Option<&Value>,
    decimals: Option<&Value>,
) -> Result<Value, RuntimeError> {
    let Some(number) = number else {
        return Err(RuntimeError::round_argument_error(None));
    };

    let value = match number {
        Value::Integer(value) => *value as f64,
        Value::Float(value) => *value,
        other => return Err(RuntimeError::round_argument_error(Some(other.kind()))),
    };

    let decimals = numeric_required_i64(decimals, RuntimeError::round_argument_error)?;

    if decimals == 0 && matches!(number, Value::Integer(_)) {
        return Ok(number.clone());
    }

    let rounded = round_with_decimals(value, decimals);
    if decimals <= 0 && rounded >= i64::MIN as f64 && rounded <= i64::MAX as f64 {
        return Ok(Value::from(rounded as i64));
    }

    Ok(Value::from(rounded))
}

pub fn mod_value(number: Option<&Value>, divisor: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(number) = number else {
        return Err(RuntimeError::mod_argument_error(None));
    };
    let Some(divisor) = divisor else {
        return Err(RuntimeError::mod_argument_error(None));
    };

    let number = match number {
        Value::Integer(value) => *value as f64,
        Value::Float(value) => *value,
        other => return Err(RuntimeError::mod_argument_error(Some(other.kind()))),
    };
    let divisor = match divisor {
        Value::Integer(value) => *value as f64,
        Value::Float(value) => *value,
        other => return Err(RuntimeError::mod_argument_error(Some(other.kind()))),
    };

    if divisor == 0.0 {
        return Err(RuntimeError::mod_zero_divisor());
    }

    let mut result = number % divisor;
    if result != 0.0 && ((number > 0.0 && divisor < 0.0) || (number < 0.0 && divisor > 0.0)) {
        result += divisor;
    }

    if result == 0.0 {
        result = 0.0;
    }

    Ok(Value::from(result))
}

pub fn max_value(left: Option<&Value>, right: Option<&Value>) -> Result<Value, RuntimeError> {
    extremum_value(left, right, ExtremumKind::Max)
}

pub fn min_value(left: Option<&Value>, right: Option<&Value>) -> Result<Value, RuntimeError> {
    extremum_value(left, right, ExtremumKind::Min)
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

pub fn str_value(
    number: Option<&Value>,
    width: Option<&Value>,
    decimals: Option<&Value>,
) -> Result<Value, RuntimeError> {
    let Some(number) = number else {
        return Err(RuntimeError::str_argument_error(None));
    };

    let numeric = match number {
        Value::Integer(value) => StrNumeric::Integer(*value),
        Value::Float(value) => StrNumeric::Float(*value),
        other => return Err(RuntimeError::str_argument_error(Some(other.kind()))),
    };

    let width = numeric_optional_i64(width, RuntimeError::str_argument_error)?;
    let decimals = numeric_optional_i64(decimals, RuntimeError::str_argument_error)?;

    if matches!(numeric, StrNumeric::Float(value) if !value.is_finite()) {
        return Ok(Value::from(format_non_finite_str(width)));
    }

    let formatted = if let Some(decimals) = decimals {
        format_str_fixed(numeric, decimals.max(0) as usize)
    } else if width.is_some() {
        format_str_rounded(numeric)
    } else {
        format_str_default(numeric)
    };

    Ok(Value::from(apply_str_width(formatted, width)))
}

pub fn val(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::val_argument_error(None));
    };
    let Value::String(text) = value else {
        return Err(RuntimeError::val_argument_error(Some(value.kind())));
    };

    Ok(parse_val_string(text))
}

pub fn valtype(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let type_code = match value.unwrap_or(&Value::Nil) {
        Value::Nil => "U",
        Value::Logical(_) => "L",
        Value::Integer(_) | Value::Float(_) => "N",
        Value::String(_) => "C",
        Value::Array(_) => "A",
        Value::Codeblock(_) => "B",
    };

    Ok(Value::from(type_code))
}

pub fn type_value(source: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(source) = source else {
        return Err(RuntimeError::type_argument_error(None));
    };
    let Value::String(source) = source else {
        return Err(RuntimeError::type_argument_error(Some(source.kind())));
    };

    Ok(Value::from(type_from_source_text(source)))
}

pub fn empty(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let is_empty = match value.unwrap_or(&Value::Nil) {
        Value::Nil => true,
        Value::Logical(value) => !value,
        Value::Integer(value) => *value == 0,
        Value::Float(value) => *value == 0.0,
        Value::String(text) => harbour_string_is_empty(text),
        Value::Array(values) => values.is_empty(),
        Value::Codeblock(_) => false,
    };

    Ok(Value::from(is_empty))
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

pub fn trim(value: Option<&Value>) -> Result<Value, RuntimeError> {
    rtrim(value)
}

pub fn ltrim(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::ltrim_argument_error(None));
    };
    let Value::String(text) = value else {
        return Err(RuntimeError::ltrim_argument_error(Some(value.kind())));
    };

    Ok(Value::from(text.trim_start_matches(char::is_whitespace)))
}

pub fn rtrim(value: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(value) = value else {
        return Err(RuntimeError::trim_argument_error(None));
    };
    let Value::String(text) = value else {
        return Err(RuntimeError::trim_argument_error(Some(value.kind())));
    };

    Ok(Value::from(text.trim_end_matches(' ')))
}

pub fn at(needle: Option<&Value>, haystack: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(needle) = needle else {
        return Err(RuntimeError::at_argument_error(None));
    };
    let Value::String(needle) = needle else {
        return Err(RuntimeError::at_argument_error(Some(needle.kind())));
    };

    let Some(haystack) = haystack else {
        return Err(RuntimeError::at_argument_error(None));
    };
    let Value::String(haystack) = haystack else {
        return Err(RuntimeError::at_argument_error(Some(haystack.kind())));
    };

    if needle.is_empty() || haystack.is_empty() {
        return Ok(Value::from(0_i64));
    }

    let Some(byte_index) = haystack.find(needle) else {
        return Ok(Value::from(0_i64));
    };

    let position = haystack[..byte_index].chars().count() as i64 + 1;
    Ok(Value::from(position))
}

pub fn replicate(source: Option<&Value>, count: Option<&Value>) -> Result<Value, RuntimeError> {
    let Some(source) = source else {
        return Err(RuntimeError::replicate_argument_error(None));
    };
    let Value::String(source) = source else {
        return Err(RuntimeError::replicate_argument_error(Some(source.kind())));
    };

    let count = numeric_string_count(
        count,
        RuntimeError::replicate_argument_error,
        RuntimeError::replicate_overflow_error,
        source.len(),
    )?;
    if count == 0 || source.is_empty() {
        return Ok(Value::from(""));
    }

    Ok(Value::from(source.repeat(count)))
}

pub fn space(count: Option<&Value>) -> Result<Value, RuntimeError> {
    let count = numeric_string_count(
        count,
        RuntimeError::space_argument_error,
        RuntimeError::space_overflow_error,
        1,
    )?;
    if count == 0 {
        return Ok(Value::from(""));
    }

    Ok(Value::from(" ".repeat(count)))
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

pub fn adel(array: &mut Value, position: Option<&Value>) -> Result<Value, RuntimeError> {
    if !matches!(array, Value::Array(_)) {
        return Ok(Value::Nil);
    }

    let Some(position) = position else {
        return array.array_clone();
    };

    let position = match position {
        Value::Integer(value) if *value == 0 => 1,
        Value::Integer(value) if *value > 0 => *value as usize,
        _ => return array.array_clone(),
    };

    array.array_delete(position.saturating_sub(1))?;
    array.array_clone()
}

pub fn ains(array: &mut Value, position: Option<&Value>) -> Result<Value, RuntimeError> {
    if !matches!(array, Value::Array(_)) {
        return Ok(Value::Nil);
    }

    let Some(position) = position else {
        return array.array_clone();
    };

    let position = match position {
        Value::Integer(value) if *value == 0 => 1,
        Value::Integer(value) if *value > 0 => *value as usize,
        _ => return array.array_clone(),
    };

    array.array_insert(position.saturating_sub(1))?;
    array.array_clone()
}

pub fn ascan(
    array: Option<&Value>,
    search: Option<&Value>,
    start: Option<&Value>,
    count: Option<&Value>,
) -> Result<Value, RuntimeError> {
    let (Some(array), Some(search)) = (array, search) else {
        return Ok(Value::from(0_i64));
    };
    let Value::Array(values) = array else {
        return Ok(Value::from(0_i64));
    };
    if values.is_empty() {
        return Ok(Value::from(0_i64));
    }

    let start_index = match start {
        Some(Value::Integer(value)) if *value > 0 => (*value as usize).saturating_sub(1),
        Some(Value::Integer(_)) => 0,
        _ => 0,
    };
    if start_index >= values.len() {
        return Ok(Value::from(0_i64));
    }

    let remaining = values.len() - start_index;
    let max_count = match count {
        Some(Value::Integer(value)) if *value > 0 => remaining.min(*value as usize),
        Some(Value::Integer(_)) => 0,
        _ => remaining,
    };

    for (offset, candidate) in values.iter().skip(start_index).take(max_count).enumerate() {
        if array_scan_matches(candidate, search) {
            return Ok(Value::from((start_index + offset + 1) as i64));
        }
    }

    Ok(Value::from(0_i64))
}

pub fn call_builtin(
    name: &str,
    arguments: &[Value],
    context: &mut RuntimeContext,
) -> Result<Value, RuntimeError> {
    match Builtin::lookup(name) {
        Some(Builtin::QOut) => qout(arguments, context.output_mut()),
        Some(Builtin::Eval) => {
            let rest = arguments.get(1..).unwrap_or(&[]);
            eval(arguments.first(), rest, context)
        }
        Some(Builtin::Abs) => abs(arguments.first()),
        Some(Builtin::Sqrt) => sqrt_value(arguments.first()),
        Some(Builtin::Sin) => sin_value(arguments.first()),
        Some(Builtin::Cos) => cos_value(arguments.first()),
        Some(Builtin::Tan) => tan_value(arguments.first()),
        Some(Builtin::Exp) => exp_value(arguments.first()),
        Some(Builtin::Log) => log_value(arguments.first()),
        Some(Builtin::Int) => int(arguments.first()),
        Some(Builtin::Round) => round_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Mod) => mod_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Max) => max_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Min) => min_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Len) => len(arguments.first()),
        Some(Builtin::Str) => str_value(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Val) => val(arguments.first()),
        Some(Builtin::ValType) => valtype(arguments.first()),
        Some(Builtin::Type) => type_value(arguments.first()),
        Some(Builtin::Empty) => empty(arguments.first()),
        Some(Builtin::SubStr) => substr(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Left) => left(arguments.first(), arguments.get(1)),
        Some(Builtin::Right) => right(arguments.first(), arguments.get(1)),
        Some(Builtin::Upper) => upper(arguments.first()),
        Some(Builtin::Lower) => lower(arguments.first()),
        Some(Builtin::Trim) => trim(arguments.first()),
        Some(Builtin::LTrim) => ltrim(arguments.first()),
        Some(Builtin::RTrim) => rtrim(arguments.first()),
        Some(Builtin::At) => at(arguments.first(), arguments.get(1)),
        Some(Builtin::Replicate) => replicate(arguments.first(), arguments.get(1)),
        Some(Builtin::Space) => space(arguments.first()),
        Some(Builtin::AClone) => aclone(arguments.first()),
        Some(Builtin::AScan) => ascan(
            arguments.first(),
            arguments.get(1),
            arguments.get(2),
            arguments.get(3),
        ),
        Some(Builtin::AAdd | Builtin::ASize | Builtin::ADel | Builtin::AIns) => {
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
        Some(Builtin::Eval) => {
            let Some(codeblock) = arguments.first() else {
                return Err(RuntimeError::eval_argument_error(None));
            };
            eval(Some(codeblock), &arguments[1..], context)
        }
        Some(Builtin::Abs) => abs(arguments.first()),
        Some(Builtin::Sqrt) => sqrt_value(arguments.first()),
        Some(Builtin::Sin) => sin_value(arguments.first()),
        Some(Builtin::Cos) => cos_value(arguments.first()),
        Some(Builtin::Tan) => tan_value(arguments.first()),
        Some(Builtin::Exp) => exp_value(arguments.first()),
        Some(Builtin::Log) => log_value(arguments.first()),
        Some(Builtin::Int) => int(arguments.first()),
        Some(Builtin::Round) => round_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Mod) => mod_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Max) => max_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Min) => min_value(arguments.first(), arguments.get(1)),
        Some(Builtin::Len) => len(arguments.first()),
        Some(Builtin::Str) => str_value(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Val) => val(arguments.first()),
        Some(Builtin::ValType) => valtype(arguments.first()),
        Some(Builtin::Type) => type_value(arguments.first()),
        Some(Builtin::Empty) => empty(arguments.first()),
        Some(Builtin::SubStr) => substr(arguments.first(), arguments.get(1), arguments.get(2)),
        Some(Builtin::Left) => left(arguments.first(), arguments.get(1)),
        Some(Builtin::Right) => right(arguments.first(), arguments.get(1)),
        Some(Builtin::Upper) => upper(arguments.first()),
        Some(Builtin::Lower) => lower(arguments.first()),
        Some(Builtin::Trim) => trim(arguments.first()),
        Some(Builtin::LTrim) => ltrim(arguments.first()),
        Some(Builtin::RTrim) => rtrim(arguments.first()),
        Some(Builtin::At) => at(arguments.first(), arguments.get(1)),
        Some(Builtin::Replicate) => replicate(arguments.first(), arguments.get(1)),
        Some(Builtin::Space) => space(arguments.first()),
        Some(Builtin::AClone) => aclone(arguments.first()),
        Some(Builtin::AScan) => ascan(
            arguments.first(),
            arguments.get(1),
            arguments.get(2),
            arguments.get(3),
        ),
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
        Some(Builtin::ADel) => {
            let Some((array, rest)) = arguments.split_first_mut() else {
                return Ok(Value::Nil);
            };
            adel(array, rest.first())
        }
        Some(Builtin::AIns) => {
            let Some((array, rest)) = arguments.split_first_mut() else {
                return Ok(Value::Nil);
            };
            ains(array, rest.first())
        }
        None => Err(RuntimeError::unknown_builtin(name)),
    }
}

fn array_scan_matches(candidate: &Value, search: &Value) -> bool {
    match (candidate, search) {
        (Value::Nil, Value::Nil) => true,
        (Value::Logical(left), Value::Logical(right)) => left == right,
        (Value::Integer(left), Value::Integer(right)) => left == right,
        (Value::Integer(left), Value::Float(right)) => (*left as f64) == *right,
        (Value::Float(left), Value::Integer(right)) => *left == (*right as f64),
        (Value::Float(left), Value::Float(right)) => left == right,
        (Value::String(left), Value::String(right)) => string_equals_exact_off(left, right),
        _ => false,
    }
}

fn string_equals_exact_off(left: &str, right: &str) -> bool {
    left.starts_with(right)
}

fn normalize_name(name: &str) -> String {
    name.to_ascii_uppercase()
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

impl From<CodeblockValue> for Value {
    fn from(value: CodeblockValue) -> Self {
        Self::Codeblock(value)
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

    pub fn eval_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1004 Argument error (EVAL)".to_owned(),
            expected: Some(ValueKind::Codeblock),
            actual,
        }
    }

    pub fn len_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1111 Argument error (LEN)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn abs_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1089 Argument error (ABS)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn sqrt_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1097 Argument error (SQRT)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn sin_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1091 Argument error (SIN)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn cos_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1091 Argument error (COS)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn tan_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1091 Argument error (TAN)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn exp_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1096 Argument error (EXP)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn log_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1095 Argument error (LOG)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn int_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1090 Argument error (INT)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn round_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1094 Argument error (ROUND)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn mod_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1085 Argument error (%)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn mod_zero_divisor() -> Self {
        Self {
            message: "BASE 1341 Zero divisor (%)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn max_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1093 Argument error (MAX)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn min_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1092 Argument error (MIN)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn str_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1099 Argument error (STR)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn val_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1098 Argument error (VAL)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn type_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1121 Argument error (TYPE)".to_owned(),
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

    pub fn trim_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1100 Argument error (TRIM)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn ltrim_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1101 Argument error (LTRIM)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn at_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1108 Argument error (AT)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn replicate_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1106 Argument error (REPLICATE)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn replicate_overflow_error() -> Self {
        Self {
            message: "BASE 1234 String overflow (REPLICATE)".to_owned(),
            expected: None,
            actual: None,
        }
    }

    pub fn space_argument_error(actual: Option<ValueKind>) -> Self {
        Self {
            message: "BASE 1105 Argument error (SPACE)".to_owned(),
            expected: None,
            actual,
        }
    }

    pub fn space_overflow_error() -> Self {
        Self {
            message: "BASE 1234 String overflow (SPACE)".to_owned(),
            expected: None,
            actual: None,
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
enum ExtremumKind {
    Max,
    Min,
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

fn extremum_value(
    left: Option<&Value>,
    right: Option<&Value>,
    kind: ExtremumKind,
) -> Result<Value, RuntimeError> {
    let Some(left) = left else {
        return Err(extremum_argument_error(kind, None));
    };
    let Some(right) = right else {
        return Err(extremum_argument_error(kind, None));
    };

    let ordering = compare_extremum_values(left, right).ok_or_else(|| {
        extremum_argument_error(kind, Some(unsupported_extremum_kind(left, right)))
    })?;

    let selected = match kind {
        ExtremumKind::Max => {
            if ordering == Ordering::Less {
                right
            } else {
                left
            }
        }
        ExtremumKind::Min => {
            if ordering == Ordering::Greater {
                right
            } else {
                left
            }
        }
    };

    Ok(selected.clone())
}

fn compare_extremum_values(left: &Value, right: &Value) -> Option<Ordering> {
    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => Some(left.cmp(right)),
        (Value::Integer(left), Value::Float(right)) => (*left as f64).partial_cmp(right),
        (Value::Float(left), Value::Integer(right)) => left.partial_cmp(&(*right as f64)),
        (Value::Float(left), Value::Float(right)) => left.partial_cmp(right),
        (Value::Logical(left), Value::Logical(right)) => Some(left.cmp(right)),
        _ => None,
    }
}

fn extremum_argument_error(kind: ExtremumKind, actual: Option<ValueKind>) -> RuntimeError {
    match kind {
        ExtremumKind::Max => RuntimeError::max_argument_error(actual),
        ExtremumKind::Min => RuntimeError::min_argument_error(actual),
    }
}

fn unsupported_extremum_kind(left: &Value, right: &Value) -> ValueKind {
    if is_extremum_supported_kind(left.kind()) {
        right.kind()
    } else {
        left.kind()
    }
}

fn is_extremum_supported_kind(kind: ValueKind) -> bool {
    matches!(
        kind,
        ValueKind::Integer | ValueKind::Float | ValueKind::Logical
    )
}

fn harbour_string_is_empty(text: &str) -> bool {
    text.as_bytes().iter().all(u8::is_ascii_whitespace)
}

fn type_from_source_text(text: &str) -> &'static str {
    let text = text.trim_matches(|ch: char| ch.is_ascii_whitespace());
    if text.is_empty() {
        return "U";
    }

    if text.eq_ignore_ascii_case("NIL") {
        return "U";
    }

    if text.eq_ignore_ascii_case(".T.") || text.eq_ignore_ascii_case(".F.") {
        return "L";
    }

    if is_type_numeric_text(text) {
        return "N";
    }

    if is_type_quoted_text(text) {
        return "C";
    }

    if is_type_array_literal_text(text) {
        return "A";
    }

    "U"
}

fn is_type_numeric_text(text: &str) -> bool {
    let text = text
        .strip_prefix('+')
        .or_else(|| text.strip_prefix('-'))
        .unwrap_or(text);
    if text.is_empty() {
        return false;
    }

    if let Some(fraction) = text.strip_prefix('.') {
        return !fraction.is_empty() && fraction.chars().all(|ch| ch.is_ascii_digit());
    }

    let mut parts = text.split('.');
    let integer = parts.next().unwrap_or_default();
    if integer.is_empty() || !integer.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }

    match (parts.next(), parts.next()) {
        (None, None) => true,
        (Some(fraction), None) => {
            !fraction.is_empty() && fraction.chars().all(|ch| ch.is_ascii_digit())
        }
        _ => false,
    }
}

fn is_type_quoted_text(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    matches!(first, '"' | '\'') && text.len() >= 2 && text.ends_with(first)
}

fn is_type_array_literal_text(text: &str) -> bool {
    text.starts_with('{') && text.ends_with('}')
}

#[derive(Clone, Copy)]
enum StrNumeric {
    Integer(i64),
    Float(f64),
}

fn format_str_default(number: StrNumeric) -> String {
    match number {
        StrNumeric::Integer(value) => value.to_string(),
        StrNumeric::Float(value) => trim_default_float(format!("{value:.15}")),
    }
}

fn format_str_rounded(number: StrNumeric) -> String {
    match number {
        StrNumeric::Integer(value) => value.to_string(),
        StrNumeric::Float(value) => format!("{:.0}", value),
    }
}

fn format_str_fixed(number: StrNumeric, decimals: usize) -> String {
    match number {
        StrNumeric::Integer(value) => format!("{:.*}", decimals, value as f64),
        StrNumeric::Float(value) => format!("{:.*}", decimals, value),
    }
}

fn format_non_finite_str(width: Option<i64>) -> String {
    match width {
        None => "*".repeat(23),
        Some(width) if width <= 0 => "*".repeat(10),
        Some(width) => "*".repeat(width as usize),
    }
}

fn trim_default_float(mut text: String) -> String {
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.push('0');
    }
    text
}

fn parse_val_string(text: &str) -> Value {
    let mut chars = text.chars().peekable();
    while chars.next_if(|ch| ch.is_ascii_whitespace()).is_some() {}

    let sign = match chars.peek().copied() {
        Some('-') => {
            chars.next();
            -1.0_f64
        }
        Some('+') => {
            chars.next();
            1.0_f64
        }
        _ => 1.0_f64,
    };

    let mut integer_part = String::new();
    while let Some(ch) = chars.next_if(|ch| ch.is_ascii_digit()) {
        integer_part.push(ch);
    }

    let mut fractional_part = String::new();
    let mut saw_fraction_marker = false;
    let mut degraded_fraction = false;
    if matches!(chars.peek(), Some('.')) {
        saw_fraction_marker = true;
        chars.next();
        while let Some(ch) = chars.peek().copied() {
            if ch.is_ascii_digit() {
                fractional_part.push(if degraded_fraction { '0' } else { ch });
                chars.next();
            } else if ch == '.' {
                degraded_fraction = true;
                fractional_part.push('0');
                chars.next();
            } else {
                break;
            }
        }
    }

    if integer_part.is_empty() && !saw_fraction_marker {
        return Value::from(0_i64);
    }

    if !fractional_part.is_empty() {
        let mut numeric = String::new();
        if integer_part.is_empty() {
            numeric.push('0');
        } else {
            numeric.push_str(&integer_part);
        }
        numeric.push('.');
        numeric.push_str(&fractional_part);

        let parsed = numeric.parse::<f64>().unwrap_or(0.0) * sign;
        if parsed == 0.0 {
            return Value::from(0.0_f64);
        }
        return Value::from(parsed);
    }

    let parsed = integer_part.parse::<i64>().unwrap_or(0);
    if sign.is_sign_negative() {
        Value::from(-parsed)
    } else {
        Value::from(parsed)
    }
}

fn apply_str_width(formatted: String, width: Option<i64>) -> String {
    let width = match width {
        None => 10usize,
        Some(width) if width <= 0 => 10usize,
        Some(width) => width as usize,
    };

    if formatted.len() > width {
        "*".repeat(width)
    } else if formatted.len() >= width {
        formatted
    } else {
        format!("{formatted:>width$}")
    }
}

fn numeric_optional_i64(
    value: Option<&Value>,
    argument_error: fn(Option<ValueKind>) -> RuntimeError,
) -> Result<Option<i64>, RuntimeError> {
    let Some(value) = value else {
        return Ok(None);
    };

    match value {
        Value::Integer(value) => Ok(Some(*value)),
        Value::Float(value) => Ok(Some(value.trunc() as i64)),
        other => Err(argument_error(Some(other.kind()))),
    }
}

fn numeric_required_i64(
    value: Option<&Value>,
    argument_error: fn(Option<ValueKind>) -> RuntimeError,
) -> Result<i64, RuntimeError> {
    let Some(value) = value else {
        return Err(argument_error(None));
    };

    match value {
        Value::Integer(value) => Ok(*value),
        Value::Float(value) => Ok(value.trunc() as i64),
        other => Err(argument_error(Some(other.kind()))),
    }
}

fn numeric_string_count(
    value: Option<&Value>,
    argument_error: fn(Option<ValueKind>) -> RuntimeError,
    overflow_error: fn() -> RuntimeError,
    unit_len: usize,
) -> Result<usize, RuntimeError> {
    const XBASE_MAX_STRING_LEN: usize = 65_535;

    let Some(value) = value else {
        return Err(argument_error(None));
    };

    let count = match value {
        Value::Integer(value) => *value,
        Value::Float(value) => value.trunc() as i64,
        other => return Err(argument_error(Some(other.kind()))),
    };

    if count <= 0 {
        return Ok(0);
    }

    let count = count as usize;
    let Some(total_len) = unit_len.checked_mul(count) else {
        return Err(overflow_error());
    };
    if total_len > XBASE_MAX_STRING_LEN {
        return Err(overflow_error());
    }

    Ok(count)
}

fn round_with_decimals(value: f64, decimals: i64) -> f64 {
    if decimals >= 0 {
        let factor = 10_f64.powi(decimals as i32);
        (value * factor).round() / factor
    } else {
        let factor = 10_f64.powi((-decimals) as i32);
        (value / factor).round() * factor
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        OutputBuffer, RuntimeContext, RuntimeError, Value, ValueKind, aadd, abs, aclone, asize, at,
        call_builtin, call_builtin_mut, cos_value, exp_value, int, len, log_value, max_value,
        min_value, mod_value, qout, replicate, round_value, sin_value, space, sqrt_value,
        str_value, tan_value, type_value, val,
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
            Value::from("12345").equals(&Value::from("123")),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from("123").equals(&Value::from("12345")),
            Ok(Value::from(false))
        );
        assert_eq!(
            Value::from("123").equals(&Value::from("")),
            Ok(Value::from(true))
        );
        assert_eq!(
            Value::from("AA").exact_equals(&Value::from("A")),
            Ok(Value::from(false))
        );
        assert_eq!(
            Value::from("AA").not_equals(&Value::from("A")),
            Ok(Value::from(false))
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
    fn abs_follows_the_current_numeric_runtime_baseline() {
        assert_eq!(abs(Some(&Value::from(0_i64))), Ok(Value::from(0_i64)));
        assert_eq!(abs(Some(&Value::from(10_i64))), Ok(Value::from(10_i64)));
        assert_eq!(abs(Some(&Value::from(-10_i64))), Ok(Value::from(10_i64)));
        assert_eq!(abs(Some(&Value::from(0.1_f64))), Ok(Value::from(0.1_f64)));
        assert_eq!(
            abs(Some(&Value::from(-10.7_f64))),
            Ok(Value::from(10.7_f64))
        );
    }

    #[test]
    fn abs_reports_xbase_style_argument_errors() {
        assert_eq!(
            abs(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1089 Argument error (ABS)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            abs(None),
            Err(RuntimeError {
                message: "BASE 1089 Argument error (ABS)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn abs_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("ABS", &[Value::from(-10_i64)], &mut context),
            Ok(Value::from(10_i64))
        );

        let mut mutable_arguments = [Value::from(-150.245_f64)];
        assert_eq!(
            call_builtin_mut("abs", &mut mutable_arguments, &mut context),
            Ok(Value::from(150.245_f64))
        );
        assert_eq!(mutable_arguments[0], Value::from(-150.245_f64));
    }

    #[test]
    fn sqrt_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            sqrt_value(Some(&Value::from(-1_i64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            sqrt_value(Some(&Value::from(0_i64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            sqrt_value(Some(&Value::from(4_i64))),
            Ok(Value::from(2.0_f64))
        );
        assert_eq!(
            sqrt_value(Some(&Value::from(10_i64))),
            Ok(Value::from(10_f64.sqrt()))
        );
        assert_eq!(
            sqrt_value(Some(&Value::from(3.0_f64))),
            Ok(Value::from(3.0_f64.sqrt()))
        );
    }

    #[test]
    fn sqrt_reports_xbase_style_argument_errors() {
        assert_eq!(
            sqrt_value(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1097 Argument error (SQRT)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            sqrt_value(None),
            Err(RuntimeError {
                message: "BASE 1097 Argument error (SQRT)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn sqrt_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("SQRT", &[Value::from(4_i64)], &mut context),
            Ok(Value::from(2.0_f64))
        );

        let mut mutable_arguments = [Value::from(10_i64)];
        assert_eq!(
            call_builtin_mut("sqrt", &mut mutable_arguments, &mut context),
            Ok(Value::from(10_f64.sqrt()))
        );
        assert_eq!(mutable_arguments[0], Value::from(10_i64));
    }

    #[test]
    fn sin_and_cos_match_the_current_numeric_runtime_baseline() {
        assert_eq!(
            sin_value(Some(&Value::from(0_i64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            cos_value(Some(&Value::from(0_i64))),
            Ok(Value::from(1.0_f64))
        );
        assert_eq!(
            round_value(
                sin_value(Some(&Value::from(1_i64))).ok().as_ref(),
                Some(&Value::from(2_i64))
            ),
            Ok(Value::from(0.84_f64))
        );
        assert_eq!(
            round_value(
                cos_value(Some(&Value::from(1_i64))).ok().as_ref(),
                Some(&Value::from(2_i64))
            ),
            Ok(Value::from(0.54_f64))
        );
    }

    #[test]
    fn sin_and_cos_report_xbase_style_argument_errors() {
        assert_eq!(
            sin_value(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1091 Argument error (SIN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            cos_value(None),
            Err(RuntimeError {
                message: "BASE 1091 Argument error (COS)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn sin_and_cos_dispatch_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("SIN", &[Value::from(0_i64)], &mut context),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            call_builtin("COS", &[Value::from(0_i64)], &mut context),
            Ok(Value::from(1.0_f64))
        );

        let mut mutable_arguments = [Value::from(1_i64)];
        assert_eq!(
            call_builtin_mut("sin", &mut mutable_arguments, &mut context),
            Ok(Value::from(1_f64.sin()))
        );
        assert_eq!(mutable_arguments[0], Value::from(1_i64));
    }

    #[test]
    fn tan_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            tan_value(Some(&Value::from(0_i64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            round_value(
                tan_value(Some(&Value::from(1_i64))).ok().as_ref(),
                Some(&Value::from(4_i64))
            ),
            Ok(Value::from(1.5574_f64))
        );
    }

    #[test]
    fn tan_reports_xbase_style_argument_errors() {
        assert_eq!(
            tan_value(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1091 Argument error (TAN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            tan_value(None),
            Err(RuntimeError {
                message: "BASE 1091 Argument error (TAN)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn tan_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("TAN", &[Value::from(0_i64)], &mut context),
            Ok(Value::from(0.0_f64))
        );

        let mut mutable_arguments = [Value::from(1_i64)];
        assert_eq!(
            call_builtin_mut("tan", &mut mutable_arguments, &mut context),
            Ok(Value::from(1_f64.tan()))
        );
        assert_eq!(mutable_arguments[0], Value::from(1_i64));
    }

    #[test]
    fn exp_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            exp_value(Some(&Value::from(0_i64))),
            Ok(Value::from(1.0_f64))
        );
        assert_eq!(
            exp_value(Some(&Value::from(1_i64))),
            Ok(Value::from(1_f64.exp()))
        );
        assert_eq!(
            exp_value(Some(&Value::from(15_i64))),
            Ok(Value::from(15_f64.exp()))
        );
        assert_eq!(
            round_value(
                exp_value(Some(&Value::from(1_i64))).ok().as_ref(),
                Some(&Value::from(2_i64))
            ),
            Ok(Value::from(2.72_f64))
        );
        assert_eq!(
            str_value(
                exp_value(Some(&Value::from(15_i64))).ok().as_ref(),
                None,
                None
            ),
            Ok(Value::from("3269017.37247211067006"))
        );
    }

    #[test]
    fn exp_reports_xbase_style_argument_errors() {
        assert_eq!(
            exp_value(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1096 Argument error (EXP)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            exp_value(None),
            Err(RuntimeError {
                message: "BASE 1096 Argument error (EXP)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn exp_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("EXP", &[Value::from(0_i64)], &mut context),
            Ok(Value::from(1.0_f64))
        );

        let mut mutable_arguments = [Value::from(1_i64)];
        assert_eq!(
            call_builtin_mut("exp", &mut mutable_arguments, &mut context),
            Ok(Value::from(1_f64.exp()))
        );
        assert_eq!(mutable_arguments[0], Value::from(1_i64));
    }

    #[test]
    fn log_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            log_value(Some(&Value::from(-1_i64))),
            Ok(Value::from(f64::NEG_INFINITY))
        );
        assert_eq!(
            log_value(Some(&Value::from(1_i64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            log_value(Some(&Value::from(12_i64))),
            Ok(Value::from(12_f64.ln()))
        );
        assert_eq!(
            log_value(Some(&Value::from(10.0_f64))),
            Ok(Value::from(10_f64.ln()))
        );
        assert_eq!(
            str_value(
                log_value(Some(&Value::from(-1_i64))).ok().as_ref(),
                None,
                None
            ),
            Ok(Value::from("***********************"))
        );
    }

    #[test]
    fn log_reports_xbase_style_argument_errors() {
        assert_eq!(
            log_value(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1095 Argument error (LOG)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            log_value(None),
            Err(RuntimeError {
                message: "BASE 1095 Argument error (LOG)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn log_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("LOG", &[Value::from(1_i64)], &mut context),
            Ok(Value::from(0.0_f64))
        );

        let mut mutable_arguments = [Value::from(10_i64)];
        assert_eq!(
            call_builtin_mut("log", &mut mutable_arguments, &mut context),
            Ok(Value::from(10_f64.ln()))
        );
        assert_eq!(mutable_arguments[0], Value::from(10_i64));
    }

    #[test]
    fn int_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(int(Some(&Value::from(0_i64))), Ok(Value::from(0_i64)));
        assert_eq!(int(Some(&Value::from(10_i64))), Ok(Value::from(10_i64)));
        assert_eq!(int(Some(&Value::from(-10_i64))), Ok(Value::from(-10_i64)));
        assert_eq!(int(Some(&Value::from(10.5_f64))), Ok(Value::from(10_i64)));
        assert_eq!(int(Some(&Value::from(-10.5_f64))), Ok(Value::from(-10_i64)));
    }

    #[test]
    fn int_reports_xbase_style_argument_errors() {
        assert_eq!(
            int(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1090 Argument error (INT)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            int(None),
            Err(RuntimeError {
                message: "BASE 1090 Argument error (INT)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn int_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("INT", &[Value::from(10.7_f64)], &mut context),
            Ok(Value::from(10_i64))
        );

        let mut mutable_arguments = [Value::from(-150.245_f64)];
        assert_eq!(
            call_builtin_mut("int", &mut mutable_arguments, &mut context),
            Ok(Value::from(-150_i64))
        );
        assert_eq!(mutable_arguments[0], Value::from(-150.245_f64));
    }

    #[test]
    fn round_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            round_value(Some(&Value::from(0.5_f64)), Some(&Value::from(0_i64))),
            Ok(Value::from(1_i64))
        );
        assert_eq!(
            round_value(Some(&Value::from(0.55_f64)), Some(&Value::from(1_i64))),
            Ok(Value::from(0.6_f64))
        );
        assert_eq!(
            round_value(Some(&Value::from(0.557_f64)), Some(&Value::from(2_i64))),
            Ok(Value::from(0.56_f64))
        );
        assert_eq!(
            round_value(Some(&Value::from(50_i64)), Some(&Value::from(-2_i64))),
            Ok(Value::from(100_i64))
        );
        assert_eq!(
            round_value(Some(&Value::from(-0.55_f64)), Some(&Value::from(1_i64))),
            Ok(Value::from(-0.6_f64))
        );
    }

    #[test]
    fn round_reports_xbase_style_argument_errors() {
        assert_eq!(
            round_value(Some(&Value::Nil), Some(&Value::from(0_i64))),
            Err(RuntimeError {
                message: "BASE 1094 Argument error (ROUND)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            round_value(Some(&Value::from(0_i64)), Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1094 Argument error (ROUND)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
    }

    #[test]
    fn round_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "ROUND",
                &[Value::from(0.557_f64), Value::from(2_i64)],
                &mut context
            ),
            Ok(Value::from(0.56_f64))
        );

        let mut mutable_arguments = [Value::from(-0.55_f64), Value::from(1_i64)];
        assert_eq!(
            call_builtin_mut("round", &mut mutable_arguments, &mut context),
            Ok(Value::from(-0.6_f64))
        );
        assert_eq!(mutable_arguments[0], Value::from(-0.55_f64));
    }

    #[test]
    fn mod_matches_the_current_numeric_runtime_baseline() {
        assert_eq!(
            mod_value(Some(&Value::from(100_i64)), Some(&Value::from(60_i64))),
            Ok(Value::from(40.0_f64))
        );
        assert_eq!(
            mod_value(Some(&Value::from(2_i64)), Some(&Value::from(4_i64))),
            Ok(Value::from(2.0_f64))
        );
        assert_eq!(
            mod_value(Some(&Value::from(4_i64)), Some(&Value::from(2.0_f64))),
            Ok(Value::from(0.0_f64))
        );
        assert_eq!(
            mod_value(Some(&Value::from(-1_i64)), Some(&Value::from(3_i64))),
            Ok(Value::from(2.0_f64))
        );
        assert_eq!(
            mod_value(Some(&Value::from(1_i64)), Some(&Value::from(-3_i64))),
            Ok(Value::from(-2.0_f64))
        );
    }

    #[test]
    fn mod_reports_xbase_style_argument_and_zero_divisor_errors() {
        assert_eq!(
            mod_value(Some(&Value::Nil), Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1085 Argument error (%)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            mod_value(Some(&Value::from(1_i64)), Some(&Value::from(0_i64))),
            Err(RuntimeError {
                message: "BASE 1341 Zero divisor (%)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn mod_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "MOD",
                &[
                    Value::from(100_i64),
                    Value::from(60_i64),
                    Value::from("ignored"),
                ],
                &mut context
            ),
            Ok(Value::from(40.0_f64))
        );

        let mut mutable_arguments = [Value::from(-2_i64), Value::from(-3_i64)];
        assert_eq!(
            call_builtin_mut("mod", &mut mutable_arguments, &mut context),
            Ok(Value::from(-2.0_f64))
        );
        assert_eq!(mutable_arguments[0], Value::from(-2_i64));
    }

    #[test]
    fn max_and_min_match_the_current_runtime_baseline() {
        assert_eq!(
            max_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64))),
            Ok(Value::from(10_i64))
        );
        assert_eq!(
            max_value(Some(&Value::from(10_i64)), Some(&Value::from(10.5_f64))),
            Ok(Value::from(10.5_f64))
        );
        assert_eq!(
            max_value(Some(&Value::from(false)), Some(&Value::from(true))),
            Ok(Value::from(true))
        );
        assert_eq!(
            max_value(Some(&Value::from(10_i64)), Some(&Value::from(10.0_f64))),
            Ok(Value::from(10_i64))
        );
        assert_eq!(
            min_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64))),
            Ok(Value::from(5_i64))
        );
        assert_eq!(
            min_value(Some(&Value::from(10_i64)), Some(&Value::from(10.5_f64))),
            Ok(Value::from(10_i64))
        );
        assert_eq!(
            min_value(Some(&Value::from(false)), Some(&Value::from(true))),
            Ok(Value::from(false))
        );
        assert_eq!(
            min_value(Some(&Value::from(10.0_f64)), Some(&Value::from(10_i64))),
            Ok(Value::from(10.0_f64))
        );
    }

    #[test]
    fn max_and_min_report_xbase_style_argument_errors() {
        assert_eq!(
            max_value(Some(&Value::Nil), Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1093 Argument error (MAX)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            min_value(Some(&Value::from(10_i64)), Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1092 Argument error (MIN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            max_value(None, Some(&Value::from(10_i64))),
            Err(RuntimeError {
                message: "BASE 1093 Argument error (MAX)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            min_value(Some(&Value::from("A")), Some(&Value::from(1_i64))),
            Err(RuntimeError {
                message: "BASE 1092 Argument error (MIN)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
    }

    #[test]
    fn max_and_min_dispatch_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "MAX",
                &[Value::from(10_i64), Value::from(100_i64)],
                &mut context
            ),
            Ok(Value::from(100_i64))
        );
        assert_eq!(
            call_builtin(
                "MIN",
                &[Value::from(true), Value::from(false)],
                &mut context
            ),
            Ok(Value::from(false))
        );

        let mut mutable_arguments = [Value::from(2.5_f64), Value::from(2_i64)];
        assert_eq!(
            call_builtin_mut("max", &mut mutable_arguments, &mut context),
            Ok(Value::from(2.5_f64))
        );
        assert_eq!(
            call_builtin_mut("min", &mut mutable_arguments, &mut context),
            Ok(Value::from(2_i64))
        );
        assert_eq!(mutable_arguments[0], Value::from(2.5_f64));
        assert_eq!(mutable_arguments[1], Value::from(2_i64));
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
    fn str_follows_the_current_numeric_runtime_baseline() {
        assert_eq!(
            str_value(Some(&Value::from(10_i64)), None, None),
            Ok(Value::from("        10"))
        );
        assert_eq!(
            str_value(Some(&Value::from(0_i64)), None, None),
            Ok(Value::from("         0"))
        );
        assert_eq!(
            str_value(Some(&Value::from(10.5_f64)), None, None),
            Ok(Value::from("      10.5"))
        );
        assert_eq!(
            str_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64)), None),
            Ok(Value::from("   10"))
        );
        assert_eq!(
            str_value(
                Some(&Value::from(10.6_f64)),
                Some(&Value::from(5_i64)),
                None
            ),
            Ok(Value::from("   11"))
        );
        assert_eq!(
            str_value(
                Some(&Value::from(2_i64)),
                Some(&Value::from(5_i64)),
                Some(&Value::from(2_i64)),
            ),
            Ok(Value::from(" 2.00"))
        );
        assert_eq!(
            str_value(
                Some(&Value::from(3.125_f64)),
                Some(&Value::from(8_i64)),
                Some(&Value::from(2_i64)),
            ),
            Ok(Value::from("    3.12"))
        );
        assert_eq!(
            str_value(
                Some(&Value::from(100000_i64)),
                Some(&Value::from(5_i64)),
                None
            ),
            Ok(Value::from("*****"))
        );
    }

    #[test]
    fn str_reports_xbase_style_argument_errors() {
        assert_eq!(
            str_value(Some(&Value::Nil), None, None),
            Err(RuntimeError {
                message: "BASE 1099 Argument error (STR)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            str_value(
                Some(&Value::from("A")),
                Some(&Value::from(10_i64)),
                Some(&Value::from(2_i64)),
            ),
            Err(RuntimeError {
                message: "BASE 1099 Argument error (STR)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            str_value(
                Some(&Value::from(100_i64)),
                Some(&Value::from(10_i64)),
                Some(&Value::from("A")),
            ),
            Err(RuntimeError {
                message: "BASE 1099 Argument error (STR)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
    }

    #[test]
    fn str_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("STR", &[Value::from(10_i64)], &mut context),
            Ok(Value::from("        10"))
        );

        let mut mutable_arguments = [Value::from(2_i64), Value::from(5_i64), Value::from(2_i64)];
        assert_eq!(
            call_builtin_mut("str", &mut mutable_arguments, &mut context),
            Ok(Value::from(" 2.00"))
        );
        assert_eq!(mutable_arguments[0], Value::from(2_i64));
    }

    #[test]
    fn val_matches_the_current_string_to_numeric_runtime_baseline() {
        assert_eq!(val(Some(&Value::from(""))), Ok(Value::from(0_i64)));
        assert_eq!(val(Some(&Value::from("A"))), Ok(Value::from(0_i64)));
        assert_eq!(val(Some(&Value::from("10"))), Ok(Value::from(10_i64)));
        assert_eq!(val(Some(&Value::from("  -12"))), Ok(Value::from(-12_i64)));
        assert_eq!(
            val(Some(&Value::from("15.001 "))),
            Ok(Value::from(15.001_f64))
        );
        assert_eq!(val(Some(&Value::from("1HELLO."))), Ok(Value::from(1_i64)));
        assert_eq!(val(Some(&Value::from("0x10"))), Ok(Value::from(0_i64)));
        assert_eq!(val(Some(&Value::from(".1"))), Ok(Value::from(0.1_f64)));
        assert_eq!(val(Some(&Value::from("-.1"))), Ok(Value::from(-0.1_f64)));
    }

    #[test]
    fn val_reports_xbase_style_argument_errors() {
        assert_eq!(
            val(Some(&Value::Nil)),
            Err(RuntimeError {
                message: "BASE 1098 Argument error (VAL)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Nil),
            })
        );
        assert_eq!(
            val(Some(&Value::from(10_i64))),
            Err(RuntimeError {
                message: "BASE 1098 Argument error (VAL)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
        assert_eq!(
            val(None),
            Err(RuntimeError {
                message: "BASE 1098 Argument error (VAL)".to_owned(),
                expected: None,
                actual: None,
            })
        );
    }

    #[test]
    fn val_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("VAL", &[Value::from("15.001 ")], &mut context),
            Ok(Value::from(15.001_f64))
        );

        let mut mutable_arguments = [Value::from("1HELLO.")];
        assert_eq!(
            call_builtin_mut("val", &mut mutable_arguments, &mut context),
            Ok(Value::from(1_i64))
        );
        assert_eq!(mutable_arguments[0], Value::from("1HELLO."));
    }

    #[test]
    fn type_follows_the_current_textual_runtime_baseline() {
        assert_eq!(type_value(Some(&Value::from("NIL"))), Ok(Value::from("U")));
        assert_eq!(type_value(Some(&Value::from(".T."))), Ok(Value::from("L")));
        assert_eq!(type_value(Some(&Value::from("10.5"))), Ok(Value::from("N")));
        assert_eq!(
            type_value(Some(&Value::from("{ 1, 2 }"))),
            Ok(Value::from("A"))
        );
        assert_eq!(
            type_value(Some(&Value::from("'abc'"))),
            Ok(Value::from("C"))
        );
        assert_eq!(
            type_value(Some(&Value::from("mxNotHere"))),
            Ok(Value::from("U"))
        );
    }

    #[test]
    fn type_reports_xbase_style_argument_errors() {
        assert_eq!(
            type_value(None),
            Err(RuntimeError {
                message: "BASE 1121 Argument error (TYPE)".to_owned(),
                expected: None,
                actual: None,
            })
        );
        assert_eq!(
            type_value(Some(&Value::from(100_i64))),
            Err(RuntimeError {
                message: "BASE 1121 Argument error (TYPE)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
    }

    #[test]
    fn type_dispatches_through_the_builtin_surfaces() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin("TYPE", &[Value::from(".F.")], &mut context),
            Ok(Value::from("L"))
        );

        let mut mutable_arguments = [Value::from("{ 1 }")];
        assert_eq!(
            call_builtin_mut("type", &mut mutable_arguments, &mut context),
            Ok(Value::from("A"))
        );
        assert_eq!(mutable_arguments[0], Value::from("{ 1 }"));
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
    fn replicate_and_space_follow_the_current_string_runtime_baseline() {
        assert_eq!(
            replicate(Some(&Value::from("")), Some(&Value::from(10_i64))),
            Ok(Value::from(""))
        );
        assert_eq!(
            replicate(Some(&Value::from("HE")), Some(&Value::from(3.7_f64))),
            Ok(Value::from("HEHEHE"))
        );
        assert_eq!(
            replicate(Some(&Value::from("HE")), Some(&Value::from(-3_i64))),
            Ok(Value::from(""))
        );

        assert_eq!(space(Some(&Value::from(0_i64))), Ok(Value::from("")));
        assert_eq!(space(Some(&Value::from(-10_i64))), Ok(Value::from("")));
        assert_eq!(space(Some(&Value::from(3.1_f64))), Ok(Value::from("   ")));
    }

    #[test]
    fn replicate_and_space_dispatch_through_the_immutable_builtin_surface() {
        let mut context = RuntimeContext::new();

        assert_eq!(
            call_builtin(
                "REPLICATE",
                &[Value::from("A"), Value::from(2_i64)],
                &mut context,
            ),
            Ok(Value::from("AA"))
        );
        assert_eq!(
            call_builtin("SPACE", &[Value::from(4_i64)], &mut context),
            Ok(Value::from("    "))
        );

        let mut mutable_arguments = [Value::from("HE"), Value::from(3.1_f64)];
        assert_eq!(
            call_builtin_mut("REPLICATE", &mut mutable_arguments, &mut context),
            Ok(Value::from("HEHEHE"))
        );
        assert_eq!(mutable_arguments[0], Value::from("HE"));
    }

    #[test]
    fn replicate_and_space_report_xbase_style_argument_errors() {
        assert_eq!(
            replicate(Some(&Value::from(200_i64)), Some(&Value::from(0_i64))),
            Err(RuntimeError {
                message: "BASE 1106 Argument error (REPLICATE)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
        assert_eq!(
            replicate(Some(&Value::from("A")), Some(&Value::from("B"))),
            Err(RuntimeError {
                message: "BASE 1106 Argument error (REPLICATE)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            space(Some(&Value::from("A"))),
            Err(RuntimeError {
                message: "BASE 1105 Argument error (SPACE)".to_owned(),
                expected: None,
                actual: Some(ValueKind::String),
            })
        );
        assert_eq!(
            at(Some(&Value::from(90_i64)), Some(&Value::from(100_i64))),
            Err(RuntimeError {
                message: "BASE 1108 Argument error (AT)".to_owned(),
                expected: None,
                actual: Some(ValueKind::Integer),
            })
        );
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
