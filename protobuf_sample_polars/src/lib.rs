mod any;
mod field;

use any::ToAnyValue;
use field::field_type_to_data_type;

use polars_core::prelude::{
    polars_err, AnyValue, BinaryType, ChunkedArray, CompatLevel, Field, PolarsError, PolarsResult,
    Series,
};

use prost::Message;
use protobuf_sample::sample;
use pyo3_polars::{derive::polars_expr, export::polars_plan::dsl::FieldsMapper};
use serde::Deserialize;
use structpath::{FieldType, StructPath};

#[derive(Deserialize)]
pub struct ExtractKwargs {
    path: String,
}

/// Trait for message types that can extract fields to different types
trait ExtractFromChunkedArray: StructPath + Message + Default {
    fn extract_from_chunked_array<R>(
        ca: &ChunkedArray<BinaryType>,
        path: &str,
    ) -> PolarsResult<Series>
    where
        R: ToAnyValue;
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
        R: ToAnyValue,
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

fn extract_output<T>(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field>
where
    T: StructPath + Message + Default,
{
    let path = kwargs.path.as_str();
    let path_type = T::get_type_safe(path)
        .map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?;
    let data_type = field_type_to_data_type(path_type);
    FieldsMapper::new(input_fields).with_dtype(data_type)
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
