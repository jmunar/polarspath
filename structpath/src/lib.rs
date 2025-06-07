mod error;
mod path;
mod traits;
mod value;
pub use error::StructPathError;
pub use path::{Path, PathComponent};
pub use traits::StructPathTrait;
pub use value::{StructValue, Value};

#[cfg(feature = "derive")]
extern crate structpath_derive;

#[cfg(feature = "derive")]
pub use structpath_derive::StructPath;
