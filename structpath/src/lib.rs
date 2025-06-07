mod error;
mod path;
pub mod prelude;
mod traits;
mod value;

#[cfg(feature = "derive")]
extern crate structpath_derive;

#[cfg(feature = "derive")]
pub use structpath_derive::StructPath;
