use crate::{DataTypeOpt, DataTypeOptError, Path};
use indexmap::IndexMap;
use polars_core::prelude::{AnyValue, DataType, Field};

/// This trait is used to define the methods that are available on a Rust struct
/// with nested fields being accessible using a path.
///
/// The end user methods are:
/// - `fields`
/// - `data_type`
/// - `get_type`
/// - `get_value`
///
/// Note that the `get` methods only throw an error if the path is invalid,
/// not if a parent of the field is null or the array item is out of bounds.
///
pub trait StructPath {
    /// Typically implemented using a macro
    fn fields_opt() -> &'static IndexMap<String, DataTypeOpt>;

    /// Returns the polars Field definitions for this struct.
    /// Must be implemented with per-type static storage to avoid sharing cached values across types.
    fn fields() -> &'static [Field];

    /// Returns the DataTypeOpt representation of this struct.
    /// Must be implemented with per-type static storage to avoid sharing cached values across types.
    fn data_type_opt() -> &'static DataTypeOpt;

    /// Returns the polars DataType representation of this struct.
    /// Must be implemented with per-type static storage to avoid sharing cached values across types.
    fn data_type() -> &'static DataType;

    fn get_type_by_path(path: &Path) -> Result<DataTypeOpt, DataTypeOptError> {
        Self::data_type_opt().get_type_by_path(path)
    }

    fn get_type(path: &str) -> Result<DataTypeOpt, DataTypeOptError> {
        Self::data_type_opt().get_type(path)
    }

    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue, DataTypeOptError>;
    fn get_value(&self, path: &str) -> Result<AnyValue, DataTypeOptError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_value_by_path(&path),
            Err(e) => Err(DataTypeOptError::InvalidPath(e.to_string())),
        }
    }
}
