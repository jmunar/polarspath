//! Module for converting Rust values to Polars `AnyValue`.
//!
//! This module provides the `IntoAnyValueWith` trait and implementations for
//! converting various Rust types (scalars, vectors, options) to Polars `AnyValue`,
//! which is the type-erased value type used in Polars DataFrames and Series.

use crate::{DataTypeOpt, DataTypeWrapper};
use polars_core::prelude::{AnyValue, DataType, PolarsDataType, Series};

/// Trait for converting Rust values to Polars `AnyValue`.
///
/// This trait enables conversion of Rust types to Polars `AnyValue`, which is
/// the type-erased value type used in Polars DataFrames and Series. The conversion
/// is performed using a `DataTypeWrapper` that provides the necessary type information.
///
/// # Lifetime Semantics
///
/// The `to_any_value()` method takes a reference to the value because `AnyValue`
/// is actually `AnyValue<'_>` - a type with a lifetime parameter. The returned
/// `AnyValue` lives as long as the reference to the input value, allowing for
/// zero-copy conversions when possible.
///
/// # Type Parameter
///
/// The trait is generic over the input type `T` that will be converted. The
/// `ChunkDataType` associated type specifies the Polars chunk data type that
/// corresponds to this conversion.
///
/// # Example
///
/// ```rust
/// use polars_core::prelude::AnyValue;
/// use polars_structpath_types::{DataTypeWrapper, DataTypeOpt, IntoAnyValueWith};
///
/// // Convert a string
/// let wrapper = DataTypeWrapper::new(DataTypeOpt::String);
/// let value = "hello".to_string();
/// let any_value = wrapper.to_any_value(&value);
/// assert!(matches!(any_value, AnyValue::StringOwned(_)));
///
/// // Convert an integer
/// let wrapper = DataTypeWrapper::new(DataTypeOpt::Int64);
/// let value = 42i64;
/// let any_value = wrapper.to_any_value(&value);
/// assert_eq!(any_value, AnyValue::Int64(42));
/// ```
pub trait IntoAnyValueWith<T> {
    /// The Polars chunk data type for this conversion.
    type ChunkDataType: PolarsDataType;

    /// Converts a Rust value to a Polars `AnyValue`.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the Rust value to convert
    ///
    /// # Returns
    ///
    /// A `AnyValue<'_>` representing the converted value. The lifetime of the
    /// returned value is tied to the lifetime of the input reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use polars_core::prelude::AnyValue;
    /// use polars_structpath_types::{DataTypeWrapper, DataTypeOpt, IntoAnyValueWith};
    ///
    /// let wrapper = DataTypeWrapper::new(DataTypeOpt::String);
    /// let value = "test".to_string();
    /// let any_value = wrapper.to_any_value(&value);
    /// ```
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

impl IntoAnyValueWith<Vec<u8>> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::BinaryType;

    fn to_any_value(&self, value: &Vec<u8>) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::Bytes => AnyValue::BinaryOwned(value.clone()),
            _ => panic!("Unsupported DataTypeOpt for Vec<u8>: {:?}", self),
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

impl IntoAnyValueWith<u32> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::UInt32Type;

    fn to_any_value(&self, value: &u32) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::UInt32 => AnyValue::UInt32(*value),
            _ => panic!("Unsupported DataTypeOpt for u32: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<u64> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::UInt64Type;

    fn to_any_value(&self, value: &u64) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::UInt64 => AnyValue::UInt64(*value),
            _ => panic!("Unsupported DataTypeOpt for u64: {:?}", self),
        }
    }
}

impl IntoAnyValueWith<f32> for DataTypeWrapper {
    type ChunkDataType = ::polars_core::prelude::Float32Type;

    fn to_any_value(&self, value: &f32) -> AnyValue<'_> {
        match self.raw {
            DataTypeOpt::Float32 => AnyValue::Float32(*value),
            _ => panic!("Unsupported DataTypeOpt for f32: {:?}", self),
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
    fn test_to_any_value_bytes() {
        let data_type_wrapper = data_type_wrapper!(Bytes);
        let value = b"test".to_vec();
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::BinaryOwned(value));
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
    fn test_to_any_value_u32() {
        let data_type_wrapper = data_type_wrapper!(UInt32);
        let value = 1u32;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::UInt32(value));
    }

    #[test]
    fn test_to_any_value_u64() {
        let data_type_wrapper = data_type_wrapper!(UInt64);
        let value = 1u64;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::UInt64(value));
    }

    #[test]
    fn test_to_any_value_f32() {
        let data_type_wrapper = data_type_wrapper!(Float32);
        let value = 1.0f32;
        let any_value = data_type_wrapper.to_any_value(&value);
        assert_eq!(any_value, AnyValue::Float32(value));
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
