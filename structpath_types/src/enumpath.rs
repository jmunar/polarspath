use crate::HasDataTypeWrapper;
use polars_core::prelude::{CategoricalMapping, DataType};
use std::sync::Arc;

/// This trait is used to define the internals of the conversion
/// of an enum into a Polars CategoricalMapping.
///
pub trait EnumPath: HasDataTypeWrapper {
    /// Returns the polars CategoricalMapping for this enum.
    fn mapping() -> &'static Arc<CategoricalMapping> {
        match Self::data_type() {
            DataType::Enum(_, ref mapping) => mapping,
            _ => unreachable!(),
        }
    }
}
