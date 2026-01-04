#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
extern crate polars_structpath_derive;

pub use polars_structpath_types::{ArrowBuffer, FromArrow, IntoArrow};

#[cfg(feature = "derive")]
pub use polars_structpath_derive::{EnumPath, StructPath};
