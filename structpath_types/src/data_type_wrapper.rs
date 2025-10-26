use crate::path::{Path, PathComponent};

use indexmap::IndexMap;
use polars_core::prelude::{DataType, Field};
use polars_dtype::categorical::{CategoricalMapping, FrozenCategories};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;

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
pub enum DataTypeOpt {
    // Types supported from DataType
    String,
    Int32,
    Int64,
    Float64,
    Boolean,
    Enum(IndexMap<String, u32>),
    List(Box<DataTypeWrapper>),
    Struct(IndexMap<String, DataTypeWrapper>),
    // We need to add option to be able to extract the value from the
    // structure in a different way than required fields
    Option(Box<DataTypeWrapper>),
    // Special type for structs with full type only known at runtime
    StructFuture(&'static str),
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
            DataTypeOpt::Float64 => DataType::Float64,
            DataTypeOpt::Boolean => DataType::Boolean,
            DataTypeOpt::Enum(enum_values) => {
                let categories =
                    FrozenCategories::new(enum_values.keys().map(|s| s.as_str())).unwrap();
                let mapping = CategoricalMapping::new(enum_values.len());
                enum_values.keys().for_each(|s| {
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
        };
        Self { raw, polars }
    }

    pub fn field_type(&self, field_name: &str) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        match &self.raw {
            DataTypeOpt::Option(t) => {
                let inner_type = t.field_type(field_name)?;
                Ok(DataTypeWrapper::new(DataTypeOpt::Option(Box::new(
                    inner_type,
                ))))
            }
            DataTypeOpt::Struct(fields) => match fields.get(field_name) {
                Some(field_type) => Ok(field_type.clone()),
                None => Err(DataTypeWrapperError::FieldNotFound(field_name.to_string())),
            },
            _ => Err(DataTypeWrapperError::NotAStruct),
        }
    }

    pub fn get_type_by_path(&self, path: &Path) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        let path_component = path.components[0].clone();

        if path.components.len() > 1 {
            let remaining_path = Path {
                components: path.components[1..].to_vec(),
            };
            return match path_component {
                PathComponent::Field(field) => {
                    let field_type = self.field_type(&field)?;
                    match &field_type.raw {
                        // Struct
                        DataTypeOpt::Struct(_) => field_type.get_type_by_path(&remaining_path),
                        // Option(Struct)
                        DataTypeOpt::Option(t) if matches!(t.raw, DataTypeOpt::Struct(_)) => {
                            t.get_type_by_path(&remaining_path).map(|type_opt| {
                                DataTypeWrapper::new(DataTypeOpt::Option(Box::new(type_opt)))
                            })
                        }
                        _ => Err(DataTypeWrapperError::InvalidPath(field)),
                    }
                }
                PathComponent::ArrayIndex(field, _) => {
                    let field_type = self.field_type(&field)?;
                    match &field_type.raw {
                        // List(Struct) or List(Option(Struct))
                        DataTypeOpt::List(t) => t.get_type_by_path(&remaining_path),
                        // Option(List(Struct)) or Option(List(Option(Struct)))
                        DataTypeOpt::Option(t0) if matches!(t0.raw, DataTypeOpt::List(_)) => {
                            if let DataTypeOpt::List(ref t) = t0.raw {
                                t.get_type_by_path(&remaining_path).map(|type_opt| {
                                    DataTypeWrapper::new(DataTypeOpt::Option(Box::new(type_opt)))
                                })
                            } else {
                                unreachable!()
                            }
                        }
                        _ => Err(DataTypeWrapperError::InvalidPath(field)),
                    }
                }
            };
        }

        match path_component.clone() {
            PathComponent::Field(field) => {
                let field_type = self.field_type(&field)?;
                Ok(field_type.clone())
            }
            PathComponent::ArrayIndex(field, _) => {
                let field_type = self.field_type(&field)?;
                match &field_type.raw {
                    DataTypeOpt::List(t) => Ok(*t.clone()),
                    DataTypeOpt::Option(midt) if matches!(midt.raw, DataTypeOpt::List(_)) => {
                        if let DataTypeOpt::List(ref t) = midt.raw {
                            Ok(DataTypeWrapper::new(DataTypeOpt::Option(Box::new(
                                *t.clone(),
                            ))))
                        } else {
                            unreachable!()
                        }
                    }
                    _ => Err(DataTypeWrapperError::InvalidPath(field)),
                }
            }
        }
    }

    pub fn get_type(&self, path: &str) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_type_by_path(&path),
            Err(e) => Err(DataTypeWrapperError::InvalidPath(e.to_string())),
        }
    }
}

impl ToTokens for DataTypeWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &self.raw {
            DataTypeOpt::String => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::String) },
            DataTypeOpt::Int32 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Int32) },
            DataTypeOpt::Int64 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Int64) },
            DataTypeOpt::Float64 => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Float64) },
            DataTypeOpt::Boolean => quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Boolean) },
            DataTypeOpt::Enum(enum_values) => {
                // Convert inner_type to tokens
                let enum_values = enum_values
                    .iter()
                    .map(|(name, value)| {
                        quote! { (#name.into(), #value) }
                    })
                    .collect::<Vec<_>>();
                quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Enum(::structpath::indexmap::IndexMap::from([#(#enum_values),*]))) }
            }
            DataTypeOpt::List(inner_type) => {
                quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::List(Box::new(#inner_type))) }
            }
            DataTypeOpt::StructFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    <#inner_type as ::structpath::HasDataTypeWrapper>::data_type_wrapper().clone()
                }
            }
            DataTypeOpt::Option(inner_type) => {
                quote! { ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Option(Box::new(#inner_type))) }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_type_wrapper;

    #[test]
    fn data_type_wrapper_field_type_ok() {
        let data_type_wrapper = data_type_wrapper!(Struct([("field1", String)]));
        assert_eq!(
            data_type_wrapper.field_type("field1"),
            Ok(data_type_wrapper!(String))
        );
    }

    #[test]
    fn data_type_wrapper_field_type_ok_with_enum() {
        let data_type_wrapper = data_type_wrapper!(Struct([("enum1", Enum([("ITEM1", 1)]))]));
        assert_eq!(
            data_type_wrapper.field_type("enum1"),
            Ok(data_type_wrapper!(Enum([("ITEM1", 1)])))
        );
    }

    #[test]
    fn data_type_wrapper_field_type_ok_with_nested_struct() {
        let data_type_wrapper =
            data_type_wrapper!(Struct([("field1", Struct([("field2", String)]))]));
        assert_eq!(
            data_type_wrapper.field_type("field1"),
            Ok(data_type_wrapper!(Struct([("field2", String)])))
        );
    }

    #[test]
    fn data_type_wrapper_field_type_is_not_struct() {
        let data_type_wrapper = data_type_wrapper!(String);
        assert_eq!(
            data_type_wrapper.field_type("field1"),
            Err(DataTypeWrapperError::NotAStruct)
        );
    }

    #[test]
    fn data_type_wrapper_field_type_field_not_found() {
        let data_type_wrapper = data_type_wrapper!(Struct([("field1", String)]));
        assert_eq!(
            data_type_wrapper.field_type("field2"),
            Err(DataTypeWrapperError::FieldNotFound("field2".to_string()))
        );
    }

    #[test]
    fn data_type_wrapper_get_type_ok() {
        // Create a complex Struct data type
        let data_type_wrapper = data_type_wrapper!(Struct([
            ("req_str", String),
            ("req_int", Int64),
            ("req_struct", Struct([("req_str", String)])),
            ("req_list_of_str", List(String)),
            ("req_list_of_struct", List(Struct([("field6", String)])))
        ]));

        assert_eq!(
            data_type_wrapper.get_type("req_str"),
            Ok(data_type_wrapper!(String))
        );
        assert_eq!(
            data_type_wrapper.get_type("req_int"),
            Ok(data_type_wrapper!(Int64))
        );
        assert_eq!(
            data_type_wrapper.get_type("req_struct"),
            Ok(data_type_wrapper!(Struct([("req_str", String)])))
        );
        assert_eq!(
            data_type_wrapper.get_type("req_list_of_str"),
            Ok(data_type_wrapper!(List(String)))
        );
        assert_eq!(
            data_type_wrapper.get_type("req_list_of_struct"),
            Ok(data_type_wrapper!(List(Struct([("field6", String)]))))
        );
        assert_eq!(
            data_type_wrapper.get_type("req_struct.req_str"),
            Ok(data_type_wrapper!(String))
        );
    }
}
