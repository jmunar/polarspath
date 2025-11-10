#[cfg(feature = "extension-module")]
pub mod extension_impl {

    use crate::sample;
    use polars_core::prelude::{BinaryType, ChunkedArray, Field, PolarsResult, Series};
    use pyo3_polars::derive::{polars_expr, CallerContext};
    use serde::Deserialize;
    use structpath_protobuf::{get_type, get_value};

    #[derive(Deserialize)]
    pub struct ExtractKwargs {
        path: String,
    }

    pub fn user_get_type(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field> {
        let path = kwargs.path.as_str();
        get_type::<sample::User>(input_fields, path)
    }

    #[polars_expr(output_type_func_with_kwargs=user_get_type)]
    pub fn user_get_value(
        inputs: &[Series],
        context: CallerContext,
        kwargs: ExtractKwargs,
    ) -> PolarsResult<Series> {
        let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
        let path = kwargs.path.as_str();
        get_value::<sample::User>(ca, path, context.parallel())
    }
}
