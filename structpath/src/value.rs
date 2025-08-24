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

// Create a function for Value printing the type of the value
fn value_type(value: &Value) -> String {
    match value {
        Value::String(_) => "String".to_string(),
        Value::Integer(_) => "Integer".to_string(),
        Value::Float(_) => "Float".to_string(),
        Value::Boolean(_) => "Boolean".to_string(),
        Value::Boxed(_) => "Boxed".to_string(),
        Value::Vec(_) => "Vec".to_string(),
        Value::Option(inner_value) => match inner_value {
            Some(inner_value) => {
                let inner_type = value_type(inner_value);
                format!("Option({})", inner_type)
            }
            None => "Option(None)".to_string(),
        },
    }
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
                    _ => panic!("Expected {}, got {}", stringify!($type), value_type(&value)),
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

impl<T: FromValue<Value>> FromValue<Value> for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(inner_type)) => Some(<T>::from_value(*inner_type)),
            _ => Some(<T>::from_value(value)),
        }
    }
}

impl<'a> FromValue<&'a Value> for &'a str {
    fn from_value(value: &'a Value) -> &'a str {
        match value {
            Value::String(inner_value) => inner_value,
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::String(_)) => {
                let Value::String(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                inner_value
            }
            _ => panic!("Value is not a string"),
        }
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::String(inner_value) => PartialEq::eq(inner_value, other),
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::String(_)) => {
                let Value::String(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                PartialEq::eq(inner_value, other)
            }
            _ => false,
        }
    }
}

impl<'a> FromValue<&'a Value> for Option<&'a str> {
    fn from_value(value: &'a Value) -> Option<&'a str> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(mid_type)) if matches!(**mid_type, Value::String(_)) => {
                let Value::String(ref inner_type) = **mid_type else {
                    unreachable!()
                };
                Some(inner_type.as_str())
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
            Value::Boxed(inner_value) | Value::Vec(inner_value) => {
                inner_value.as_ref().as_any().downcast_ref::<T>().unwrap()
            }
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::Boxed(_)) => {
                let Value::Boxed(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                inner_value.as_ref().as_any().downcast_ref::<T>().unwrap()
            }
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::Vec(_)) => {
                let Value::Vec(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                inner_value.as_ref().as_any().downcast_ref::<T>().unwrap()
            }
            _ => panic!("Value is not a boxable"),
        }
    }
}

impl<T: BoxedValue + PartialEq<T>> PartialEq<&T> for Value {
    fn eq(&self, other: &&T) -> bool {
        match self {
            Value::Boxed(inner_value) | Value::Vec(inner_value) => PartialEq::eq(
                inner_value.as_ref().as_any().downcast_ref::<T>().unwrap(),
                other,
            ),
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::Boxed(_)) => {
                let Value::Boxed(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                PartialEq::eq(
                    inner_value.as_ref().as_any().downcast_ref::<T>().unwrap(),
                    other,
                )
            }
            Value::Option(Some(mid_value)) if matches!(**mid_value, Value::Vec(_)) => {
                let Value::Vec(ref inner_value) = **mid_value else {
                    unreachable!()
                };
                PartialEq::eq(
                    inner_value.as_ref().as_any().downcast_ref::<T>().unwrap(),
                    other,
                )
            }
            _ => false,
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

impl<T> PartialEq<Option<T>> for Value
where
    Value: PartialEq<T>,
{
    fn eq(&self, other: &Option<T>) -> bool {
        match self {
            Value::Option(None) => other.is_none(),
            Value::Option(Some(mid_value)) => {
                other.is_some() && PartialEq::eq(&**mid_value, other.as_ref().unwrap())
            }
            _ => false,
        }
    }
}
