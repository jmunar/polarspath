/// Trait for types that are kept boxed inside a `Value`
pub trait BoxedValue: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn BoxedValue>;
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}

impl<T: Clone + Send + Sync + 'static> BoxedValue for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn BoxedValue> {
        Box::new(self.clone())
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
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
macro_rules! impl_from_value {
    ($type:ty, $variant:path) => {
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
    };
}

impl_from_value!(i64, Value::Integer);
impl_from_value!(f64, Value::Float);
impl_from_value!(bool, Value::Boolean);
impl_from_value!(String, Value::String);

impl<T: FromValue<Value>> FromValue<Value> for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        match value {
            Value::Option(None) => None,
            Value::Option(Some(inner_type)) => Some(<T>::from_value(*inner_type)),
            _ => Some(<T>::from_value(value)),
        }
    }
}

impl<T: FromValue<Value> + Clone + 'static> FromValue<Value> for Vec<T> {
    fn from_value(value: Value) -> Vec<T> {
        match value {
            Value::Vec(inner_value) => *inner_value.into_any().downcast::<Vec<T>>().unwrap(),
            _ => panic!("Expected Vec, got {}", value_type(&value)),
        }
    }
}

macro_rules! impl_partial_eq_with_value {
    ($type:ty, $variant:path) => {
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

impl_partial_eq_with_value!(i64, Value::Integer);
impl_partial_eq_with_value!(f64, Value::Float);
impl_partial_eq_with_value!(bool, Value::Boolean);
impl_partial_eq_with_value!(String, Value::String);

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
