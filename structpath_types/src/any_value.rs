use crate::data_type_opt::DataTypeOpt;
use polars_core::prelude::{AnyValue, PolarsDataType, Series};

/// Trait for types allowing conversion to AnyValue
///
/// The function to_any_value() takes a reference as first argument because
/// AnyValue is actually AnyValue<'_>. Thus, the return value lives as long
/// as the reference to the value.
pub trait IntoAnyValueWith<T> {
    type ChunkDataType: PolarsDataType;

    fn to_any_value(&self, value: &T) -> AnyValue;
}

impl IntoAnyValueWith<String> for DataTypeOpt {
    type ChunkDataType = ::polars_core::prelude::StringType;

    fn to_any_value(&self, value: &String) -> AnyValue {
        match self {
            DataTypeOpt::String => AnyValue::StringOwned(value.clone().into()),
            _ => panic!("Unsupported DataTypeOpt for String: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<i64> for DataTypeOpt {
    type ChunkDataType = ::polars_core::prelude::Int64Type;

    fn to_any_value(&self, value: &i64) -> AnyValue {
        match self {
            DataTypeOpt::Int64 => AnyValue::Int64(*value),
            _ => panic!("Unsupported DataTypeOpt for i64: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<f64> for DataTypeOpt {
    type ChunkDataType = ::polars_core::prelude::Float64Type;

    fn to_any_value(&self, value: &f64) -> AnyValue {
        match self {
            DataTypeOpt::Float64 => AnyValue::Float64(*value),
            _ => panic!("Unsupported DataTypeOpt for f64: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<bool> for DataTypeOpt {
    type ChunkDataType = ::polars_core::prelude::BooleanType;

    fn to_any_value(&self, value: &bool) -> AnyValue {
        match self {
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

impl<T> IntoAnyValueWith<Vec<T>> for DataTypeOpt
where
    DataTypeOpt: IntoAnyValueWith<T>,
{
    type ChunkDataType = ::polars_core::prelude::ListType;

    fn to_any_value(&self, value: &Vec<T>) -> AnyValue {
        match self {
            DataTypeOpt::List(inner_type) => {
                let any_values: Vec<AnyValue> = value
                    .into_iter()
                    .map(|item| (**inner_type).to_any_value(item))
                    .collect();
                let any_values = any_values;
                let series = Series::from_any_values("".into(), &any_values, true).unwrap();
                AnyValue::List(series)
            }
            _ => panic!("Unsupported DataTypeOpt for Vec<T>: {:?}", self),
        }
    }
}

impl<T> IntoAnyValueWith<Option<T>> for DataTypeOpt
where
    DataTypeOpt: IntoAnyValueWith<T>,
{
    type ChunkDataType = <DataTypeOpt as IntoAnyValueWith<T>>::ChunkDataType;

    fn to_any_value(&self, value: &Option<T>) -> AnyValue {
        match self {
            DataTypeOpt::Option(inner_type) => match value {
                Some(value) => inner_type.to_any_value(value),
                None => AnyValue::Null,
            },
            _ => panic!("Unsupported DataTypeOpt for Option<T>: {:?}", self),
        }
    }
}
