use crate::sample::{SampleStruct, SampleSubstruct};

use polars_core::prelude::AnyValue;
use polars_structpath_types::{
    DataTypeOpt, DataTypeWrapperError, IntoAnyValueWith, Path, PathComponent, StructPath,
};

impl StructPath for SampleSubstruct {
    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue<'_>, DataTypeWrapperError> {
        let path_component = path.components[0].clone();
        match path_component {
            PathComponent::Field(name) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeWrapperError::FieldNotFound(name.to_string()))?;
                match name.as_str() {
                    "subf_string" => Ok(field_type.to_any_value(&self.subf_string)),
                    _ => Err(DataTypeWrapperError::FieldNotFound(name.to_string())),
                }
            }
            PathComponent::ArrayIndex(name, _) => {
                Err(DataTypeWrapperError::FieldNotFound(name.to_string()))
            }
        }
    }
}

impl StructPath for SampleStruct {
    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue<'_>, DataTypeWrapperError> {
        let path_component = path.components[0].clone();

        if path.components.len() > 1 {
            let remaining_path = Path {
                components: path.components[1..].to_vec(),
            };
            return match path_component {
                PathComponent::Field(field) => match field.as_str() {
                    "req_struct" => self.req_struct.get_value_by_path(&remaining_path),
                    "opt_struct" => match self.opt_struct {
                        Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                        None => Ok(AnyValue::Null),
                    },
                    _ => Err(DataTypeWrapperError::FieldNotFound(field)),
                },
                PathComponent::ArrayIndex(field, index) => match field.as_str() {
                    "req_vec_req_item_struct" => {
                        self.req_vec_req_item_struct[index].get_value_by_path(&remaining_path)
                    }
                    "opt_vec_req_item_struct" => match self.opt_vec_req_item_struct {
                        Some(ref vec) => vec[index].get_value_by_path(&remaining_path),
                        None => Ok(AnyValue::Null),
                    },
                    "req_vec_opt_item_struct" => match self.req_vec_opt_item_struct[index] {
                        Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_struct" => match self.opt_vec_opt_item_struct {
                        Some(ref vec) => match vec[index] {
                            Some(ref struct_value) => {
                                struct_value.get_value_by_path(&remaining_path)
                            }
                            None => Ok(AnyValue::Null),
                        },
                        None => Ok(AnyValue::Null),
                    },
                    _ => Err(DataTypeWrapperError::FieldNotFound(field)),
                },
            };
        }

        match path_component {
            PathComponent::Field(name) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeWrapperError::FieldNotFound(name.to_string()))?;
                match name.as_str() {
                    "req_string" => Ok(field_type.to_any_value(&self.req_string)),
                    "req_bytes" => Ok(field_type.to_any_value(&self.req_bytes)),
                    "req_i32" => Ok(field_type.to_any_value(&self.req_i32)),
                    "req_i64" => Ok(field_type.to_any_value(&self.req_i64)),
                    "req_u32" => Ok(field_type.to_any_value(&self.req_u32)),
                    "req_u64" => Ok(field_type.to_any_value(&self.req_u64)),
                    "req_f32" => Ok(field_type.to_any_value(&self.req_f32)),
                    "req_f64" => Ok(field_type.to_any_value(&self.req_f64)),
                    "req_bool" => Ok(field_type.to_any_value(&self.req_bool)),
                    "req_struct" => Ok(field_type.to_any_value(&self.req_struct)),
                    "req_enum" => Ok(field_type.to_any_value(&self.req_enum)),
                    "req_enum2" => Ok(field_type.to_any_value(&self.req_enum2)),
                    "opt_string" => Ok(field_type.to_any_value(&self.opt_string)),
                    "opt_bytes" => Ok(field_type.to_any_value(&self.opt_bytes)),
                    "opt_i32" => Ok(field_type.to_any_value(&self.opt_i32)),
                    "opt_i64" => Ok(field_type.to_any_value(&self.opt_i64)),
                    "opt_u32" => Ok(field_type.to_any_value(&self.opt_u32)),
                    "opt_u64" => Ok(field_type.to_any_value(&self.opt_u64)),
                    "opt_f32" => Ok(field_type.to_any_value(&self.opt_f32)),
                    "opt_f64" => Ok(field_type.to_any_value(&self.opt_f64)),
                    "opt_bool" => Ok(field_type.to_any_value(&self.opt_bool)),
                    "opt_struct" => Ok(field_type.to_any_value(&self.opt_struct)),
                    "opt_enum" => Ok(field_type.to_any_value(&self.opt_enum)),
                    "opt_enum2" => Ok(field_type.to_any_value(&self.opt_enum2)),
                    "req_vec_req_item_string" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_string))
                    }
                    "req_vec_req_item_bytes" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_bytes))
                    }
                    "req_vec_req_item_i32" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_i32))
                    }
                    "req_vec_req_item_i64" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_i64))
                    }
                    "req_vec_req_item_u32" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_u32))
                    }
                    "req_vec_req_item_u64" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_u64))
                    }
                    "req_vec_req_item_f32" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_f32))
                    }
                    "req_vec_req_item_f64" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_f64))
                    }
                    "req_vec_req_item_bool" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_bool))
                    }
                    "req_vec_req_item_struct" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_struct))
                    }
                    "req_vec_req_item_enum" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_enum))
                    }
                    "req_vec_req_item_enum2" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_enum2))
                    }
                    "opt_vec_req_item_string" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_string))
                    }
                    "opt_vec_req_item_bytes" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_bytes))
                    }
                    "opt_vec_req_item_i32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_i32))
                    }
                    "opt_vec_req_item_i64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_i64))
                    }
                    "opt_vec_req_item_u32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_u32))
                    }
                    "opt_vec_req_item_u64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_u64))
                    }
                    "opt_vec_req_item_f32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_f32))
                    }
                    "opt_vec_req_item_f64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_f64))
                    }
                    "opt_vec_req_item_bool" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_bool))
                    }
                    "opt_vec_req_item_struct" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_struct))
                    }
                    "opt_vec_req_item_enum" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_enum))
                    }
                    "opt_vec_req_item_enum2" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_enum2))
                    }
                    "req_vec_opt_item_string" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_string))
                    }
                    "req_vec_opt_item_bytes" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_bytes))
                    }
                    "req_vec_opt_item_i32" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_i32))
                    }
                    "req_vec_opt_item_i64" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_i64))
                    }
                    "req_vec_opt_item_u32" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_u32))
                    }
                    "req_vec_opt_item_u64" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_u64))
                    }
                    "req_vec_opt_item_f32" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_f32))
                    }
                    "req_vec_opt_item_f64" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_f64))
                    }
                    "req_vec_opt_item_bool" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_bool))
                    }
                    "req_vec_opt_item_struct" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_struct))
                    }
                    "req_vec_opt_item_enum" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_enum))
                    }
                    "req_vec_opt_item_enum2" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_enum2))
                    }
                    "opt_vec_opt_item_string" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_string))
                    }
                    "opt_vec_opt_item_bytes" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_bytes))
                    }
                    "opt_vec_opt_item_i32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_i32))
                    }
                    "opt_vec_opt_item_i64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_i64))
                    }
                    "opt_vec_opt_item_u32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_u32))
                    }
                    "opt_vec_opt_item_u64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_u64))
                    }
                    "opt_vec_opt_item_f32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_f32))
                    }
                    "opt_vec_opt_item_f64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_f64))
                    }
                    "opt_vec_opt_item_bool" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_bool))
                    }
                    "opt_vec_opt_item_struct" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_struct))
                    }
                    "opt_vec_opt_item_enum" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_enum))
                    }
                    "opt_vec_opt_item_enum2" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_enum2))
                    }
                    _ => Err(DataTypeWrapperError::FieldNotFound(name.to_string())),
                }
            }
            PathComponent::ArrayIndex(name, index) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeWrapperError::FieldNotFound(name.to_string()))?;
                let field_inner_type = match &field_type.raw {
                    DataTypeOpt::List(inner_type) => inner_type,
                    DataTypeOpt::Option(midt) if matches!(midt.raw, DataTypeOpt::List(_)) => {
                        if let DataTypeOpt::List(ref inner_type) = midt.raw {
                            inner_type
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                };
                match name.as_str() {
                    "req_vec_req_item_string" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_string[index]))
                    }
                    "req_vec_req_item_bytes" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_bytes[index]))
                    }
                    "req_vec_req_item_i32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_i32[index]))
                    }
                    "req_vec_req_item_i64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_i64[index]))
                    }
                    "req_vec_req_item_u32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_u32[index]))
                    }
                    "req_vec_req_item_u64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_u64[index]))
                    }
                    "req_vec_req_item_f32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_f32[index]))
                    }
                    "req_vec_req_item_f64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_f64[index]))
                    }
                    "req_vec_req_item_bool" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_bool[index]))
                    }
                    "req_vec_req_item_struct" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_struct[index]))
                    }
                    "req_vec_req_item_enum" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_enum[index]))
                    }
                    "req_vec_req_item_enum2" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_req_item_enum2[index]))
                    }
                    "opt_vec_req_item_string" => match self.opt_vec_req_item_string {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_bytes" => match self.opt_vec_req_item_bytes {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_i32" => match self.opt_vec_req_item_i32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_i64" => match self.opt_vec_req_item_i64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_u32" => match self.opt_vec_req_item_u32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_u64" => match self.opt_vec_req_item_u64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_f32" => match self.opt_vec_req_item_f32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_f64" => match self.opt_vec_req_item_f64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_bool" => match self.opt_vec_req_item_bool {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_struct" => match self.opt_vec_req_item_struct {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_enum" => match self.opt_vec_req_item_enum {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_req_item_enum2" => match self.opt_vec_req_item_enum2 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "req_vec_opt_item_string" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_string[index]))
                    }
                    "req_vec_opt_item_bytes" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_bytes[index]))
                    }
                    "req_vec_opt_item_i32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_i32[index]))
                    }
                    "req_vec_opt_item_i64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_i64[index]))
                    }
                    "req_vec_opt_item_u32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_u32[index]))
                    }
                    "req_vec_opt_item_u64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_u64[index]))
                    }
                    "req_vec_opt_item_f32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_f32[index]))
                    }
                    "req_vec_opt_item_f64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_f64[index]))
                    }
                    "req_vec_opt_item_bool" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_bool[index]))
                    }
                    "req_vec_opt_item_struct" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_struct[index]))
                    }
                    "req_vec_opt_item_enum" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_enum[index]))
                    }
                    "req_vec_opt_item_enum2" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_enum2[index]))
                    }
                    "opt_vec_opt_item_string" => match self.opt_vec_opt_item_string {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_bytes" => match self.opt_vec_opt_item_bytes {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_i32" => match self.opt_vec_opt_item_i32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_i64" => match self.opt_vec_opt_item_i64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_u32" => match self.opt_vec_opt_item_u32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_u64" => match self.opt_vec_opt_item_u64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_f32" => match self.opt_vec_opt_item_f32 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_f64" => match self.opt_vec_opt_item_f64 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_bool" => match self.opt_vec_opt_item_bool {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_struct" => match self.opt_vec_opt_item_struct {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_enum" => match self.opt_vec_opt_item_enum {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    "opt_vec_opt_item_enum2" => match self.opt_vec_opt_item_enum2 {
                        Some(ref vec) => Ok(field_inner_type.to_any_value(&vec[index])),
                        None => Ok(AnyValue::Null),
                    },
                    _ => Err(DataTypeWrapperError::FieldNotFound(name.to_string())),
                }
            }
        }
    }
}
