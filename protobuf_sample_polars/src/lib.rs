#[cfg(feature = "extension-module")]
use polars_core::prelude::Field;
use polars_core::prelude::{AnyValue, BinaryType, ChunkedArray, PolarsError, PolarsResult, Series};

use prost::Message;
#[cfg(feature = "extension-module")]
use protobuf_sample::sample;
#[cfg(feature = "extension-module")]
use pyo3_polars::derive::polars_expr;
#[cfg(feature = "extension-module")]
use serde::Deserialize;
use structpath::StructPath;

#[cfg(feature = "extension-module")]
#[derive(Deserialize)]
pub struct ExtractKwargs {
    path: String,
}

#[cfg(feature = "extension-module")]
fn get_value_output_type<T>(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field>
where
    T: StructPath + Message + Default,
{
    let path = kwargs.path.as_str();
    let data_type_wrapper =
        T::get_type(path).map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?;
    let data_type = data_type_wrapper.polars;
    let name = input_fields
        .first()
        .map(|f| f.name().clone())
        .unwrap_or_else(|| "".into());
    Ok(Field::new(name, data_type))
}

pub fn get_value<T>(ca: &ChunkedArray<BinaryType>, path: &str) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
{
    let any_values = ca
        .into_iter()
        .map(|opt_bytes| match opt_bytes {
            Some(bytes) => {
                let message = T::decode(bytes)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                let value = message
                    .get_value(path)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                Ok(value.into_static())
            }
            None => Ok(AnyValue::Null),
        })
        .collect::<PolarsResult<Vec<AnyValue>>>()?;

    Series::from_any_values("".into(), &any_values, true)
}

#[cfg(feature = "extension-module")]
fn user_get_value_output_type(
    input_fields: &[Field],
    kwargs: ExtractKwargs,
) -> PolarsResult<Field> {
    get_value_output_type::<sample::User>(input_fields, kwargs)
}

#[cfg(feature = "extension-module")]
#[polars_expr(output_type_func_with_kwargs=user_get_value_output_type)]
fn user_get_value(inputs: &[Series], kwargs: ExtractKwargs) -> PolarsResult<Series> {
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    get_value::<sample::User>(ca, path)
}
