use polars_core::prelude::{
    polars_err, AnyValue, BinaryType, BooleanType, ChunkedArray, CompatLevel, DataType, Field,
    Float64Type, Int64Type, ListType, PolarsError, PolarsResult, Series, StringType,
};

use polars_plan::dsl::FieldsMapper;
use prost::Message;
use protobuf_sample::sample;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::IntoSeries;
use serde::Deserialize;
use std::iter::FromIterator;
use structpath::{FieldType, FromValue, StructPath, Value};

#[derive(Deserialize)]
pub struct ExtractKwargs {
    path: String,
}

fn match_type(path_type: FieldType) -> DataType {
    match path_type {
        FieldType::String => DataType::String,
        FieldType::Integer => DataType::Int64,
        FieldType::Float => DataType::Float64,
        FieldType::Boolean => DataType::Boolean,
        FieldType::Option(inner_type) => match_type(*inner_type),
        FieldType::Vec(inner_type) => {
            let inner_data_type = match_type(*inner_type);
            DataType::List(Box::new(inner_data_type))
        }
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}

fn extract_output<T>(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field>
where
    T: StructPath + Message + Default,
{
    let path = kwargs.path.as_str();
    let path_type = T::get_type_safe(path)
        .map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?;
    let data_type = match_type(path_type);
    FieldsMapper::new(input_fields).with_dtype(data_type)
}

/// Trait to map types to their corresponding ChunkedArray types and convert Value to AnyValue
trait ToChunkedArrayType {
    type ChunkedArrayType;

    /// Convert a Value to AnyValue for this type
    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>>;
}

impl ToChunkedArrayType for String {
    type ChunkedArrayType = ChunkedArray<StringType>;

    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => {
                let string_val = String::from_value(value);
                Ok(AnyValue::StringOwned(string_val.into()))
            }
        }
    }
}

impl ToChunkedArrayType for i64 {
    type ChunkedArrayType = ChunkedArray<Int64Type>;

    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => Ok(AnyValue::Int64(i64::from_value(value))),
        }
    }
}

impl ToChunkedArrayType for f64 {
    type ChunkedArrayType = ChunkedArray<Float64Type>;

    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => Ok(AnyValue::Float64(f64::from_value(value))),
        }
    }
}

impl ToChunkedArrayType for bool {
    type ChunkedArrayType = ChunkedArray<BooleanType>;

    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => Ok(AnyValue::Boolean(bool::from_value(value))),
        }
    }
}

impl<T: ToChunkedArrayType> ToChunkedArrayType for Vec<T>
where
    T: Clone + Send + Sync + 'static,
    T::ChunkedArrayType: FromIterator<Option<T>> + IntoSeries,
{
    type ChunkedArrayType = ChunkedArray<ListType>;

    fn to_any_value(value: Value) -> PolarsResult<AnyValue<'static>> {
        match &value {
            Value::Option(None) => Ok(AnyValue::Null),
            _ => {
                let vec_data = <&Vec<T>>::from_value(&value);
                let inner_ca: T::ChunkedArrayType = vec_data.iter().cloned().map(Some).collect();
                Ok(AnyValue::List(inner_ca.into_series()))
            }
        }
    }
}

/// Trait for message types that can extract fields to different types
trait ExtractFromChunkedArray: StructPath + Message + Default {
    fn extract_from_chunked_array<R>(
        ca: &ChunkedArray<BinaryType>,
        path: &str,
    ) -> PolarsResult<Series>
    where
        R: ToChunkedArrayType;
}

// Generic implementation for all message types
impl<T> ExtractFromChunkedArray for T
where
    T: StructPath + Message + Default,
{
    fn extract_from_chunked_array<R>(
        ca: &ChunkedArray<BinaryType>,
        path: &str,
    ) -> PolarsResult<Series>
    where
        R: ToChunkedArrayType,
    {
        let any_values = ca
            .into_iter()
            .map(|opt_bytes| match opt_bytes {
                Some(bytes) => {
                    let message = T::decode(bytes)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    let value = message
                        .get_value_safe(path)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    R::to_any_value(value)
                }
                None => Ok(AnyValue::Null),
            })
            .collect::<PolarsResult<Vec<AnyValue>>>()?;

        Series::from_any_values("".into(), &any_values, true)
    }
}

fn extract_impl_inner<T>(
    ca: &ChunkedArray<BinaryType>,
    path_type: FieldType,
    path: &str,
) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
{
    match path_type {
        FieldType::String => T::extract_from_chunked_array::<String>(ca, path),
        FieldType::Integer => T::extract_from_chunked_array::<i64>(ca, path),
        FieldType::Float => T::extract_from_chunked_array::<f64>(ca, path),
        FieldType::Boolean => T::extract_from_chunked_array::<bool>(ca, path),
        FieldType::Vec(inner_type) => match *inner_type {
            FieldType::String => T::extract_from_chunked_array::<Vec<String>>(ca, path),
            FieldType::Integer => T::extract_from_chunked_array::<Vec<i64>>(ca, path),
            FieldType::Float => T::extract_from_chunked_array::<Vec<f64>>(ca, path),
            FieldType::Boolean => T::extract_from_chunked_array::<Vec<bool>>(ca, path),
            _ => panic!("Unsupported vector inner type: {:?}", inner_type),
        },
        FieldType::Option(inner_type) => extract_impl_inner::<T>(ca, *inner_type, path),
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}

pub fn extract_impl<T>(ca: &ChunkedArray<BinaryType>, path: &str) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
{
    extract_impl_inner::<T>(ca, T::get_type_safe(path).unwrap(), path)
}

fn user_extract_output(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field> {
    extract_output::<sample::User>(input_fields, kwargs)
}

#[polars_expr(output_type_func_with_kwargs=user_extract_output)]
fn user_extract(inputs: &[Series], kwargs: ExtractKwargs) -> PolarsResult<Series> {
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    extract_impl::<sample::User>(ca, path)
}
