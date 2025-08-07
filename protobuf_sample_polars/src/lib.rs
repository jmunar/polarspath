use polars_core::error::PolarsError;
use polars_core::prelude::{
    polars_err, BinaryType, BooleanType, ChunkedArray, CompatLevel, DataType, Field, Float64Type,
    Int64Type, PolarsResult, Series, StringType,
};
use polars_plan::dsl::FieldsMapper;
use prost::{DecodeError, Message};
use protobuf_sample::sample;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::IntoSeries;
use serde::Deserialize;
use structpath::{FieldType, StructPath};

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
        },
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}

fn extract_output(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field> {
    let path = kwargs.path.as_str();
    let path_type = sample::User::get_type(path).unwrap();
    let data_type = match_type(path_type);
    FieldsMapper::new(input_fields).with_dtype(data_type)
}

#[polars_expr(output_type_func_with_kwargs=extract_output)]
fn user_extract(inputs: &[Series], kwargs: ExtractKwargs) -> PolarsResult<Series> {
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    let path_type = sample::User::get_type(path).unwrap();

    match path_type {
        FieldType::String => {
            let chunk_out: Result<ChunkedArray<StringType>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes).map(|user| {
                                user.get_value(path)
                                    .unwrap()
                                    .as_string()
                            })
                        })
                        .transpose()
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::String) => {
            let chunk_out: Result<ChunkedArray<StringType>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes).map(|user| {
                                user.get_value(path)
                                    .unwrap()
                                    .as_option()
                                    .map(|s| s.as_string())
                            })
                        })
                        .transpose()
                        .map(|opt| opt.flatten())
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Integer => {
            let chunk_out: Result<ChunkedArray<Int64Type>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_i64())
                        })
                        .transpose()
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Integer) => {
            let chunk_out: Result<ChunkedArray<Int64Type>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_option().map(|u| u.as_i64()))
                        })
                        .transpose()
                        .map(|opt| opt.flatten())
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Float => {
            let chunk_out: Result<ChunkedArray<Float64Type>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_f64())
                        })
                        .transpose()
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Float) => {
            let chunk_out: Result<ChunkedArray<Float64Type>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_option().map(|u| u.as_f64()))
                        })
                        .transpose()
                        .map(|opt| opt.flatten())
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Boolean => {
            let chunk_out: Result<ChunkedArray<BooleanType>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_bool())
                        })
                        .transpose()
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        FieldType::Option(inner_type) if matches!(*inner_type, FieldType::Boolean) => {
            let chunk_out: Result<ChunkedArray<BooleanType>, DecodeError> = ca
                .into_iter()
                .map(|opt_bytes| {
                    opt_bytes
                        .map(|bytes| {
                            sample::User::decode(bytes)
                                .map(|user| user.get_value(path).unwrap().as_option().map(|u| u.as_bool()))
                        })
                        .transpose()
                        .map(|opt| opt.flatten())
                })
                .collect();
            Ok(chunk_out
                .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
                .into_series())
        }
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}
