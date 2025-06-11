/// Trait for types that are kept boxed inside a `Value`
pub trait BoxedValue: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn BoxedValue>;
}

impl<T: Clone + Send + Sync + 'static> BoxedValue for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn BoxedValue> {
        Box::new(self.clone())
    }
}

impl std::fmt::Debug for Box<dyn BoxedValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructValue").finish()
    }
}

impl Clone for Box<dyn BoxedValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn BoxedValue> {
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
    Boxed(Box<dyn BoxedValue>),
    Vec(Box<dyn BoxedValue>),
    Option(Option<Box<Value>>),
}

impl Value {
    pub fn unwrap(&self) -> &Value {
        match self {
            Value::Option(Some(value)) => value,
            _ => panic!("Value is not an optional"),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::String(value) => value,
            _ => panic!("Value is not a string"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::String(value) => value.clone(),
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

    pub fn as_unboxed<T: BoxedValue + 'static>(&self) -> &T {
        match self {
            Value::Boxed(boxed) => boxed.as_ref().as_any().downcast_ref::<T>().unwrap(),
            _ => panic!("Value is not a boxable"),
        }
    }

    pub fn as_array<T: BoxedValue + 'static>(&self) -> &T {
        match self {
            Value::Vec(boxed) => boxed.as_ref().as_any().downcast_ref::<T>().unwrap(),
            _ => panic!("Value is not an array"),
        }
    }

    pub fn as_option(self) -> Option<Value> {
        match self {
            Value::Option(Some(value)) => Some(*value),
            Value::Option(None) => None,
            _ => panic!("Value is not an optional"),
        }
    }
}
