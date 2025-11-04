use crate::{DataTypeOpt, DataTypeWrapper};
use polars_core::prelude::{AnyValue, DataType, PolarsDataType, Series};

/// Trait for types allowing conversion to AnyValue
///
/// The function to_any_value() takes a reference as first argument because
/// AnyValue is actually AnyValue<'_>. Thus, the return value lives as long
/// as the reference to the value.
pub trait IntoAnyValueWith<T> {
    type ChunkDataType: PolarsDataType;

    fn to_any_value(&self, value: &T) -> AnyValue<'_>;
}

impl IntoAnyValueWith<String> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::StringType;

    fn to_any_value(&self, value: &String) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::String => AnyValue::StringOwned(value.clone().into()),
            _ => panic!("Unsupported DataTypeOpt for String: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<i32> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::Int32Type;

    fn to_any_value(&self, value: &i32) -> AnyValue<'_> {
        match &self.polars {
            DataType::Int32 => AnyValue::Int32(*value),
            DataType::Enum(_, ref mapping) => {
                if let DataTypeOpt::Enum(ref info) = self.raw {
                    let polars_index = info
                        .rust_index_to_polars_index
                        .get(&(*value as u32))
                        .unwrap();
                    AnyValue::Enum(*polars_index, mapping)
                } else {
                    unreachable!()
                }
            }
            _ => panic!("Unsupported DataTypeOpt for i32: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<i64> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::Int64Type;

    fn to_any_value(&self, value: &i64) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::Int64 => AnyValue::Int64(*value),
            _ => panic!("Unsupported DataTypeOpt for i64: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<f64> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::Float64Type;

    fn to_any_value(&self, value: &f64) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::Float64 => AnyValue::Float64(*value),
            _ => panic!("Unsupported DataTypeOpt for f64: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<bool> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::BooleanType;

    fn to_any_value(&self, value: &bool) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::Boolean => AnyValue::Boolean(*value),
            _ => panic!("Unsupported DataTypeOpt for bool: {:?}", self),
        }
    }
}

// This implementation is probably faster, but won't work for structs
// impl<T> IntoAnyValueWith<Vec<T>> for DataTypeOpt
// where
//     DataTypeOpt: IntoAnyValueWith<T>,
//     ChunkedArray<<DataTypeOpt as IntoAnyValueWith<T>>::ChunkDataType>: FromIterator<Option<T>> + IntoSeries,
// {
//     type ChunkDataType = ::polars_core::prelude::ListType;
//     fn to_any_value(&self, value: Vec<T>) -> AnyValue {
//         match self {
//             DataTypeOpt::List(inner_type) => {
//                 let inner_ca: ChunkedArray<<DataTypeOpt as IntoAnyValueWith<T>>::ChunkDataType> = value.into_iter().map(Some).collect();
//                 AnyValue::List(inner_ca.into_series())
//             }
//             _ => panic!("Unsupported DataTypeOpt for Vec<T>: {:?}", self),
//         }
//     }
// }

impl<T> IntoAnyValueWith<Vec<T>> for DataTypeWrapper
where
    DataTypeWrapper: IntoAnyValueWith<T>,
{
    type ChunkDataType = ::polars_core::prelude::ListType;

    fn to_any_value(&self, value: &Vec<T>) -> AnyValue<'_> {
        match &self.raw {
            DataTypeOpt::List(inner_type) => {
                let any_values: Vec<AnyValue> = value
                    .iter()
                    .map(|item| inner_type.to_any_value(item))
                    .collect();
                // Use from_any_values_and_dtype() instead of from_any_values() because
                // Enum doesn't contain the full data type to create the series.
                let values = AnyValue::List(
                    Series::from_any_values_and_dtype(
                        "".into(),
                        &any_values,
                        &inner_type.polars,
                        true,
                    )
                    .unwrap(),
                );
                values
            }
            _ => panic!("Unsupported DataTypeOpt for Vec<T>: {:?}", self),
        }
    }
}

impl<T> IntoAnyValueWith<Option<T>> for DataTypeWrapper
where
    DataTypeWrapper: IntoAnyValueWith<T>,
{
    type ChunkDataType = <DataTypeWrapper as IntoAnyValueWith<T>>::ChunkDataType;

    fn to_any_value(&self, value: &Option<T>) -> AnyValue<'_> {
        match &self.raw {
            DataTypeOpt::Option(inner_type) => match value {
                Some(value) => inner_type.to_any_value(value),
                None => AnyValue::Null,
            },
            _ => panic!("Unsupported DataTypeWrapper for Option<T>: {:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_type_wrapper;

    #[test]
    fn test_to_any_value_string() {
        let data_type_wrapper = data_type_wrapper!(String);
        let value = "test".to_string();
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::StringOwned(value.into()));
    }

    #[test]
    fn test_to_any_value_i32() {
        let data_type_wrapper = data_type_wrapper!(Int32);
        let value = 1;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Int32(value));
    }

    #[test]
    fn test_to_any_value_i64() {
        let data_type_wrapper = data_type_wrapper!(Int64);
        let value = 1;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Int64(value));
    }

    #[test]
    fn test_to_any_value_f64() {
        let data_type_wrapper = data_type_wrapper!(Float64);
        let value = 1.0;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Float64(value));
    }

    #[test]
    fn test_to_any_value_bool() {
        let data_type_wrapper = data_type_wrapper!(Boolean);
        let value = true;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Boolean(value));
    }

    #[test]
    fn test_to_any_value_list_scalar() {
        let data_type_wrapper = data_type_wrapper!(List(String));
        let value = vec!["test".to_string()];
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::List(Series::from_iter(value)));
    }

    #[test]
    fn test_to_any_value_option_scalar() {
        let data_type_wrapper = data_type_wrapper!(Option(String));

        let value = Some("test".to_string());
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::StringOwned("test".into()));

        let value: Option<String> = None;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Null);
    }

    #[test]
    fn test_to_any_value_option_list() {
        let data_type_wrapper = data_type_wrapper!(Option(List(String)));

        let value = Some(vec!["test".to_string()]);
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(
            any_value,
            AnyValue::List(Series::from_iter(vec!["test".to_string()]))
        );

        let value: Option<Vec<String>> = None;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Null);
    }
}
