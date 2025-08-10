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

pub trait FromValue<T> {
    fn from_value(value: T) -> Self;
}

// Macro to generate FromValue implementations for primitive types
macro_rules! impl_from_value_and_partial_eq {
    ($type:ty, $variant:path, $panic_msg:expr) => {
        impl FromValue<Value> for $type {
            fn from_value(value: Value) -> $type {
                match value {
                    $variant(inner_value) => inner_value,
                    Value::Option(Some(mid_value)) if matches!(*mid_value, $variant(_)) => {
                        let $variant(inner_value) = *mid_value else {
                            unreachable!()
                        };
                        inner_value
                    }
                    _ => panic!($panic_msg),
                }
            }
        }

        impl PartialEq<$type> for Value {
            fn eq(&self, other: &$type) -> bool {
                match self {
                    $variant(inner_value) => PartialEq::eq(inner_value, other),
                    Value::Option(Some(mid_value)) if matches!(**mid_value, $variant(_)) => {
                        let $variant(ref inner_value) = **mid_value else {
                            unreachable!()
                        };
                        PartialEq::eq(inner_value, other)
                    }
                    _ => false,
                }
            }
        }
    };
}

// Use the macro to generate implementations
impl_from_value_and_partial_eq!(i64, Value::Integer, "Value is not an integer");
impl_from_value_and_partial_eq!(f64, Value::Float, "Value is not a float");
impl_from_value_and_partial_eq!(bool, Value::Boolean, "Value is not a boolean");
impl_from_value_and_partial_eq!(String, Value::String, "Value is not a string");

impl<'a> FromValue<&'a Value> for &'a str {
    fn from_value(value: &'a Value) -> &'a str {
        match value {
            Value::String(value) => value,
            Value::Option(Some(value)) => <&str>::from_value(&**value),
            _ => panic!("Value is not a string"),
        }
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&'a str) -> bool {
        PartialEq::eq(&<&str>::from_value(self), other)
    }
}

impl<'a> FromValue<&'a Value> for Option<&'a str> {
    fn from_value(value: &'a Value) -> Option<&'a str> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(mid_type)) if matches!(**mid_type, Value::String(_)) => {
                if let Value::String(ref inner_type) = **mid_type {
                    Some(inner_type.as_str())
                } else {
                    panic!("Unreachable")
                }
            }
            _ => {
                panic!("Value is not a string")
            }
        }
    }
}

impl<'a, T: BoxedValue> FromValue<&'a Value> for &'a T {
    fn from_value(value: &'a Value) -> &'a T {
        match value {
            Value::Boxed(boxed) | Value::Vec(boxed) => {
                boxed.as_ref().as_any().downcast_ref::<T>().unwrap()
            }
            Value::Option(Some(value)) => <&T>::from_value(&**value),
            _ => panic!("Value is not a boxable"),
        }
    }
}

impl<T: FromValue<Value>> FromValue<Value> for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(inner_type)) => Some(<T>::from_value(*inner_type)),
            _ => panic!("Value is not an optional"),
        }
    }
}

impl<'a, T: BoxedValue> FromValue<&'a Value> for Option<&'a T> {
    fn from_value(value: &'a Value) -> Option<&'a T> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(ref mid_type))
                if matches!(**mid_type, Value::Boxed(_) | Value::Vec(_)) =>
            {
                Some(<&T>::from_value(mid_type))
            }
            _ => panic!("Value is not a boxable"),
        }
    }
}

impl Value {
    pub fn as_option(self) -> Option<Value> {
        match self {
            Value::Option(Some(value)) => Some(*value),
            Value::Option(None) => None,
            _ => panic!("Value is not an optional"),
        }
    }
}
