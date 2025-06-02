/// Trait for types that can be used as struct values
pub trait StructValue: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn StructValue>;
}

impl<T: Clone + Send + Sync + 'static> StructValue for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn StructValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn StructValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn StructValue> {
    fn eq(&self, other: &Self) -> bool {
        self.as_any().type_id() == other.as_any().type_id()
    }
}

impl std::fmt::Debug for Box<dyn StructValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructValue").finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Enum(Box<dyn StructValue>),
    Struct(Box<dyn StructValue>),

    StringArray(Vec<String>),
    IntegerArray(Vec<i64>),
    FloatArray(Vec<f64>),
    BooleanArray(Vec<bool>),
    EnumArray(Vec<Box<dyn StructValue>>),
    StructArray(Vec<Box<dyn StructValue>>),

    StringOptArray(Vec<Option<String>>),
    IntegerOptArray(Vec<Option<i64>>),
    FloatOptArray(Vec<Option<f64>>),
    BooleanOptArray(Vec<Option<bool>>),
    EnumOptArray(Vec<Option<Box<dyn StructValue>>>),
    StructOptArray(Vec<Option<Box<dyn StructValue>>>),

    Optional(Option<Box<Value>>),
}

impl Value {
    /// Returns a reference to the inner value if this is an Optional variant
    pub fn as_ref(&self) -> Option<&Value> {
        match self {
            Value::Optional(Some(value)) => Some(value),
            _ => None,
        }
    }

    /// Takes ownership of the inner value if this is an Optional variant
    pub fn into_inner(self) -> Option<Value> {
        match self {
            Value::Optional(Some(value)) => Some(*value),
            _ => None,
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Self {
        Self::StringArray(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Self::IntegerArray(value)
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Self::FloatArray(value)
    }
}

impl From<Vec<bool>> for Value {
    fn from(value: Vec<bool>) -> Self {
        Self::BooleanArray(value)
    }
}

impl From<Vec<Option<String>>> for Value {
    fn from(value: Vec<Option<String>>) -> Self {
        Self::StringOptArray(value)
    }
}

impl From<Vec<Option<i64>>> for Value {
    fn from(value: Vec<Option<i64>>) -> Self {
        Self::IntegerOptArray(value)
    }
}

impl From<Vec<Option<f64>>> for Value {
    fn from(value: Vec<Option<f64>>) -> Self {
        Self::FloatOptArray(value)
    }
}

impl From<Vec<Option<bool>>> for Value {
    fn from(value: Vec<Option<bool>>) -> Self {
        Self::BooleanOptArray(value)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        Self::Optional(value.map(|t| Box::new(t.into())))
    }
}

impl From<(&Vec<String>, usize)> for Value {
    fn from((vec, index): (&Vec<String>, usize)) -> Self {
        vec[index].clone().into()
    }
}

impl From<(&Vec<i64>, usize)> for Value {
    fn from((vec, index): (&Vec<i64>, usize)) -> Self {
        vec[index].into()
    }
}

impl From<(&Vec<f64>, usize)> for Value {
    fn from((vec, index): (&Vec<f64>, usize)) -> Self {
        vec[index].into()
    }
}

impl From<(&Vec<bool>, usize)> for Value {
    fn from((vec, index): (&Vec<bool>, usize)) -> Self {
        vec[index].into()
    }
}

impl From<(&Vec<Option<String>>, usize)> for Value {
    fn from((vec, index): (&Vec<Option<String>>, usize)) -> Self {
        vec[index].clone().into()
    }
}

impl From<(&Vec<Option<i64>>, usize)> for Value {
    fn from((vec, index): (&Vec<Option<i64>>, usize)) -> Self {
        vec[index].into()
    }
}

impl From<(&Vec<Option<f64>>, usize)> for Value {
    fn from((vec, index): (&Vec<Option<f64>>, usize)) -> Self {
        vec[index].into()
    }
}

impl From<(&Vec<Option<bool>>, usize)> for Value {
    fn from((vec, index): (&Vec<Option<bool>>, usize)) -> Self {
        vec[index].into()
    }
}

impl<T> From<(&Option<Vec<T>>, usize)> for Value
where
    T: Clone + Into<Value>,
{
    fn from((opt_vec, index): (&Option<Vec<T>>, usize)) -> Self {
        match opt_vec {
            Some(vec) => vec[index].clone().into(),
            None => Value::Optional(None),
        }
    }
}
