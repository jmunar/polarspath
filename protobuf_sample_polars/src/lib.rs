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

/// Trait to map types to their corresponding ChunkedArray types
trait ToChunkedArrayType {
    type ChunkedArrayType;
}

impl ToChunkedArrayType for String {
    type ChunkedArrayType = ChunkedArray<StringType>;
}

impl ToChunkedArrayType for i64 {
    type ChunkedArrayType = ChunkedArray<Int64Type>;
}

impl ToChunkedArrayType for f64 {
    type ChunkedArrayType = ChunkedArray<Float64Type>;
}

impl ToChunkedArrayType for bool {
    type ChunkedArrayType = ChunkedArray<BooleanType>;
}

impl<T: ToChunkedArrayType> ToChunkedArrayType for Vec<T> {
    type ChunkedArrayType = ChunkedArray<ListType>;
}

/// Trait for message types that can extract fields to different types
trait ExtractFromChunkedArray: StructPath + Message + Default {
    fn extract_from_chunked_array<R>(
        ca: &ChunkedArray<BinaryType>,
        path: &str,
    ) -> PolarsResult<Series>
    where
        R: ToChunkedArrayType + FromValue<Value>,
        R::ChunkedArrayType: FromIterator<Option<R>> + IntoSeries;
}

// Generic helper function for Vec<R> extraction where R is any supported inner type
fn extract_vec_from_chunked_array<T, R>(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
    R: ToChunkedArrayType + FromValue<Value> + Clone + Send + Sync + 'static,
    R::ChunkedArrayType: FromIterator<Option<R>> + IntoSeries,
{
    // Combine both iterations into a single pass and create Series directly
    let any_values = ca
        .into_iter()
        .map(|opt_bytes| match opt_bytes {
            Some(bytes) => {
                let message = T::decode(bytes)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                let value = message
                    .get_value_safe(path)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                let result = <&Vec<R>>::from_value(&value);

                // Create ChunkedArray directly from Vec<R>
                let inner_ca: R::ChunkedArrayType = result.iter().cloned().map(Some).collect();
                // Convert ChunkedArray to Series only for AnyValue::List
                Ok(AnyValue::List(inner_ca.into_series()))
            }
            None => Ok(AnyValue::Null),
        })
        .collect::<PolarsResult<Vec<AnyValue>>>()?;

    // Create final Series directly from AnyValue vector
    let result = Series::from_any_values("".into(), &any_values, true)?;

    Ok(result)
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
        R: ToChunkedArrayType + FromValue<Value>,
        R::ChunkedArrayType: FromIterator<Option<R>> + IntoSeries,
    {
        let chunked_array: R::ChunkedArrayType = ca
            .into_iter()
            .map(|opt_bytes| match opt_bytes {
                Some(bytes) => {
                    let message = T::decode(bytes)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    let value = message
                        .get_value_safe(path)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    Ok(Option::<R>::from_value(value))
                }
                None => Ok(None),
            })
            .collect::<PolarsResult<R::ChunkedArrayType>>()?;

        Ok(chunked_array.into_series())
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
            FieldType::String => extract_vec_from_chunked_array::<T, String>(ca, path),
            FieldType::Integer => extract_vec_from_chunked_array::<T, i64>(ca, path),
            FieldType::Float => extract_vec_from_chunked_array::<T, f64>(ca, path),
            FieldType::Boolean => extract_vec_from_chunked_array::<T, bool>(ca, path),
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
