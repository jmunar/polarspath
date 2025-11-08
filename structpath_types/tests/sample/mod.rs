use polars_core::prelude::AnyValue;
use std::sync::OnceLock;
use structpath_types::{
    data_type_wrapper, DataTypeOpt, DataTypeWrapper, DataTypeWrapperError, EnumOptInfo, EnumPath,
    HasDataTypeWrapper, IntoAnyValueWith, Path, PathComponent, StructPath,
};

#[derive(Debug, Clone)]
pub struct SampleSubstruct {
    pub subf_string: String,
}

impl HasDataTypeWrapper for SampleSubstruct {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_OPT: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_OPT.get_or_init(|| data_type_wrapper!(Struct([("subf_string", String)])))
    }
}

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

impl IntoAnyValueWith<SampleSubstruct> for DataTypeWrapper
where
    SampleSubstruct: StructPath,
{
    type ChunkDataType = ::polars_core::prelude::StructType;
    fn to_any_value(&self, value: &SampleSubstruct) -> AnyValue<'_> {
        let field_defs = SampleSubstruct::fields().clone();
        let field_values = SampleSubstruct::fields_opt()
            .iter()
            .map(|(field_name, _)| value.get_value(field_name).unwrap().into_static())
            .collect::<Vec<AnyValue>>();
        AnyValue::StructOwned(Box::new((field_values, field_defs)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SampleEnum {
    #[allow(clippy::upper_case_acronyms)]
    ITEM = 1,
}

impl HasDataTypeWrapper for SampleEnum {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_WRAPPER: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_WRAPPER.get_or_init(|| {
            DataTypeWrapper::new(DataTypeOpt::Enum(EnumOptInfo::from_iter([("ITEM", 1)])))
        })
    }
}

impl EnumPath for SampleEnum {}

impl IntoAnyValueWith<SampleEnum> for DataTypeWrapper
where
    SampleEnum: EnumPath,
{
    type ChunkDataType = ::polars_core::prelude::CategoricalType;

    fn to_any_value(&self, value: &SampleEnum) -> AnyValue<'_> {
        match &self.raw {
            DataTypeOpt::Enum(_) => match value {
                SampleEnum::ITEM => AnyValue::Enum(0, SampleEnum::mapping()),
            },
            _ => panic!("Unsupported DataTypeWrapper for SampleEnum: {:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SampleStruct {
    pub req_string: String,
    pub req_i32: i32,
    pub req_i64: i64,
    pub req_u32: u32,
    pub req_u64: u64,
    pub req_f32: f32,
    pub req_f64: f64,
    pub req_bool: bool,
    pub req_struct: SampleSubstruct,
    pub req_enum: SampleEnum,
    pub req_enum2: i32,

    pub opt_string: Option<String>,
    pub opt_i32: Option<i32>,
    pub opt_i64: Option<i64>,
    pub opt_u32: Option<u32>,
    pub opt_u64: Option<u64>,
    pub opt_f32: Option<f32>,
    pub opt_f64: Option<f64>,
    pub opt_bool: Option<bool>,
    pub opt_struct: Option<SampleSubstruct>,
    pub opt_enum: Option<SampleEnum>,
    pub opt_enum2: Option<i32>,

    pub req_vec_req_item_string: Vec<String>,
    pub req_vec_req_item_i32: Vec<i32>,
    pub req_vec_req_item_i64: Vec<i64>,
    pub req_vec_req_item_u32: Vec<u32>,
    pub req_vec_req_item_u64: Vec<u64>,
    pub req_vec_req_item_f32: Vec<f32>,
    pub req_vec_req_item_f64: Vec<f64>,
    pub req_vec_req_item_bool: Vec<bool>,
    pub req_vec_req_item_struct: Vec<SampleSubstruct>,
    pub req_vec_req_item_enum: Vec<SampleEnum>,
    pub req_vec_req_item_enum2: Vec<i32>,

    pub opt_vec_req_item_string: Option<Vec<String>>,
    pub opt_vec_req_item_i32: Option<Vec<i32>>,
    pub opt_vec_req_item_i64: Option<Vec<i64>>,
    pub opt_vec_req_item_u32: Option<Vec<u32>>,
    pub opt_vec_req_item_u64: Option<Vec<u64>>,
    pub opt_vec_req_item_f32: Option<Vec<f32>>,
    pub opt_vec_req_item_f64: Option<Vec<f64>>,
    pub opt_vec_req_item_bool: Option<Vec<bool>>,
    pub opt_vec_req_item_struct: Option<Vec<SampleSubstruct>>,
    pub opt_vec_req_item_enum: Option<Vec<SampleEnum>>,
    pub opt_vec_req_item_enum2: Option<Vec<i32>>,

    pub req_vec_opt_item_string: Vec<Option<String>>,
    pub req_vec_opt_item_i32: Vec<Option<i32>>,
    pub req_vec_opt_item_i64: Vec<Option<i64>>,
    pub req_vec_opt_item_u32: Vec<Option<u32>>,
    pub req_vec_opt_item_u64: Vec<Option<u64>>,
    pub req_vec_opt_item_f32: Vec<Option<f32>>,
    pub req_vec_opt_item_f64: Vec<Option<f64>>,
    pub req_vec_opt_item_bool: Vec<Option<bool>>,
    pub req_vec_opt_item_struct: Vec<Option<SampleSubstruct>>,
    pub req_vec_opt_item_enum: Vec<Option<SampleEnum>>,
    pub req_vec_opt_item_enum2: Vec<Option<i32>>,

    pub opt_vec_opt_item_string: Option<Vec<Option<String>>>,
    pub opt_vec_opt_item_i32: Option<Vec<Option<i32>>>,
    pub opt_vec_opt_item_i64: Option<Vec<Option<i64>>>,
    pub opt_vec_opt_item_u32: Option<Vec<Option<u32>>>,
    pub opt_vec_opt_item_u64: Option<Vec<Option<u64>>>,
    pub opt_vec_opt_item_f32: Option<Vec<Option<f32>>>,
    pub opt_vec_opt_item_f64: Option<Vec<Option<f64>>>,
    pub opt_vec_opt_item_bool: Option<Vec<Option<bool>>>,
    pub opt_vec_opt_item_struct: Option<Vec<Option<SampleSubstruct>>>,
    pub opt_vec_opt_item_enum: Option<Vec<Option<SampleEnum>>>,
    pub opt_vec_opt_item_enum2: Option<Vec<Option<i32>>>,
}

impl HasDataTypeWrapper for SampleStruct {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_WRAPPER: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_WRAPPER.get_or_init(|| {
            data_type_wrapper!(Struct([
                // Required scalar fields
                ("req_string", String),
                ("req_i32", Int32),
                ("req_i64", Int64),
                ("req_u32", UInt32),
                ("req_u64", UInt64),
                ("req_f32", Float32),
                ("req_f64", Float64),
                ("req_bool", Boolean),
                ("req_struct", FromStructPath(SampleSubstruct)),
                ("req_enum", FromEnumPath(SampleEnum)),
                ("req_enum2", FromEnumPath(SampleEnum)),
                // Optional scalar fields
                ("opt_string", Option(String)),
                ("opt_i32", Option(Int32)),
                ("opt_i64", Option(Int64)),
                ("opt_u32", Option(UInt32)),
                ("opt_u64", Option(UInt64)),
                ("opt_f32", Option(Float32)),
                ("opt_f64", Option(Float64)),
                ("opt_bool", Option(Boolean)),
                ("opt_struct", Option(FromStructPath(SampleSubstruct))),
                ("opt_enum", Option(FromEnumPath(SampleEnum))),
                ("opt_enum2", Option(FromEnumPath(SampleEnum))),
                // Required vector fields with required items
                ("req_vec_req_item_string", List(String)),
                ("req_vec_req_item_i32", List(Int32)),
                ("req_vec_req_item_i64", List(Int64)),
                ("req_vec_req_item_u32", List(UInt32)),
                ("req_vec_req_item_u64", List(UInt64)),
                ("req_vec_req_item_f32", List(Float32)),
                ("req_vec_req_item_f64", List(Float64)),
                ("req_vec_req_item_bool", List(Boolean)),
                (
                    "req_vec_req_item_struct",
                    List(FromStructPath(SampleSubstruct))
                ),
                ("req_vec_req_item_enum", List(FromEnumPath(SampleEnum))),
                ("req_vec_req_item_enum2", List(FromEnumPath(SampleEnum))),
                // Optional vector fields with required items
                ("opt_vec_req_item_string", Option(List(String))),
                ("opt_vec_req_item_i32", Option(List(Int32))),
                ("opt_vec_req_item_i64", Option(List(Int64))),
                ("opt_vec_req_item_u32", Option(List(UInt32))),
                ("opt_vec_req_item_u64", Option(List(UInt64))),
                ("opt_vec_req_item_f32", Option(List(Float32))),
                ("opt_vec_req_item_f64", Option(List(Float64))),
                ("opt_vec_req_item_bool", Option(List(Boolean))),
                (
                    "opt_vec_req_item_struct",
                    Option(List(FromStructPath(SampleSubstruct)))
                ),
                (
                    "opt_vec_req_item_enum",
                    Option(List(FromEnumPath(SampleEnum)))
                ),
                (
                    "opt_vec_req_item_enum2",
                    Option(List(FromEnumPath(SampleEnum)))
                ),
                // Required vector fields with optional items
                ("req_vec_opt_item_string", List(Option(String))),
                ("req_vec_opt_item_i32", List(Option(Int32))),
                ("req_vec_opt_item_i64", List(Option(Int64))),
                ("req_vec_opt_item_u32", List(Option(UInt32))),
                ("req_vec_opt_item_u64", List(Option(UInt64))),
                ("req_vec_opt_item_f32", List(Option(Float32))),
                ("req_vec_opt_item_f64", List(Option(Float64))),
                ("req_vec_opt_item_bool", List(Option(Boolean))),
                (
                    "req_vec_opt_item_struct",
                    List(Option(FromStructPath(SampleSubstruct)))
                ),
                (
                    "req_vec_opt_item_enum",
                    List(Option(FromEnumPath(SampleEnum)))
                ),
                (
                    "req_vec_opt_item_enum2",
                    List(Option(FromEnumPath(SampleEnum)))
                ),
                // Optional vector fields with optional items
                ("opt_vec_opt_item_string", Option(List(Option(String)))),
                ("opt_vec_opt_item_i32", Option(List(Option(Int32)))),
                ("opt_vec_opt_item_i64", Option(List(Option(Int64)))),
                ("opt_vec_opt_item_u32", Option(List(Option(UInt32)))),
                ("opt_vec_opt_item_u64", Option(List(Option(UInt64)))),
                ("opt_vec_opt_item_f32", Option(List(Option(Float32)))),
                ("opt_vec_opt_item_f64", Option(List(Option(Float64)))),
                ("opt_vec_opt_item_bool", Option(List(Option(Boolean)))),
                (
                    "opt_vec_opt_item_struct",
                    Option(List(Option(FromStructPath(SampleSubstruct))))
                ),
                (
                    "opt_vec_opt_item_enum",
                    Option(List(Option(FromEnumPath(SampleEnum))))
                ),
                (
                    "opt_vec_opt_item_enum2",
                    Option(List(Option(FromEnumPath(SampleEnum))))
                )
            ]))
        })
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

impl IntoAnyValueWith<SampleStruct> for DataTypeWrapper
where
    SampleStruct: StructPath,
{
    type ChunkDataType = ::polars_core::prelude::StructType;
    fn to_any_value(&self, value: &SampleStruct) -> AnyValue<'_> {
        let field_defs = SampleStruct::fields().clone();
        let field_values = SampleStruct::fields_opt()
            .iter()
            .map(|(field_name, _)| value.get_value(field_name).unwrap().into_static())
            .collect::<Vec<AnyValue>>();
        AnyValue::StructOwned(Box::new((field_values, field_defs)))
    }
}
