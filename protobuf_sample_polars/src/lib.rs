use polars_core::prelude::{
    polars_err, BinaryType, BooleanChunked, BooleanType, ChunkedArray, CompatLevel, DataType, Field, Float64Type,
    Int64Type, PolarsError, PolarsNumericType, PolarsResult, Series, StringChunked, StringType,
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

fn user_extract_typed<TP>(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
    option: bool,
) -> Result<ChunkedArray<TP>, PolarsError>
where
    TP: PolarsNumericType,
    TP::OwnedPhysical: FromValue<structpath::Value>,
    Option<TP::OwnedPhysical>: FromValue<structpath::Value>,
{
    let chunk_out: Result<ChunkedArray<TP>, PolarsError> = ca
        .into_iter()
        .map(|opt_bytes| {
            match opt_bytes {
                Some(bytes) => {

                    let user = sample::User::decode(bytes)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    let value = user.get_value_safe(path)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    
                    if option {
                        Ok(Option::<TP::OwnedPhysical>::from_value(value))
                    } else {
                        Ok(Some(TP::OwnedPhysical::from_value(value)))
                    }
                }
                None => Ok(None)
            }
        })
        .collect();

    chunk_out
}

fn user_extract_string(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
    option: bool,
) -> Result<ChunkedArray<StringType>, PolarsError> {
    let values: Result<Vec<Option<String>>, PolarsError> = ca
        .into_iter()
        .map(|opt_bytes| {
            match opt_bytes {
                Some(bytes) => {
                    let user = sample::User::decode(bytes)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    let value = user.get_value_safe(path)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

                    if option {
                        Ok(Option::<String>::from_value(value))
                    } else {
                        Ok(Some(String::from_value(value)))
                    }
                }
                None => Ok(None)
            }
        })
        .collect();
    
    values.map(|v| v.into_iter().collect::<StringChunked>())
}

fn user_extract_bool(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
    option: bool,
) -> Result<ChunkedArray<BooleanType>, PolarsError> {
    let values: Result<Vec<Option<bool>>, PolarsError> = ca
        .into_iter()
        .map(|opt_bytes| {
            match opt_bytes {
                Some(bytes) => {
                    let user = sample::User::decode(bytes)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    let value = user.get_value_safe(path)
                        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                    
                    if option {
                        Ok(Option::<bool>::from_value(value))
                    } else {
                        Ok(Some(bool::from_value(value)))
                    }
                }
                None => Ok(None)
            }
        })
        .collect();
    
    values.map(|v| v.into_iter().collect::<BooleanChunked>())
}

#[polars_expr(output_type_func_with_kwargs=extract_output)]
fn user_extract(inputs: &[Series], kwargs: ExtractKwargs) -> PolarsResult<Series> {
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    let path_type = sample::User::get_type_safe(path).unwrap();

    match path_type {
        FieldType::String => user_extract_string(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::String) => {
            user_extract_string(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Integer => user_extract_typed::<Int64Type>(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Integer) => {
            user_extract_typed::<Int64Type>(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Float => user_extract_typed::<Float64Type>(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Float) => {
            user_extract_typed::<Float64Type>(ca, path, true).map(|c| c.into_series())
        }
        FieldType::Boolean => user_extract_bool(ca, path, false).map(|c| c.into_series()),
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Boolean) => {
            user_extract_bool(ca, path, true).map(|c| c.into_series())
        }
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}
