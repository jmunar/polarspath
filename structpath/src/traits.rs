use crate::{error::StructPathError, path::Path, value::Value};
use structpath_types::FieldsInfo;

pub trait StructPath {
    fn get_value_by_path(&self, path: &Path) -> Result<Value, StructPathError>;
    fn get_value(&self, path: &str) -> Result<Value, StructPathError>;
}

pub trait StructInfo {
    fn get_fields_info() -> FieldsInfo;
}
