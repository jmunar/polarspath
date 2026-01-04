#![doc = include_str!("../README.md")]

mod enumpath;
mod structpath;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use enumpath::derive_enum_path_impl;
use structpath::derive_struct_path_impl;

/// Derive macro for generating Arrow buffer implementations for structs.
///
/// This macro automatically generates code that calls `impl_struct_buffer!` from
/// `polars_structpath_types`, which creates:
///
/// - A buffer struct (e.g., `UserBuffer`) implementing `ArrowBuffer`
/// - `IntoArrow` implementation for the struct
/// - `FromArrow` implementation for the struct
///
/// # Requirements
///
/// - The struct must have named fields (not tuple structs or unit structs)
/// - All field types must implement `IntoArrow` and `FromArrow` from `polars_structpath_types`
///
/// # Attributes
///
/// - `#[type_hint(...)]`: Currently accepted but not used. Reserved for future use.
///
/// # Example
///
/// ```ignore
/// use polars_structpath::StructPath;
/// use polars_structpath_types::{IntoArrow, ArrowBuffer};
///
/// #[derive(StructPath)]
/// struct User {
///     name: String,
///     age: i64,
/// }
///
/// // Now User implements IntoArrow and FromArrow
/// let mut buffer = User::new_buffer(1);
/// buffer.push(User {
///     name: "Alice".to_string(),
///     age: 30,
/// });
/// let array = buffer.to_arrow().unwrap();
/// ```
#[proc_macro_derive(StructPath, attributes(type_hint))]
pub fn derive_struct_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_path_impl(input).into()
}

/// Derive macro for generating Arrow buffer implementations for enums.
///
/// This macro automatically generates code that calls `impl_enum_buffer!` from
/// `polars_structpath_types`, which creates:
///
/// - A buffer struct (e.g., `StatusBuffer`) implementing `ArrowBuffer`
/// - Helper methods for index conversion (`from_arrow_idx`, `rust_idx_to_arrow_idx`)
/// - `IntoArrow` implementation for the enum
/// - `FromArrow` implementation for the enum
///
/// # Requirements
///
/// - The enum must be a unit enum (no fields in variants)
/// - All variants must have explicit discriminant values
///
/// # Attributes
///
/// - `#[type_hint(...)]`: Currently accepted but not used. Reserved for future use.
/// - `#[enum_path(...)]`: Currently accepted but not used. Reserved for future use.
///
/// # Example
///
/// ```ignore
/// use polars_structpath::EnumPath;
/// use polars_structpath_types::{IntoArrow, ArrowBuffer};
///
/// #[derive(EnumPath)]
/// enum Status {
///     Active = 1,
///     Inactive = 2,
/// }
///
/// // Now Status implements IntoArrow and FromArrow
/// let mut buffer = Status::new_buffer(2);
/// buffer.push(Status::Active);
/// buffer.push(Status::Inactive);
/// let array = buffer.to_arrow().unwrap();
/// ```
#[proc_macro_derive(EnumPath, attributes(type_hint, enum_path))]
pub fn derive_enum_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_enum_path_impl(input).into()
}
