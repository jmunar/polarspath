use indexmap::IndexMap;
use polars_core::prelude::{CategoricalMapping, DataType, Field, FrozenCategories};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{collections::HashMap, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataTypeWrapperError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Vector index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    #[error("Cannot access field of null value")]
    NullValue,

    #[error("Functionality not yet implemented")]
    NotImplemented,

    #[error("DataTypeOpt is not a struct")]
    NotAStruct,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumOptInfo {
    // Vector of categories, in the order of polars
    pub categories: Vec<String>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTypeOpt {
    // Types supported from DataType
    String,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Boolean,
    Enum(EnumOptInfo),
    List(Box<DataTypeWrapper>),
    Struct(IndexMap<String, DataTypeWrapper>),
    // We need to add option to be able to extract the value from the
    // structure in a different way than required fields
    Option(Box<DataTypeWrapper>),
    // Special type for structs with full type only known at runtime
    StructFuture(&'static str),
    EnumFuture(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTypeWrapper {
    pub raw: DataTypeOpt,
    pub polars: DataType,
}

impl DataTypeWrapper {
    pub fn new(raw: DataTypeOpt) -> Self {
        let polars = match &raw {
            DataTypeOpt::String => DataType::String,
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
            DataTypeOpt::String => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::String) },
            DataTypeOpt::Int32 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Int32) },
            DataTypeOpt::Int64 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Int64) },
            DataTypeOpt::UInt32 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::UInt32) },
            DataTypeOpt::UInt64 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::UInt64) },
            DataTypeOpt::Float32 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Float32) },
            DataTypeOpt::Float64 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Float64) },
            DataTypeOpt::Boolean => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Boolean) },
            DataTypeOpt::List(inner_type) => {
                quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::List(Box::new(#inner_type))) }
            }
            DataTypeOpt::Option(inner_type) => {
                quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Option(Box::new(#inner_type))) }
            }
            DataTypeOpt::StructFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    <#inner_type as ::structpath::HasDataTypeWrapper>::data_type_wrapper().clone()
                }
            }
            DataTypeOpt::EnumFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    <#inner_type as ::structpath::HasDataTypeWrapper>::data_type_wrapper().clone()
                }
            }
            _ => panic!("Unsupported to_tokens method for DataTypeWrapper: {:?}", self),
        })
    }
}

pub trait HasDataTypeWrapper {
    /// Returns the DataTypeWrapper representation of the type.
    /// Must be implemented with per-type static storage to avoid sharing cached values across types.
    fn data_type_wrapper() -> &'static DataTypeWrapper;

    /// Returns the polars DataType representation of this struct.
    fn data_type() -> &'static DataType {
        &Self::data_type_wrapper().polars
    }
}
