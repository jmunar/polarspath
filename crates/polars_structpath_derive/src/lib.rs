//! # polars_structpath_derive
//!
//! This crate provides procedural derive macros for the `polars_structpath` ecosystem.
//! It automatically generates implementations of the `StructPath` and `EnumPath` traits,
//! enabling dynamic path-based access to nested Rust structures with seamless integration
//! to Polars DataFrames.
//!
//! ## Overview
//!
//! `polars_structpath_derive` is a procedural macro crate that generates code for:
//!
//! - **StructPath derive macro**: Automatically implements the `StructPath` trait for structs,
//!   enabling path-based field access using string paths like `"parent.name"` or `"parents[0].age"`
//! - **EnumPath derive macro**: Automatically implements the `EnumPath` trait for enums,
//!   enabling path-based enum value access with case conversion support
//!
//! ## Usage
//!
//! ### StructPath Derive
//!
//! Apply the `#[derive(StructPath)]` attribute to structs with named fields:
//!
//! ```ignore
//! use polars_structpath::StructPath;
//!
//! #[derive(StructPath)]
//! struct User {
//!     name: String,
//!     age: i64,
//!     #[type_hint("struct")]
//!     parent: Option<Parent>,
//! }
//!
//! #[derive(StructPath)]
//! struct Parent {
//!     name: String,
//! }
//! ```
//!
//! The `#[type_hint]` attribute can be used to provide type information for complex types
//! like nested structs, enums, or collections that cannot be automatically inferred.
//!
//! ### EnumPath Derive
//!
//! Apply the `#[derive(EnumPath)]` attribute to enums:
//!
//! ```ignore
//! use polars_structpath::EnumPath;
//!
//! #[derive(EnumPath)]
//! #[enum_path(camel_case_to_upper_snake_case)]
//! enum Status {
//!     Active,
//!     Inactive,
//! }
//! ```
//!
//! The `#[enum_path]` attribute supports case conversion functions for mapping enum variant
//! names to string representations. Supported conversion functions include:
//!
//! - `camel_case_to_snake_case`
//! - `camel_case_to_upper_snake_case`
//! - `snake_case_to_camel_case`
//!
//! ## Generated Code
//!
//! The derive macros generate implementations of:
//!
//! - `StructPath::get_type()` - Returns type information for a field by path string
//! - `StructPath::get_type_by_path()` - Returns type information for a field by parsed path
//! - `StructPath::get_value()` - Returns the value of a field by path string
//! - `StructPath::get_value_by_path()` - Returns the value of a field by parsed path
//! - `HasDataTypeWrapper::get_data_type_wrapper()` - Returns the type metadata map
//!
//! For enums, the `EnumPath` trait provides similar methods for accessing enum values by name.
//!
//! ## Integration
//!
//! This crate is used by:
//!
//! - `polars_structpath`: The main user-facing library that re-exports these derive macros
//! - End users: Who apply `#[derive(StructPath)]` or `#[derive(EnumPath)]` to their structs and enums
//!
//! For end-user documentation and examples, see the [polars_structpath](../polars_structpath/index.html) crate.

mod enumpath;
mod string;
mod structpath;
mod utils;

use enumpath::derive_enum_path_impl;
use proc_macro::TokenStream;
use structpath::derive_struct_path_impl;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructPath, attributes(type_hint))]
pub fn derive_struct_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_path_impl(input).into()
}

#[proc_macro_derive(EnumPath, attributes(type_hint, enum_path))]
pub fn derive_enum_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_enum_path_impl(input).into()
}
