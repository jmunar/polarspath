use crate::{error::StructPathError, path::Path, value::Value};

pub trait StructPathTrait {
    fn get_value_by_path(&self, path: &Path) -> Result<Value, StructPathError>;
    fn get_value(&self, path: &str) -> Result<Value, StructPathError>;
}
