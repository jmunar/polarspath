use polars_core::prelude::{AnyValue, PolarsResult, Series};
use structpath::{FromValue, Value};

/// Trait to map Value to AnyValue
pub trait ToAnyValue
where
    Self: Sized,
{
    fn _value2rust(value: Value) -> Self;
    fn _rust2any(value: Self) -> AnyValue<'static>;
    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => Ok(Self::_rust2any(Self::_value2rust(value))),
        }
    }
}

impl ToAnyValue for String {
    fn _value2rust(value: Value) -> String {
        String::from_value(value)
    }

    fn _rust2any(value: String) -> AnyValue<'static> {
        AnyValue::StringOwned(value.into())
    }
}

impl ToAnyValue for i64 {
    fn _value2rust(value: Value) -> i64 {
        i64::from_value(value)
    }

    fn _rust2any(value: i64) -> AnyValue<'static> {
        AnyValue::Int64(value)
    }
}

impl ToAnyValue for f64 {
    fn _value2rust(value: Value) -> f64 {
        f64::from_value(value)
    }

    fn _rust2any(value: f64) -> AnyValue<'static> {
        AnyValue::Float64(value)
    }
}

impl ToAnyValue for bool {
    fn _value2rust(value: Value) -> bool {
        bool::from_value(value)
    }

    fn _rust2any(value: bool) -> AnyValue<'static> {
        AnyValue::Boolean(value)
    }
}

impl<T: ToAnyValue + Clone + Send + Sync + 'static> ToAnyValue for Vec<T>
where
    Vec<T>: FromValue<Value>,
{
    fn _value2rust(value: Value) -> Vec<T> {
        <Option<Vec<T>>>::from_value(value).unwrap()
    }

    fn _rust2any(value: Vec<T>) -> AnyValue<'static> {
        let any_values = value
            .into_iter()
            .map(T::_rust2any)
            .collect::<Vec<AnyValue>>();
        let series = Series::from_any_values("".into(), &any_values, true).unwrap();
        AnyValue::List(series)
    }
}

// impl<T: StructPath> ToAnyValue for &T {
//     fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
//         let field_defs = T::fields()
//             .iter()
//             .map(|field| {
//                 Field::new(
//                     field.name.clone().into(),
//                     field_type_to_data_type(field.r#type.clone()),
//                 )
//             })
//             .collect::<Vec<Field>>();

//         match &value {
//             Value::Option(None) => Ok(AnyValue::Null),
//             _ => {
//                 let field_values: Vec<AnyValue> = vec![
//                     // AnyValue::StringOwned(value.f_string.clone().into()),
//                     // AnyValue::Int64(value.f_integer),
//                 ];
//                 Ok(AnyValue::StructOwned(Box::new((field_values, field_defs))))
//             }
//         }
//     }
// }
