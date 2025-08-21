use polars_core::prelude::{
    polars_err, BinaryType, BooleanType, ChunkedArray, CompatLevel, DataType, Field, Float64Type,
    Int64Type, PolarsError, PolarsResult, Series, StringType,
};
use polars_plan::dsl::FieldsMapper;
use prost::Message;
use protobuf_sample::sample;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::IntoSeries;
use serde::Deserialize;
use structpath::{FieldType, FromValue, StructPath};

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

fn extract_output(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field> {
    let path = kwargs.path.as_str();
    let path_type = sample::User::get_type_safe(path)
        .map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?;
    let data_type = match_type(path_type);
    FieldsMapper::new(input_fields).with_dtype(data_type)
}

/// Trait for types that can be extracted from protobuf User messages
trait UserExtractable {
    type ChunkedArrayType;

    fn extract_from_user_chunked_array(
        ca: &ChunkedArray<BinaryType>,
        path: &str,
        option: bool,
    ) -> Result<Self::ChunkedArrayType, PolarsError>;
}

/// Macro to generate UserExtractable implementations
macro_rules! impl_user_extractable {
    ($polars_type:ty, $rust_type:ty) => {
        impl UserExtractable for $polars_type {
            type ChunkedArrayType = ChunkedArray<$polars_type>;

            fn extract_from_user_chunked_array(
                ca: &ChunkedArray<BinaryType>,
                path: &str,
                option: bool,
            ) -> Result<Self::ChunkedArrayType, PolarsError> {
                ca.into_iter()
                    .map(|opt_bytes| {
                        match opt_bytes {
                            Some(bytes) => {
                                let user = sample::User::decode(bytes)
                                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                                let value = user.get_value_safe(path)
                                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                                
                                if option {
                                    Ok(Option::<$rust_type>::from_value(value))
                                } else {
                                    Ok(Some(<$rust_type>::from_value(value)))
                                }
                            }
                            None => Ok(None)
                        }
                    })
                    .collect()
            }
        }
    };
}

impl_user_extractable!(Int64Type, i64);
impl_user_extractable!(Float64Type, f64);
impl_user_extractable!(StringType, String);
impl_user_extractable!(BooleanType, bool);

#[polars_expr(output_type_func_with_kwargs=extract_output)]
fn user_extract(inputs: &[Series], kwargs: ExtractKwargs) -> PolarsResult<Series> {
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    let path_type = sample::User::get_type_safe(path).unwrap();

    match path_type {
        FieldType::String => StringType::extract_from_user_chunked_array(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::String) => {
            StringType::extract_from_user_chunked_array(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Integer => Int64Type::extract_from_user_chunked_array(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Integer) => {
            Int64Type::extract_from_user_chunked_array(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Float => Float64Type::extract_from_user_chunked_array(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Float) => {
            Float64Type::extract_from_user_chunked_array(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Boolean => BooleanType::extract_from_user_chunked_array(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Boolean) => {
            BooleanType::extract_from_user_chunked_array(ca, path, true).map(|c| c.into_series())
        }
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}
