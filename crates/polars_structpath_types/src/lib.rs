//! # polars_structpath_types
//!
//! Core types and traits for converting Rust types to and from Apache Arrow arrays.
//!
//! This crate provides the foundational infrastructure for bidirectional conversion between
//! Rust data structures and Polars-compatible Arrow arrays. It enables seamless integration
//! of custom Rust types with the Polars DataFrame ecosystem.
//!
//! ## Overview
//!
//! `polars_structpath_types` provides:
//!
//! - **Core Traits**: `ArrowBuffer`, `IntoArrow`, and `FromArrow` for type conversion
//! - **Buffer Implementations**: Ready-to-use implementations for primitives, strings, collections
//! - **Conversion Macros**: `impl_struct_buffer!` and `impl_enum_buffer!` for custom types
//!
//! ## Key Features
//!
//! - **Type-Safe Conversion**: Compile-time guarantees for correct Arrow array generation
//! - **Null Handling**: Full support for nullable values via `Option<T>`
//! - **Nested Collections**: Support for `Vec<T>` and nested combinations like `Option<Vec<T>>`
//! - **Custom Types**: Macros for generating buffer code for structs and enums
//! - **Polars Integration**: Direct compatibility with Polars DataFrames and Series
//!
//! ## Example
//!
//! ```rust
//! use polars_structpath_types::{ArrowBuffer, IntoArrow, FromArrow};
//!
//! // Built-in types work out of the box
//! let mut buffer = i32::new_buffer(3);
//! buffer.push(1);
//! buffer.push(2);
//! buffer.push_null();
//!
//! let array = buffer.to_arrow().unwrap();
//! let values: Vec<i32> = i32::from_arrow(Box::new(array));
//! ```
//!
//! For custom structs and enums, use the provided macros:
//!
//! ```rust
//! use polars_structpath_types::impl_struct_buffer;
//!
//! pub struct Person {
//!     name: String,
//!     age: i32,
//! }
//!
//! impl_struct_buffer!(
//!     Person,
//!     [(name, String), (age, i32)]
//! );
//! ```
//!
//! ## Module Structure
//!
//! - `base`: Core buffer implementations for primitives, strings, and collections
//! - `structpath`: Macro for generating struct buffer implementations
//! - `enumpath`: Macro for generating enum buffer implementations
//! - `traits`: Core traits (`ArrowBuffer`, `IntoArrow`, `FromArrow`)

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
