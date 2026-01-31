#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
extern crate polars_structpath_derive;

pub use polars_structpath_types::{ArrowBuffer, FromArrow, IntoArrow};

// Re-export the types crate for derive macros
pub use polars_structpath_types;

// Re-export polars types for convenience
pub use polars_structpath_types::polars_arrow;
pub use polars_structpath_types::polars_core;

#[cfg(feature = "derive")]
pub use polars_structpath_derive::{EnumPath, StructPath};
