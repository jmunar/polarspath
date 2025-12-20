//! Module for type system wrappers and metadata.
//!
//! This module provides the core type system components that extend Polars' `DataType`
//! with Rust-specific information needed for path traversal and value extraction.
//! It includes `DataTypeWrapper`, `DataTypeOpt`, and related error types.

use indexmap::IndexMap;
use polars_core::prelude::{CategoricalMapping, DataType, Field, FrozenCategories};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{collections::HashMap, str::FromStr};

use thiserror::Error;

/// Error type for operations on `DataTypeWrapper` and path-based access.
///
/// This error type is used throughout the crate to represent various failure
/// conditions when working with type wrappers and path-based field access.
///
/// # Examples
///
/// ```rust
/// use polars_structpath_types::DataTypeWrapperError;
///
/// // Field not found error
/// let err = DataTypeWrapperError::FieldNotFound("missing_field".to_string());
/// assert!(err.to_string().contains("missing_field"));
///
/// // Invalid path error
/// let err = DataTypeWrapperError::InvalidPath("invalid.path".to_string());
/// assert!(err.to_string().contains("invalid.path"));
/// ```
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataTypeWrapperError {
    /// The provided path is invalid or cannot be resolved.
    ///
    /// This typically occurs when a path string cannot be parsed or when
    /// attempting to access a field that doesn't exist in the type structure.
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// A field with the given name was not found in the struct.
    ///
    /// The contained string is the name of the field that was not found.
    #[error("Field not found: {0}")]
    FieldNotFound(String),

    /// A type mismatch occurred during an operation.
    ///
    /// The `expected` field contains the expected type, and `actual` contains
    /// the type that was actually encountered.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    /// An array index was out of bounds.
    ///
    /// The contained `usize` is the invalid index that was accessed.
    #[error("Vector index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    /// Attempted to access a field of a null/None value.
    ///
    /// This occurs when trying to access a nested field through an `Option<T>`
    /// that is `None`.
    #[error("Cannot access field of null value")]
    NullValue,

    /// The requested functionality is not yet implemented.
    #[error("Functionality not yet implemented")]
    NotImplemented,

    /// Attempted to access a field on a type that is not a struct.
    ///
    /// This occurs when trying to use field access methods on a non-struct type.
    #[error("DataTypeOpt is not a struct")]
    NotAStruct,
}

/// Metadata for enum types used in Polars categorical mapping.
///
/// This struct contains the information needed to map between Rust enum variants
/// and their Polars categorical representation. It stores the category names in
/// the order expected by Polars, and a mapping from Rust enum discriminant values
/// to Polars categorical indices.
///
/// # Fields
///
/// * `categories` - A vector of category names in the order used by Polars
/// * `rust_index_to_polars_index` - A mapping from Rust enum discriminant values
///   (as `u32`) to Polars categorical indices (as `u32`)
///
/// # Example
///
/// ```rust,no_run
/// use polars_structpath_types::EnumOptInfo;
///
/// // For an enum like:
/// // enum Status { Active = 0, Inactive = 1 }
/// //
/// // The EnumOptInfo would contain:
/// // - categories: vec!["Active".to_string(), "Inactive".to_string()]
/// // - rust_index_to_polars_index: {0 => 0, 1 => 1}
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumOptInfo {
    /// Vector of category names, in the order expected by Polars.
    pub categories: Vec<String>,
    /// Mapping from Rust enum discriminant values to Polars categorical indices.
    pub rust_index_to_polars_index: HashMap<u32, u32>,
}

impl<'a> FromIterator<(&'a str, u32)> for EnumOptInfo {
    fn from_iter<I>(category_to_rust_index: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, u32)>,
    {
        let items: Vec<_> = category_to_rust_index.into_iter().collect();
        let categories: Vec<String> = items
            .iter()
            .map(|(category, _)| category.to_string())
            .collect();
        let rust_index_to_polars_index: HashMap<u32, u32> = items
            .iter()
            .enumerate()
            .map(|(polars_index, (_, rust_index))| (*rust_index, polars_index as u32))
            .collect();
        EnumOptInfo {
            categories,
            rust_index_to_polars_index,
        }
    }
}

/// Internal representation of data types with Rust-specific extensions.
///
/// This enum represents all supported data types, including both standard Polars
/// types and Rust-specific types like `Option<T>` that need special handling.
///
/// # Type Variants
///
/// ## Scalar Types
///
/// - `String` - UTF-8 string type
/// - `Bytes` - Binary/byte array type
/// - `Int32`, `Int64` - Signed integer types
/// - `UInt32`, `UInt64` - Unsigned integer types
/// - `Float32`, `Float64` - Floating-point types
/// - `Boolean` - Boolean type
///
/// ## Composite Types
///
/// - `Enum(EnumOptInfo)` - Enum type with categorical mapping information
/// - `List(Box<DataTypeWrapper>)` - List/array type containing another type
/// - `Struct(IndexMap<String, DataTypeWrapper>)` - Struct type with named fields
/// - `Option(Box<DataTypeWrapper>)` - Optional type (Rust-specific)
///
/// ## Future Types
///
/// - `StructFuture(&'static str)` - Placeholder for struct types known only at runtime
/// - `EnumFuture(&'static str)` - Placeholder for enum types known only at runtime
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::{DataTypeOpt, DataTypeWrapper};
///
/// // Create a simple type
/// let string_type = DataTypeWrapper::new(DataTypeOpt::String);
///
/// // Create a list type
/// let list_type = DataTypeWrapper::new(DataTypeOpt::List(
///     Box::new(DataTypeWrapper::new(DataTypeOpt::String))
/// ));
///
/// // Create an optional type
/// let opt_type = DataTypeWrapper::new(DataTypeOpt::Option(
///     Box::new(DataTypeWrapper::new(DataTypeOpt::Int64))
/// ));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTypeOpt {
    /// UTF-8 string type.
    String,
    /// Binary/byte array type.
    Bytes,
    /// 32-bit signed integer type.
    Int32,
    /// 64-bit signed integer type.
    Int64,
    /// 32-bit unsigned integer type.
    UInt32,
    /// 64-bit unsigned integer type.
    UInt64,
    /// 32-bit floating-point type.
    Float32,
    /// 64-bit floating-point type.
    Float64,
    /// Boolean type.
    Boolean,
    /// Enum type with categorical mapping information.
    Enum(EnumOptInfo),
    /// List/array type containing another type.
    List(Box<DataTypeWrapper>),
    /// Struct type with named fields.
    ///
    /// The `IndexMap` preserves field order and maps field names to their types.
    Struct(IndexMap<String, DataTypeWrapper>),
    /// Optional type (Rust `Option<T>`).
    ///
    /// This allows distinguishing between required and optional fields,
    /// which is important for proper value extraction from nested structures.
    Option(Box<DataTypeWrapper>),
    /// Placeholder for struct types whose full type is only known at runtime.
    ///
    /// The contained string is the type name that will be resolved later.
    StructFuture(&'static str),
    /// Placeholder for enum types whose full type is only known at runtime.
    ///
    /// The contained string is the type name that will be resolved later.
    EnumFuture(&'static str),
}

/// A wrapper around Polars `DataType` with additional metadata for path traversal.
///
/// This type extends Polars' `DataType` with Rust-specific information needed for
/// path-based field access. It maintains both an internal representation (`raw`)
/// and the corresponding Polars `DataType` (`polars`).
///
/// # Fields
///
/// * `raw` - The internal `DataTypeOpt` representation with Rust-specific types
/// * `polars` - The corresponding Polars `DataType` for DataFrame operations
///
/// # Purpose
///
/// The wrapper is necessary because:
///
/// 1. Polars `DataType` doesn't distinguish between required and optional fields
/// 2. Enum mappings need to be preserved for proper conversion
/// 3. Path traversal requires knowledge of the full type structure
///
/// # Example
///
/// ```rust
/// use polars_core::prelude::DataType;
/// use polars_structpath_types::{DataTypeOpt, DataTypeWrapper};
///
/// // Create a string type wrapper
/// let string_wrapper = DataTypeWrapper::new(DataTypeOpt::String);
/// assert_eq!(string_wrapper.polars, DataType::String);
///
/// // Create an optional type
/// let opt_wrapper = DataTypeWrapper::new(DataTypeOpt::Option(
///     Box::new(DataTypeWrapper::new(DataTypeOpt::Int64))
/// ));
/// assert_eq!(opt_wrapper.polars, DataType::Int64);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTypeWrapper {
    /// The internal type representation with Rust-specific extensions.
    pub raw: DataTypeOpt,
    /// The corresponding Polars `DataType` for DataFrame operations.
    pub polars: DataType,
}

impl DataTypeWrapper {
    /// Creates a new `DataTypeWrapper` from a `DataTypeOpt`.
    ///
    /// This method automatically converts the internal `DataTypeOpt` representation
    /// to the corresponding Polars `DataType`. For complex types like enums and
    /// structs, it performs the necessary conversions and mappings.
    ///
    /// # Arguments
    ///
    /// * `raw` - The internal type representation
    ///
    /// # Returns
    ///
    /// A new `DataTypeWrapper` with both `raw` and `polars` fields populated.
    ///
    /// # Example
    ///
    /// ```rust
    /// use polars_core::prelude::DataType;
    /// use polars_structpath_types::{DataTypeOpt, DataTypeWrapper};
    ///
    /// let wrapper = DataTypeWrapper::new(DataTypeOpt::String);
    /// assert_eq!(wrapper.polars, DataType::String);
    /// ```
    pub fn new(raw: DataTypeOpt) -> Self {
        let polars = match &raw {
            DataTypeOpt::String => DataType::String,
            DataTypeOpt::Bytes => DataType::Binary,
            DataTypeOpt::Int32 => DataType::Int32,
            DataTypeOpt::Int64 => DataType::Int64,
            DataTypeOpt::UInt32 => DataType::UInt32,
            DataTypeOpt::UInt64 => DataType::UInt64,
            DataTypeOpt::Float32 => DataType::Float32,
            DataTypeOpt::Float64 => DataType::Float64,
            DataTypeOpt::Boolean => DataType::Boolean,
            DataTypeOpt::Enum(enum_mapping) => {
                let categories =
                    FrozenCategories::new(enum_mapping.categories.iter().map(|s| s.as_str()))
                        .unwrap();
                let mapping = CategoricalMapping::new(enum_mapping.categories.len());
                enum_mapping.categories.iter().for_each(|s| {
                    let _ = mapping.insert_cat(s).unwrap();
                });
                DataType::Enum(categories, std::sync::Arc::new(mapping))
            }
            DataTypeOpt::List(inner_type) => DataType::List(Box::new(inner_type.polars.clone())),
            DataTypeOpt::Struct(fields) => DataType::Struct(
                fields
                    .iter()
                    .map(|(field_name, field_type)| {
                        Field::new(field_name.into(), field_type.polars.clone())
                    })
                    .collect(),
            ),
            DataTypeOpt::Option(inner_type) => inner_type.polars.clone(),
            DataTypeOpt::StructFuture(_) => DataType::Null,
            DataTypeOpt::EnumFuture(_) => DataType::Null,
        };
        Self { raw, polars }
    }
}

impl ToTokens for DataTypeWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &self.raw {
            DataTypeOpt::String => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::String) },
            DataTypeOpt::Bytes => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Bytes) },
            DataTypeOpt::Int32 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Int32) },
            DataTypeOpt::Int64 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Int64) },
            DataTypeOpt::UInt32 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::UInt32) },
            DataTypeOpt::UInt64 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::UInt64) },
            DataTypeOpt::Float32 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Float32) },
            DataTypeOpt::Float64 => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Float64) },
            DataTypeOpt::Boolean => quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Boolean) },
            DataTypeOpt::List(inner_type) => {
                quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::List(Box::new(#inner_type))) }
            }
            DataTypeOpt::Option(inner_type) => {
                quote! { ::polars_structpath::DataTypeWrapper::new(::polars_structpath::DataTypeOpt::Option(Box::new(#inner_type))) }
            }
            DataTypeOpt::StructFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    <#inner_type as ::polars_structpath::HasDataTypeWrapper>::data_type_wrapper().clone()
                }
            }
            DataTypeOpt::EnumFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    <#inner_type as ::polars_structpath::HasDataTypeWrapper>::data_type_wrapper().clone()
                }
            }
            _ => panic!("Unsupported to_tokens method for DataTypeWrapper: {:?}", self),
        })
    }
}

/// Trait for types that have an associated `DataTypeWrapper`.
///
/// This trait provides access to the type metadata needed for path-based field access
/// and Polars DataFrame integration. It's the foundation trait that both `StructPath`
/// and `EnumPath` depend on.
///
/// # Implementation Requirements
///
/// The `data_type_wrapper()` method must be implemented with per-type static storage
/// to avoid sharing cached values across different types. This is typically handled
/// automatically by derive macros.
///
/// # Example
///
/// ```rust,no_run
/// use polars_core::prelude::DataType;
/// use polars_structpath_types::{HasDataTypeWrapper, DataTypeWrapper, DataTypeOpt};
///
/// // This would typically be generated by a derive macro
/// struct MyStruct {
///     name: String,
/// }
///
/// // After deriving, you can access:
/// // let wrapper = MyStruct::data_type_wrapper();
/// // let polars_type = MyStruct::data_type();
/// ```
pub trait HasDataTypeWrapper {
    /// Returns the `DataTypeWrapper` representation of the type.
    ///
    /// This method must be implemented with per-type static storage to avoid
    /// sharing cached values across types. The returned reference should be
    /// to a static value that is unique to this specific type.
    ///
    /// # Returns
    ///
    /// A reference to the static `DataTypeWrapper` for this type.
    fn data_type_wrapper() -> &'static DataTypeWrapper;

    /// Returns the Polars `DataType` representation of this type.
    ///
    /// This is a convenience method that extracts the `polars` field from
    /// the `DataTypeWrapper`. It's useful when you only need the Polars type
    /// and not the full wrapper with Rust-specific metadata.
    ///
    /// # Returns
    ///
    /// A reference to the static Polars `DataType` for this type.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_core::prelude::DataType;
    /// use polars_structpath_types::HasDataTypeWrapper;
    ///
    /// // After deriving HasDataTypeWrapper:
    /// // let dtype = MyStruct::data_type();
    /// // assert_eq!(dtype, &DataType::Struct(...));
    /// ```
    fn data_type() -> &'static DataType {
        &Self::data_type_wrapper().polars
    }
}
