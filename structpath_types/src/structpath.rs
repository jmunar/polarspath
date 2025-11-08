use crate::{
    DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, HasDataTypeWrapper, Path, PathComponent,
};
use indexmap::IndexMap;
use polars_core::prelude::{AnyValue, DataType, Field};

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

/// This trait is used to define the methods that are available on a Rust struct
/// with nested fields being accessible using a path.
///
/// The end user methods are:
/// - `fields`
/// - `get_type`
/// - `get_value`
///
/// Note that the `get` methods only throw an error if the path is invalid,
/// not if a parent of the field is null or the array item is out of bounds.
///
pub trait StructPath: HasDataTypeWrapper {
    fn fields_opt() -> &'static IndexMap<String, DataTypeWrapper> {
        match &Self::data_type_wrapper().raw {
            DataTypeOpt::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    fn fields() -> &'static Vec<Field> {
        match &Self::data_type_wrapper().polars {
            DataType::Struct(fields) => fields,
            _ => panic!("StructPath should have a DataTypeWrapper::Struct type"),
        }
    }

    fn get_type_by_path(path: &Path) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        get_type_by_path(Self::data_type_wrapper(), path)
    }

    fn get_type(path: &str) -> Result<DataTypeWrapper, DataTypeWrapperError> {
        get_type(Self::data_type_wrapper(), path)
    }

    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue<'_>, DataTypeWrapperError>;

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
