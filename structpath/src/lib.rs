
mod error;
mod path;
mod value;
pub use error::StructPathError;
pub use path::{Path, PathComponent};
pub use value::{StructValue, Value};

#[cfg(feature = "derive")]
extern crate structpath_derive;

#[cfg(feature = "derive")]
pub use structpath_derive::StructPath;

#[cfg(feature = "derive")]
pub trait StructPathTrait {
    fn get_value_by_path(&self, path: &Path) -> Result<Value, StructPathError>;
    fn get_value(&self, path: &str) -> Result<Value, StructPathError>;
}
