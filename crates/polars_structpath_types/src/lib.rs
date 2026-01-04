#![doc = include_str!("../README.md")]

mod base;
mod enumpath;
mod structpath;
mod traits;

pub use base::{ArrowBufferOption, ArrowBufferVec};
pub use traits::{ArrowBuffer, FromArrow, IntoArrow};

// Re-export crates so macros can use them
pub extern crate paste;
pub extern crate polars_arrow;
pub extern crate polars_core;
