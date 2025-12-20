//! # polars_structpath_types
//!
//! This crate provides the core types and traits that form the foundation of the
//! `polars_structpath` ecosystem. It enables dynamic path-based access to nested Rust
//! structures with seamless integration to Polars DataFrames.
//!
//! ## Overview
//!
//! `polars_structpath_types` is a foundational library that defines:
//!
//! - **Core Types**: Data structures for representing paths, data types, and type metadata
//! - **Core Traits**: Interfaces for path-based data access (`StructPath`, `EnumPath`, `HasDataTypeWrapper`)
//! - **Type System**: A wrapper around Polars `DataType` that supports additional metadata needed for path traversal
//! - **Conversion Utilities**: Traits and implementations for converting Rust values to Polars `AnyValue`
//!
//! ## Key Concepts
//!
//! ### Path-Based Access
//!
//! The crate enables accessing nested struct fields using string paths, similar to JSON path notation:
//!
//! - Simple fields: `"name"`
//! - Nested fields: `"parent.name"`
//! - Array indices: `"parents[0].name"`
//!
//! ### Type System
//!
//! The `DataTypeWrapper` type extends Polars' `DataType` with additional metadata needed for
//! path traversal, including support for Rust-specific types like `Option<T>` and enum mappings.
//!
//! ## Example Usage
//!
//! While this crate is primarily used internally by `polars_structpath` and its derive macros,
//! here's how the core types work together:
//!
//! ```rust
//! use polars_structpath_types::{Path, PathComponent, DataTypeWrapper, DataTypeOpt};
//!
//! // Parse a path string
//! let path = Path::from_str("parents[0].name").unwrap();
//! assert_eq!(path.components().len(), 2);
//! assert_eq!(
//!     path.components()[0],
//!     PathComponent::ArrayIndex("parents".to_string(), 0)
//! );
//! assert_eq!(
//!     path.components()[1],
//!     PathComponent::Field("name".to_string())
//! );
//!
//! // Work with data types
//! let string_type = DataTypeWrapper::new(DataTypeOpt::String);
//! assert_eq!(string_type.polars, polars_core::prelude::DataType::String);
//! ```
//!
//! ## Integration with polars_structpath
//!
//! This crate is used by:
//!
//! - `polars_structpath`: The main user-facing library that wraps this crate
//! - `polars_structpath_derive`: The derive macro implementation that generates code using these types
//! - `polars_protobuf`: Protocol Buffers integration that leverages these types
//!
//! For end-user documentation and examples, see the [polars_structpath](../polars_structpath/index.html) crate.

mod any_value;
mod data_type_wrapper;
mod enumpath;
mod macros_data_type;
mod macros_data_type_wrapper;
mod path;
mod structpath;

pub use any_value::IntoAnyValueWith;
pub use data_type_wrapper::{
    DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, EnumOptInfo, HasDataTypeWrapper,
};
pub use enumpath::EnumPath;
pub use path::{Path, PathComponent, PathParseError};
pub use structpath::StructPath;

pub use indexmap;
