#![doc = include_str!("../README.md")]

mod base;
mod enumpath;
mod structpath;
mod traits;

pub use base::{ArrowBufferOption, ArrowBufferVec};
pub use enumpath::{
    extract_dictionary_keys, extract_dictionary_values, try_extract_dictionary_values,
};
pub use traits::{ArrowBuffer, FromArrow, IntoArrow};

// Re-export crates so macros can use them
pub extern crate paste;
pub extern crate polars_arrow;
pub extern crate polars_core;

#[cfg(feature = "derive")]
extern crate polars_structpath_derive;

#[cfg(feature = "derive")]
pub use polars_structpath_derive::{EnumPath, StructPath};
