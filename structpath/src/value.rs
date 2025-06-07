use crate::StructPathTrait;

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
    Boxable(Box<dyn StructValue>),

    Array(Vec<Value>),
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

impl<T: StructValue + StructPathTrait> From<T> for Value {
    fn from(value: T) -> Self {
        Self::Boxable(value.clone_box())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        Self::Optional(value.map(|t| Box::new(t.into())))
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Self::Array(value.into_iter().map(|t| t.into()).collect())
    }
}

impl<T: Into<Value> + Clone> From<(&Vec<T>, usize)> for Value {
    fn from((vec, index): (&Vec<T>, usize)) -> Self {
        vec[index].clone().into()
    }
}

impl<T: Into<Value> + Clone> From<(&Option<Vec<T>>, usize)> for Value {
    fn from((opt_vec, index): (&Option<Vec<T>>, usize)) -> Self {
        match opt_vec {
            Some(vec) => vec[index].clone().into(),
            None => Value::Optional(None),
        }
    }
}
