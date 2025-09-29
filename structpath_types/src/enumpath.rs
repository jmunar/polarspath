use crate::HasDataTypeOpt;
use polars_core::prelude::CategoricalMapping;
use std::sync::Arc;

/// This trait is used to define the internals of the conversion
/// of an enum into a Polars CategoricalMapping.
///
/// The end user methods are:
/// - `mapping`
///
pub trait EnumPath: HasDataTypeOpt {
    /// Returns the polars CategoricalMapping for this enum.
    fn mapping() -> &'static Arc<CategoricalMapping>;
}
