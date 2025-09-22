use crate::path::{Path, PathComponent};

use indexmap::IndexMap;
use polars_core::prelude::{DataType, Field};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataTypeOptError {
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
    List(Box<DataTypeOpt>),
    Struct(IndexMap<String, DataTypeOpt>),
    Object(&'static str),
    // We need to add option to be able to extract the value from the
    // structure in a different way than required fields
    Option(Box<DataTypeOpt>),
    // Special type for structs with full type only known at runtime
    StructFuture(&'static str),
}

impl DataTypeOpt {
    pub fn to_data_type(&self) -> DataType {
        match self {
            DataTypeOpt::String => DataType::String,
            DataTypeOpt::Int32 => DataType::Int32,
            DataTypeOpt::Int64 => DataType::Int64,
            DataTypeOpt::Float64 => DataType::Float64,
            DataTypeOpt::Boolean => DataType::Boolean,
            DataTypeOpt::List(inner_type) => DataType::List(Box::new(inner_type.to_data_type())),
            DataTypeOpt::Struct(fields) => DataType::Struct(
                fields
                    .iter()
                    .map(|(field_name, field_type)| {
                        Field::new(field_name.into(), field_type.to_data_type())
                    })
                    .collect(),
            ),
            DataTypeOpt::Object(objname) => DataType::Object(objname),
            DataTypeOpt::Option(inner_type) => inner_type.to_data_type(),
            DataTypeOpt::StructFuture(_) => panic!("StructFuture shouldn't be used at runtime"),
        }
    }

    pub fn field_type(&self, field_name: &str) -> Result<DataTypeOpt, DataTypeOptError> {
        match self {
            DataTypeOpt::Option(t) => {
                let inner_type = t.field_type(field_name)?;
                Ok(DataTypeOpt::Option(Box::new(inner_type)))
            }
            DataTypeOpt::Struct(fields) => match fields.get(field_name) {
                Some(field_type) => Ok(field_type.clone()),
                None => Err(DataTypeOptError::FieldNotFound(field_name.to_string())),
            },
            _ => Err(DataTypeOptError::NotAStruct),
        }
    }

    pub fn get_type_by_path(&self, path: &Path) -> Result<DataTypeOpt, DataTypeOptError> {
        let path_component = path.components[0].clone();

        if path.components.len() > 1 {
            let remaining_path = Path {
                components: path.components[1..].to_vec(),
            };
            return match path_component {
                PathComponent::Field(field) => {
                    let field_type = self.field_type(&field)?;
                    match field_type {
                        // Struct
                        DataTypeOpt::Struct(_) => field_type.get_type_by_path(&remaining_path),
                        // Option(Struct)
                        DataTypeOpt::Option(t) if matches!(*t, DataTypeOpt::Struct(_)) => t
                            .get_type_by_path(&remaining_path)
                            .map(|type_opt| DataTypeOpt::Option(Box::new(type_opt))),
                        _ => Err(DataTypeOptError::InvalidPath(field)),
                    }
                }
                PathComponent::ArrayIndex(field, _) => {
                    let field_type = self.field_type(&field)?;
                    match field_type {
                        // List(Struct) or List(Option(Struct))
                        DataTypeOpt::List(t) => t.get_type_by_path(&remaining_path),
                        // Option(List(Struct)) or Option(List(Option(Struct)))
                        DataTypeOpt::Option(t0) if matches!(*t0, DataTypeOpt::List(_)) => {
                            if let DataTypeOpt::List(ref t) = *t0 {
                                t.get_type_by_path(&remaining_path)
                                    .map(|type_opt| DataTypeOpt::Option(Box::new(type_opt)))
                            } else {
                                unreachable!()
                            }
                        }
                        _ => Err(DataTypeOptError::InvalidPath(field)),
                    }
                }
            };
        }

        match path_component {
            PathComponent::Field(field) => {
                let field_type = self.field_type(&field)?;
                Ok(field_type.clone())
            }
            PathComponent::ArrayIndex(field, _) => {
                let field_type = self.field_type(&field)?;
                match field_type {
                    DataTypeOpt::List(t) => Ok(*t.clone()),
                    DataTypeOpt::Option(t) => t
                        .get_type_by_path(path)
                        .map(|type_opt| DataTypeOpt::Option(Box::new(type_opt))),
                    _ => Err(DataTypeOptError::InvalidPath(field)),
                }
            }
        }
    }

    pub fn get_type(&self, path: &str) -> Result<DataTypeOpt, DataTypeOptError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_type_by_path(&path),
            Err(e) => Err(DataTypeOptError::InvalidPath(e.to_string())),
        }
    }
}

impl ToTokens for DataTypeOpt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            DataTypeOpt::String => quote! { ::structpath::DataTypeOpt::String },
            DataTypeOpt::Int32 => quote! { ::structpath::DataTypeOpt::Int32 },
            DataTypeOpt::Int64 => quote! { ::structpath::DataTypeOpt::Int64 },
            DataTypeOpt::Float64 => quote! { ::structpath::DataTypeOpt::Float64 },
            DataTypeOpt::Boolean => quote! { ::structpath::DataTypeOpt::Boolean },
            DataTypeOpt::List(inner_type) => {
                quote! { ::structpath::DataTypeOpt::List(Box::new(#inner_type)) }
            }
            DataTypeOpt::Object(_) => {
                // let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! { ::structpath::DataTypeOpt::String }
            }
            DataTypeOpt::StructFuture(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok().unwrap();
                quote! {
                    #inner_type::data_type_opt().clone()
                }
            }
            DataTypeOpt::Option(inner_type) => {
                quote! { ::structpath::DataTypeOpt::Option(Box::new(#inner_type)) }
            }
            _ => panic!("Unsupported to_tokens method for DataTypeOpt: {:?}", self),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_type_opt_field_type_ok() {
        let data_type_opt =
            DataTypeOpt::Struct(IndexMap::from([("field1".into(), DataTypeOpt::String)]));
        assert_eq!(data_type_opt.field_type("field1"), Ok(DataTypeOpt::String));
    }

    #[test]
    fn data_type_opt_field_type_ok_with_nested_struct() {
        let data_type_opt = DataTypeOpt::Struct(IndexMap::from([(
            "field1".into(),
            DataTypeOpt::Struct(IndexMap::from([("field2".into(), DataTypeOpt::String)])),
        )]));
        assert_eq!(
            data_type_opt.field_type("field1"),
            Ok(DataTypeOpt::Struct(IndexMap::from([(
                "field2".into(),
                DataTypeOpt::String
            )])))
        );
    }

    #[test]
    fn data_type_opt_field_type_is_not_struct() {
        let data_type_opt = DataTypeOpt::String;
        assert_eq!(
            data_type_opt.field_type("field1"),
            Err(DataTypeOptError::NotAStruct)
        );
    }

    #[test]
    fn data_type_opt_field_type_field_not_found() {
        let data_type_opt =
            DataTypeOpt::Struct(IndexMap::from([("field1".into(), DataTypeOpt::String)]));
        assert_eq!(
            data_type_opt.field_type("field2"),
            Err(DataTypeOptError::FieldNotFound("field2".to_string()))
        );
    }

    #[test]
    fn data_type_opt_get_type_ok() {
        // Create a complex Struct data type
        let data_type_opt = DataTypeOpt::Struct(IndexMap::from([
            ("req_str".into(), DataTypeOpt::String),
            ("req_int".into(), DataTypeOpt::Int64),
            (
                "req_struct".into(),
                DataTypeOpt::Struct(IndexMap::from([("req_str".into(), DataTypeOpt::String)])),
            ),
            (
                "req_list_of_str".into(),
                DataTypeOpt::List(Box::new(DataTypeOpt::String)),
            ),
            (
                "req_list_of_struct".into(),
                DataTypeOpt::List(Box::new(DataTypeOpt::Struct(IndexMap::from([(
                    "field6".into(),
                    DataTypeOpt::String,
                )])))),
            ),
        ]));

        assert_eq!(data_type_opt.get_type("req_str"), Ok(DataTypeOpt::String));
        assert_eq!(data_type_opt.get_type("req_int"), Ok(DataTypeOpt::Int64));
        assert_eq!(
            data_type_opt.get_type("req_struct"),
            Ok(DataTypeOpt::Struct(IndexMap::from([(
                "req_str".into(),
                DataTypeOpt::String
            )])))
        );
        assert_eq!(
            data_type_opt.get_type("req_list_of_str"),
            Ok(DataTypeOpt::List(Box::new(DataTypeOpt::String)))
        );
        assert_eq!(
            data_type_opt.get_type("req_list_of_struct"),
            Ok(DataTypeOpt::List(Box::new(DataTypeOpt::Struct(
                IndexMap::from([("field6".into(), DataTypeOpt::String)])
            ))))
        );
        assert_eq!(
            data_type_opt.get_type("req_struct.req_str"),
            Ok(DataTypeOpt::String)
        );
    }
}
