use crate::{error::StructPathError, path::Path, value::Value};
use structpath_types::{FieldInfo, FieldType};

/// This trait is used to define the methods that are available on a Rust struct
/// with nested fields being accessible using a path.
///
/// The end user methods are:
/// - `get_type`
/// - `get_type_safe`
/// - `get_value`
/// - `get_value_safe`
///
/// The difference safe and unsafe methods is that unsafe methods throw an error if:
/// - A field in the path doesn't exist
/// - An array item in the path is out of bounds
/// - A parent field or array item is null
///
/// , whereas save methods only throw an error in the first case
///
pub trait StructPath {
    // Deep fields (i.e., including StructPath)
    fn fields() -> &'static [FieldInfo];

    fn get_type_by_path(path: &Path) -> Result<FieldType, StructPathError>;
    fn get_type(path: &str) -> Result<FieldType, StructPathError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => Self::get_type_by_path(&path),
            Err(e) => Err(StructPathError::InvalidPath(e.to_string())),
        }
    }
    fn get_type_safe(path: &str) -> Result<FieldType, StructPathError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => match Self::get_type_by_path(&path) {
                Ok(t) if matches!(t, FieldType::Option(_)) => Ok(t),
                Ok(t) => Ok(FieldType::Option(Box::new(t))),
                Err(e) => Err(e),
            },
            Err(e) => Err(StructPathError::InvalidPath(e.to_string())),
        }
    }

    fn get_value_by_path(&self, path: &Path) -> Result<Value, StructPathError>;
    fn get_value(&self, path: &str) -> Result<Value, StructPathError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_value_by_path(&path),
            Err(e) => Err(StructPathError::InvalidPath(e.to_string())),
        }
    }
    fn get_value_safe(&self, path: &str) -> Result<Value, StructPathError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => match self.get_value_by_path(&path) {
                Ok(v) if matches!(v, Value::Option(_)) => Ok(v),
                Ok(v) => Ok(Value::Option(Some(Box::new(v)))),
                Err(StructPathError::IndexOutOfBounds(_)) | Err(StructPathError::NullValue) => {
                    Ok(Value::Option(None))
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(StructPathError::InvalidPath(e.to_string())),
        }
    }
}
