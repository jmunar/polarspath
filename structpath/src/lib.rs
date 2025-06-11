mod error;
mod path;
mod traits;
mod value;

pub use error::StructPathError;
pub use path::{Path, PathComponent};
pub use traits::{StructInfo, StructPath};
pub use value::Value;

#[cfg(feature = "derive")]
extern crate structpath_derive;

#[cfg(feature = "derive")]
pub use structpath_derive::StructPath;

#[cfg(feature = "derive")]
pub use structpath_derive::StructInfo;
