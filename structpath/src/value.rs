use crate::traits::StructPathTrait;

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

impl std::fmt::Debug for Box<dyn StructValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructValue").finish()
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

/// Trait for vector-like types that can be used as array values
pub trait ArrayValue: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn ArrayValue>;
    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: Clone + Send + Sync + 'static> ArrayValue for Vec<T>
where
    T: Into<Value> + Clone + std::fmt::Debug + PartialEq,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ArrayValue> {
        Box::new(self.clone())
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl std::fmt::Debug for Box<dyn ArrayValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().debug(f)
    }
}

impl Clone for Box<dyn ArrayValue> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl PartialEq for Box<dyn ArrayValue> {
    fn eq(&self, other: &Self) -> bool {
        self.as_any().type_id() == other.as_any().type_id()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Boxable(Box<dyn StructValue>),
    Array(Box<dyn ArrayValue>),
    Optional(Option<Box<Value>>),
}

impl Value {
    pub fn unwrap(&self) -> &Value {
        match self {
            Value::Optional(Some(value)) => value,
            _ => panic!("Value is not an optional"),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::String(value) => value,
            _ => panic!("Value is not a string"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Value::Integer(value) => *value,
            _ => panic!("Value is not an integer"),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Float(value) => *value,
            _ => panic!("Value is not a float"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(value) => *value,
            _ => panic!("Value is not a boolean"),
        }
    }

    pub fn as_struct<T: StructValue + 'static>(&self) -> &T {
        match self {
            Value::Boxable(boxed) => boxed.as_ref().as_any().downcast_ref::<T>().unwrap(),
            _ => panic!("Value is not a boxable"),
        }
    }

    pub fn as_array<T: ArrayValue + 'static>(&self) -> &T {
        match self {
            Value::Array(boxed) => boxed.as_ref().as_any().downcast_ref::<T>().unwrap(),
            _ => panic!("Value is not an array"),
        }
    }

    pub fn as_option(self) -> Option<Value> {
        match self {
            Value::Optional(Some(value)) => Some(*value),
            Value::Optional(None) => None,
            _ => panic!("Value is not an optional"),
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

impl<T: Into<Value> + Clone + Send + Sync + 'static> From<Vec<T>> for Value
where
    T: std::fmt::Debug + PartialEq,
{
    fn from(value: Vec<T>) -> Self {
        Self::Array(Box::new(value))
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
