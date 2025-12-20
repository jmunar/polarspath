//! # polars_structpath
//!
//! The main user-facing library for dynamically accessing nested Rust structures using path
//! notation, with seamless integration to Polars DataFrames.
//!
//! ## Overview
//!
//! `polars_structpath` is the primary entry point for the `polars_structpath` ecosystem. It
//! provides a unified API that re-exports all necessary types, traits, and derive macros from
//! the underlying implementation crates.
//!
//! ## Key Features
//!
//! - **Path-Based Access**: Access nested struct fields using intuitive path notation like
//!   `"parent.name"` or `"parents[0].age"`
//! - **Polars Integration**: Native support for Polars `AnyValue` and `DataType` types
//! - **Derive Macros**: Procedural macros for automatically implementing `StructPath` and
//!   `EnumPath` traits (enabled via the `derive` feature)
//! - **Type Safety**: Compile-time type checking with derive macros
//!
//! ## Path Syntax
//!
//! Paths support the following syntax:
//!
//! - Simple fields: `"name"`
//! - Nested fields: `"parent.name"`
//! - Array indices: `"parents[0]"`
//! - Nested array access: `"parents[0].name"`
//!
//! ## Example Usage
//!
//! ```rust
//! use polars_core::prelude::{AnyValue, DataType};
//! use polars_structpath::StructPath;
//!
//! #[derive(StructPath, Debug, Clone)]
//! struct Parent {
//!     name: String,
//!     age: i64,
//! }
//!
//! #[derive(StructPath, Debug, Clone)]
//! struct User {
//!     name: String,
//!     age: i64,
//!     #[type_hint("struct")]
//!     parents: Vec<Parent>,
//! }
//!
//! let user = User {
//!     name: "John".to_string(),
//!     age: 32,
//!     parents: vec![Parent {
//!         name: "Joseph".to_string(),
//!         age: 65,
//!     }],
//! };
//!
//! // Access nested values using path notation
//! let father_name = user.get_value("parents[0].name").unwrap();
//! assert_eq!(father_name, AnyValue::String("Joseph"));
//! ```
//!
//! ## Supported Types
//!
//! The library supports:
//!
//! - **Scalar Types**: `String`, `i32`, `i64`, `f64`, `bool`
//! - **Nested Structures**: Any struct implementing the `StructPath` trait
//! - **Optional Types**: `Option<T>` for all types above
//! - **Vectors**: `Vec<T>` for all supported types
//! - **Enums**: Enums implementing the `EnumPath` trait
//!
//! ## Features
//!
//! - **`derive`** (default): Enables the `StructPath` and `EnumPath` derive macros
//! - **`std`** (default): Standard library support
//!
//! ## Re-exports
//!
//! This crate re-exports:
//!
//! - All types and traits from `polars_structpath_types`
//! - Derive macros from `polars_structpath_derive` (when `derive` feature is enabled)
//! - `polars_core` for Polars integration
//!
//! For more details on the underlying implementation, see:
//!
//! - [polars_structpath_types](../polars_structpath_types/index.html) - Core types and traits
//! - [polars_structpath_derive](../polars_structpath_derive/index.html) - Derive macro implementation

#[cfg(feature = "derive")]
extern crate polars_structpath_derive;

#[cfg(feature = "derive")]
pub use polars_structpath_derive::{EnumPath, StructPath};

extern crate polars_structpath_types;
pub use polars_structpath_types::{
    data_type_wrapper, indexmap, DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, EnumOptInfo,
    EnumPath, HasDataTypeWrapper, IntoAnyValueWith, Path, PathComponent, StructPath,
};

pub extern crate polars_core;
