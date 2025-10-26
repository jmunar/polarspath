use crate::{DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, HasDataTypeWrapper, Path};
use indexmap::IndexMap;
use polars_core::prelude::{AnyValue, DataType, Field};

/// This trait is used to define the methods that are available on a Rust struct
/// with nested fields being accessible using a path.
///
/// The end user methods are:
/// - `fields`
/// - `get_type`
/// - `get_value`
///
/// Note that the `get` methods only throw an error if the path is invalid,
/// not if a parent of the field is null or the array item is out of bounds.
///
pub trait StructPath: HasDataTypeWrapper {
    fn fields_opt() -> &'static IndexMap<String, DataTypeWrapper> {
        match &Self::data_type_wrapper().raw {
            DataTypeOpt::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    fn fields() -> &'static Vec<Field> {
        match &Self::data_type_wrapper().polars {
            DataType::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    fn get_type_by_path(path: &Path) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        Self::data_type_wrapper().get_type_by_path(path)
    }

    fn get_type(path: &str) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        Self::data_type_wrapper().get_type(path)
    }

    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue<'_>, DataTypeWrapperError>;

    fn get_value(&self, path: &str) -> Result<AnyValue<'_>, DataTypeWrapperError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_value_by_path(&path),
            Err(e) => Err(DataTypeWrapperError::InvalidPath(e.to_string())),
        }
    }
}
