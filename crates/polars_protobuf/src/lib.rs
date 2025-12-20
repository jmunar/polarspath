//! # polars_protobuf
//!
//! A Rust library that automatically generates [polars_structpath](https://github.com/jmunar/polarspath)
//! implementations for Protocol Buffer messages, enabling type-safe field access and Polars integration.
//!
//! ## Overview
//!
//! `polars_protobuf` provides seamless integration between Protocol Buffers and the `polars_structpath`
//! ecosystem. It enables automatic code generation during build time to apply `StructPath` and `EnumPath`
//! derives to protobuf messages and enums, allowing you to extract fields from binary protobuf columns
//! in Polars DataFrames using intuitive path notation.
//!
//! ## Key Features
//!
//! - **Automatic Code Generation**: Automatically applies `StructPath` and `EnumPath` derives to protobuf
//!   messages and enums during build time
//! - **Type-Safe Field Access**: Extract fields from binary protobuf columns using path notation like
//!   `"name"` or `"parent.name"`
//! - **Polars Integration**: Native support for converting protobuf fields to Polars `Series` and `AnyValue` types
//! - **Parallel Processing**: Optional parallel processing for improved performance when extracting values
//! - **Python Bindings**: Optional generation of Python extension modules for use with Polars Python API
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use polars_core::prelude::{BinaryType, ChunkedArray};
//! use polars_protobuf::{get_type, get_value};
//! use prost::Message;
//!
//! #[derive(polars_structpath::StructPath, Clone, Message)]
//! struct Person {
//!     #[prost(string, tag = "1")]
//!     name: String,
//!     #[prost(int64, tag = "2")]
//!     age: i64,
//! }
//!
//! fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get the type of a field
//!     let field = get_type::<Person>(&[], "name")?;
//!
//!     // Extract values from a binary column
//!     # // In real usage, you would have a ChunkedArray<BinaryType> from your DataFrame
//!     # let binary_column: ChunkedArray<BinaryType> = todo!();
//!     let name_series = get_value::<Person>(&binary_column, "name", true)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **`build`**: Enables the build-time code generation functionality (requires `prost-build` and `prost-types`)

mod base;

// Build module is available when the "build" feature is enabled
#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "build")]
mod string;

pub use base::{get_type, get_value};
