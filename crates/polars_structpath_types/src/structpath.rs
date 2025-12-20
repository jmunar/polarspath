//! Module providing the `StructPath` trait and path-based type/value access utilities.
//!
//! This module contains the core functionality for accessing nested struct fields
//! using path strings. It provides both the `StructPath` trait that types can
//! implement (typically via derive macros) and helper functions for type queries.

use crate::{
    DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, HasDataTypeWrapper, Path, PathComponent,
};
use indexmap::IndexMap;
use polars_core::prelude::{AnyValue, DataType, Field};

/// Gets the type of a field within a struct type.
///
/// This is a helper function that extracts the type of a named field from a
/// `DataTypeWrapper` that represents a struct type.
///
/// # Arguments
///
/// * `dtw` - The `DataTypeWrapper` representing the struct type
/// * `field_name` - The name of the field to query
///
/// # Returns
///
/// Returns `Ok(DataTypeWrapper)` if the field exists, or an error if:
/// - The type is not a struct (`DataTypeWrapperError::NotAStruct`)
/// - The field doesn't exist (`DataTypeWrapperError::FieldNotFound`)
///
/// # Example
///
/// ```ignore
/// use polars_structpath_types::{DataTypeWrapper, DataTypeOpt};
/// use indexmap::IndexMap;
///
/// // Create a struct type with fields
/// let mut fields = IndexMap::new();
/// fields.insert("name".to_string(), DataTypeWrapper::new(DataTypeOpt::String));
/// fields.insert("age".to_string(), DataTypeWrapper::new(DataTypeOpt::Int64));
/// let struct_type = DataTypeWrapper::new(DataTypeOpt::Struct(fields));
///
/// // Get a field's type (this is an internal helper function)
/// // let name_type = field_type(&struct_type, "name").unwrap();
/// ```
pub fn field_type(
    dtw: &DataTypeWrapper,
    field_name: &str,
) -> Result<DataTypeWrapper, DataTypeWrapperError> {
    match &dtw.raw {
        DataTypeOpt::Option(t) => {
            let inner_type = field_type(t, field_name)?;
            Ok(DataTypeWrapper::new(DataTypeOpt::Option(Box::new(
                inner_type,
            ))))
        }
        DataTypeOpt::Struct(fields) => match fields.get(field_name) {
            Some(data_type_wrapper) => Ok(data_type_wrapper.clone()),
            None => Err(DataTypeWrapperError::FieldNotFound(field_name.to_string())),
        },
        _ => Err(DataTypeWrapperError::NotAStruct),
    }
}

/// Gets the type of a field by following a parsed path.
///
/// This function traverses a `Path` through nested structures to determine
/// the type of the field at the end of the path. It handles nested structs,
/// arrays, and optional types.
///
/// # Arguments
///
/// * `dtw` - The `DataTypeWrapper` representing the root type
/// * `path` - A parsed `Path` object representing the field to query
///
/// # Returns
///
/// Returns `Ok(DataTypeWrapper)` if the path is valid, or an error if:
/// - The path is invalid (`DataTypeWrapperError::InvalidPath`)
/// - A field doesn't exist (`DataTypeWrapperError::FieldNotFound`)
///
/// # Example
///
/// ```ignore
/// use polars_structpath_types::{DataTypeWrapper, DataTypeOpt, Path};
/// use indexmap::IndexMap;
///
/// // Create a nested struct type
/// let mut parent_fields = IndexMap::new();
/// parent_fields.insert("name".to_string(), DataTypeWrapper::new(DataTypeOpt::String));
/// let parent_type = DataTypeWrapper::new(DataTypeOpt::Struct(parent_fields));
///
/// let mut root_fields = IndexMap::new();
/// root_fields.insert("parent".to_string(), parent_type);
/// let root_type = DataTypeWrapper::new(DataTypeOpt::Struct(root_fields));
///
/// // Parse a path
/// let path = Path::from_str("parent.name").unwrap();
///
/// // Get the type at the end of the path (this is an internal helper function)
/// // let field_type = get_type_by_path(&root_type, &path).unwrap();
/// ```
pub fn get_type_by_path(
    dtw: &DataTypeWrapper,
    path: &Path,
) -> Result<DataTypeWrapper, DataTypeWrapperError> {
    let path_component = path.components[0].clone();

    if path.components.len() > 1 {
        let remaining_path = Path {
            components: path.components[1..].to_vec(),
        };
        return match path_component {
            PathComponent::Field(field) => {
                let data_type_wrapper = field_type(dtw, &field)?;
                match &data_type_wrapper.raw {
                    // Struct
                    DataTypeOpt::Struct(_) => get_type_by_path(&data_type_wrapper, &remaining_path),
                    // Option(Struct)
                    DataTypeOpt::Option(t) if matches!(t.raw, DataTypeOpt::Struct(_)) => {
                        get_type_by_path(t, &remaining_path).map(|type_opt| {
                            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(type_opt)))
                        })
                    }
                    _ => Err(DataTypeWrapperError::InvalidPath(field)),
                }
            }
            PathComponent::ArrayIndex(field, _) => {
                let data_type_wrapper = field_type(dtw, &field)?;
                match &data_type_wrapper.raw {
                    // List(Struct) or List(Option(Struct))
                    DataTypeOpt::List(t) => get_type_by_path(t, &remaining_path),
                    // Option(List(Struct)) or Option(List(Option(Struct)))
                    DataTypeOpt::Option(t0) if matches!(t0.raw, DataTypeOpt::List(_)) => {
                        if let DataTypeOpt::List(ref t) = t0.raw {
                            get_type_by_path(t, &remaining_path).map(|type_opt| {
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
            let data_type_wrapper = field_type(dtw, &field)?;
            Ok(data_type_wrapper.clone())
        }
        PathComponent::ArrayIndex(field, _) => {
            let data_type_wrapper = field_type(dtw, &field)?;
            match &data_type_wrapper.raw {
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

/// Gets the type of a field by a path string.
///
/// This is a convenience function that parses a path string and then calls
/// `get_type_by_path()`. It's the most common way to query field types.
///
/// # Arguments
///
/// * `dtw` - The `DataTypeWrapper` representing the root type
/// * `path` - A string path (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
///
/// # Returns
///
/// Returns `Ok(DataTypeWrapper)` if the path is valid, or an error if:
/// - The path cannot be parsed
/// - The path is invalid (`DataTypeWrapperError::InvalidPath`)
/// - A field doesn't exist (`DataTypeWrapperError::FieldNotFound`)
///
/// # Example
///
/// ```ignore
/// use polars_structpath_types::{DataTypeWrapper, DataTypeOpt};
/// use indexmap::IndexMap;
///
/// // Create a nested struct type
/// let mut parent_fields = IndexMap::new();
/// parent_fields.insert("name".to_string(), DataTypeWrapper::new(DataTypeOpt::String));
/// let parent_type = DataTypeWrapper::new(DataTypeOpt::Struct(parent_fields));
///
/// let mut root_fields = IndexMap::new();
/// root_fields.insert("parent".to_string(), parent_type);
/// let struct_type = DataTypeWrapper::new(DataTypeOpt::Struct(root_fields));
///
/// // Get a field's type using a string path (this is an internal helper function)
/// // let field_type = get_type(&struct_type, "parent.name").unwrap();
/// ```
pub fn get_type(
    dtw: &DataTypeWrapper,
    path: &str,
) -> Result<DataTypeWrapper, DataTypeWrapperError> {
    let path = Path::from_str(path);
    match path {
        Ok(path) => get_type_by_path(dtw, &path),
        Err(e) => Err(DataTypeWrapperError::InvalidPath(e.to_string())),
    }
}

/// Trait for types that support path-based field access.
///
/// This trait enables accessing nested struct fields using string paths, similar to
/// JSON path notation. It provides methods to query type information and extract
/// values from structs at runtime.
///
/// # Methods
///
/// The trait provides the following user-facing methods:
///
/// - [`fields()`](StructPath::fields) - Get the Polars field definitions
/// - [`fields_opt()`](StructPath::fields_opt) - Get the internal field type map
/// - [`get_type()`](StructPath::get_type) - Get the type of a field by path string
/// - [`get_type_by_path()`](StructPath::get_type_by_path) - Get the type of a field by parsed path
/// - [`get_value()`](StructPath::get_value) - Get the value of a field by path string
/// - [`get_value_by_path()`](StructPath::get_value_by_path) - Get the value of a field by parsed path
///
/// # Path Syntax
///
/// Paths support the following syntax:
///
/// - Simple fields: `"name"`
/// - Nested fields: `"parent.name"`
/// - Array indices: `"parents[0]"`
/// - Nested array access: `"parents[0].name"`
///
/// # Error Handling
///
/// The `get` methods only return an error if the path is invalid (e.g., field doesn't exist).
/// They do not error if a parent field is `None` or an array index is out of bounds -
/// these cases are handled gracefully by returning appropriate null values.
///
/// # Example
///
/// ```rust,no_run
/// use polars_core::prelude::{AnyValue, DataType};
/// use polars_structpath_types::StructPath;
///
/// // This would typically be generated by the StructPath derive macro
/// // from the polars_structpath_derive crate
/// struct User {
///     name: String,
///     age: i64,
///     parent: Option<Parent>,
/// }
///
/// struct Parent {
///     name: String,
/// }
///
/// // After deriving StructPath, you can use:
/// // let user = User { ... };
/// // let name_type = User::get_type("name").unwrap();
/// // let name_value = user.get_value("name").unwrap();
/// // let parent_name = user.get_value("parent.name").unwrap();
/// ```
///
/// # Implementation Note
///
/// This trait is typically implemented automatically by the `#[derive(StructPath)]` macro
/// from the `polars_structpath_derive` crate. Manual implementation is possible but not recommended.
pub trait StructPath: HasDataTypeWrapper {
    /// Returns the internal field type map for this struct.
    ///
    /// This provides access to the `IndexMap` containing field names and their
    /// corresponding `DataTypeWrapper` representations.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_structpath_types::StructPath;
    ///
    /// // After deriving StructPath on a struct:
    /// // let fields = MyStruct::fields_opt();
    /// // assert!(fields.contains_key("field_name"));
    /// ```
    fn fields_opt() -> &'static IndexMap<String, DataTypeWrapper> {
        match &Self::data_type_wrapper().raw {
            DataTypeOpt::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    /// Returns the Polars field definitions for this struct.
    ///
    /// This provides access to the `Vec<Field>` that can be used directly with
    /// Polars DataFrames and Series.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_core::prelude::DataType;
    /// use polars_structpath_types::StructPath;
    ///
    /// // After deriving StructPath on a struct:
    /// // let fields = MyStruct::fields();
    /// // let name_field = fields.iter().find(|f| f.name() == "name").unwrap();
    /// // assert_eq!(name_field.data_type(), &DataType::String);
    /// ```
    fn fields() -> &'static Vec<Field> {
        match &Self::data_type_wrapper().polars {
            DataType::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    /// Gets the type of a field by a parsed [`Path`].
    ///
    /// This method is useful when you need to parse a path once and query multiple
    /// types, or when working with programmatically constructed paths.
    ///
    /// # Arguments
    ///
    /// * `path` - A parsed `Path` object representing the field to query
    ///
    /// # Returns
    ///
    /// Returns `Ok(DataTypeWrapper)` if the path is valid, or an error if the path
    /// doesn't exist or is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_structpath_types::{Path, StructPath};
    ///
    /// // After deriving StructPath on a struct:
    /// // let path = Path::from_str("parent.name").unwrap();
    /// // let field_type = MyStruct::get_type_by_path(&path).unwrap();
    /// ```
    fn get_type_by_path(path: &Path) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        get_type_by_path(Self::data_type_wrapper(), path)
    }

    /// Gets the type of a field by a path string.
    ///
    /// This is the most convenient method for querying field types. The path string
    /// is parsed automatically.
    ///
    /// # Arguments
    ///
    /// * `path` - A string path (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
    ///
    /// # Returns
    ///
    /// Returns `Ok(DataTypeWrapper)` if the path is valid, or an error if the path
    /// doesn't exist or is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_structpath_types::StructPath;
    ///
    /// // After deriving StructPath on a struct:
    /// // let name_type = MyStruct::get_type("name").unwrap();
    /// // let nested_type = MyStruct::get_type("parent.name").unwrap();
    /// // let array_type = MyStruct::get_type("items[0]").unwrap();
    /// ```
    fn get_type(path: &str) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        get_type(Self::data_type_wrapper(), path)
    }

    /// Gets the value of a field by a parsed [`Path`].
    ///
    /// This method is useful when you need to parse a path once and extract multiple
    /// values, or when working with programmatically constructed paths.
    ///
    /// # Arguments
    ///
    /// * `path` - A parsed `Path` object representing the field to access
    ///
    /// # Returns
    ///
    /// Returns `Ok(AnyValue)` if the path is valid. Returns an error only if the path
    /// is invalid (e.g., field doesn't exist). If a parent field is `None` or an array
    /// index is out of bounds, returns `AnyValue::Null` rather than an error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_structpath_types::{Path, StructPath};
    ///
    /// // After deriving StructPath on a struct:
    /// // let instance = MyStruct { ... };
    /// // let path = Path::from_str("parent.name").unwrap();
    /// // let value = instance.get_value_by_path(&path).unwrap();
    /// ```
    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue<'_>, DataTypeWrapperError>;

    /// Gets the value of a field by a path string.
    ///
    /// This is the most convenient method for extracting field values. The path string
    /// is parsed automatically.
    ///
    /// # Arguments
    ///
    /// * `path` - A string path (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
    ///
    /// # Returns
    ///
    /// Returns `Ok(AnyValue)` if the path is valid. Returns an error only if the path
    /// is invalid (e.g., field doesn't exist). If a parent field is `None` or an array
    /// index is out of bounds, returns `AnyValue::Null` rather than an error.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use polars_core::prelude::AnyValue;
    /// use polars_structpath_types::StructPath;
    ///
    /// // After deriving StructPath on a struct:
    /// // let user = User {
    /// //     name: "John".to_string(),
    /// //     age: 32,
    /// //     parent: Some(Parent { name: "Mary".to_string() }),
    /// // };
    /// //
    /// // let name = user.get_value("name").unwrap();
    /// // assert_eq!(name, AnyValue::String("John"));
    /// //
    /// // let parent_name = user.get_value("parent.name").unwrap();
    /// // assert_eq!(parent_name, AnyValue::String("Mary"));
    /// ```
    fn get_value(&self, path: &str) -> Result<AnyValue<'_>, DataTypeWrapperError> {
        let path = Path::from_str(path);
        match path {
            Ok(path) => self.get_value_by_path(&path),
            Err(e) => Err(DataTypeWrapperError::InvalidPath(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_type_wrapper;

    #[test]
    fn field_type_ok() {
        let data_type_wrapper = data_type_wrapper!(Struct([("field1", String)]));
        assert_eq!(
            field_type(&data_type_wrapper, "field1"),
            Ok(data_type_wrapper!(String))
        );
    }

    #[test]
    fn field_type_ok_with_enum() {
        let data_type_wrapper = data_type_wrapper!(Struct([("enum1", Enum([("ITEM1", 1)]))]));
        assert_eq!(
            field_type(&data_type_wrapper, "enum1"),
            Ok(data_type_wrapper!(Enum([("ITEM1", 1)])))
        );
    }

    #[test]
    fn field_type_ok_with_nested_struct() {
        let data_type_wrapper =
            data_type_wrapper!(Struct([("field1", Struct([("field2", String)]))]));
        assert_eq!(
            field_type(&data_type_wrapper, "field1"),
            Ok(data_type_wrapper!(Struct([("field2", String)])))
        );
    }

    #[test]
    fn field_type_is_not_struct() {
        let data_type_wrapper = data_type_wrapper!(String);
        assert_eq!(
            field_type(&data_type_wrapper, "field1"),
            Err(DataTypeWrapperError::NotAStruct)
        );
    }

    #[test]
    fn field_type_field_not_found() {
        let data_type_wrapper = data_type_wrapper!(Struct([("field1", String)]));
        assert_eq!(
            field_type(&data_type_wrapper, "field2"),
            Err(DataTypeWrapperError::FieldNotFound("field2".to_string()))
        );
    }

    #[test]
    fn get_type_ok() {
        // Create a complex Struct data type
        let data_type_wrapper = data_type_wrapper!(Struct([
            ("req_str", String),
            ("req_int", Int64),
            ("req_struct", Struct([("req_str", String)])),
            ("req_list_of_str", List(String)),
            ("req_list_of_struct", List(Struct([("field6", String)])))
        ]));

        assert_eq!(
            get_type(&data_type_wrapper, "req_str"),
            Ok(data_type_wrapper!(String))
        );
        assert_eq!(
            get_type(&data_type_wrapper, "req_int"),
            Ok(data_type_wrapper!(Int64))
        );
        assert_eq!(
            get_type(&data_type_wrapper, "req_struct"),
            Ok(data_type_wrapper!(Struct([("req_str", String)])))
        );
        assert_eq!(
            get_type(&data_type_wrapper, "req_list_of_str"),
            Ok(data_type_wrapper!(List(String)))
        );
        assert_eq!(
            get_type(&data_type_wrapper, "req_list_of_struct"),
            Ok(data_type_wrapper!(List(Struct([("field6", String)]))))
        );
        assert_eq!(
            get_type(&data_type_wrapper, "req_struct.req_str"),
            Ok(data_type_wrapper!(String))
        );
    }
}
