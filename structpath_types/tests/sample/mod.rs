use indexmap::IndexMap;
use polars_core::prelude::{AnyValue, CategoricalMapping, DataType, Field};
use std::sync::{Arc, OnceLock};
use structpath_types::{
    field_type, field_type_opt, DataTypeOpt, DataTypeOptError, EnumPath, HasDataTypeOpt,
    IntoAnyValueWith, Path, PathComponent, StructPath,
};

pub fn subfields_opt() -> IndexMap<String, DataTypeOpt> {
    IndexMap::from([field_type_opt!("subf_string", String)])
}

#[allow(dead_code)]
pub fn subfields_polars() -> Vec<Field> {
    Vec::from([Field::new("subf_string".into(), DataType::String)])
}

#[derive(Debug, Clone)]
pub struct SampleSubstruct {
    pub subf_string: String,
}

impl HasDataTypeOpt for SampleSubstruct {
    fn data_type_opt() -> &'static DataTypeOpt {
        static DATA_TYPE_OPT: OnceLock<DataTypeOpt> = OnceLock::new();
        DATA_TYPE_OPT.get_or_init(|| DataTypeOpt::Struct(Self::fields_opt().clone()))
    }

    fn data_type() -> &'static DataType {
        static DATA_TYPE: OnceLock<DataType> = OnceLock::new();
        DATA_TYPE.get_or_init(|| Self::data_type_opt().to_data_type())
    }
}

impl StructPath for SampleSubstruct {
    fn fields_opt() -> &'static IndexMap<String, DataTypeOpt> {
        static SUBFIELDS_OPT: OnceLock<IndexMap<String, DataTypeOpt>> = OnceLock::new();
        SUBFIELDS_OPT.get_or_init(|| subfields_opt())
    }

    fn fields() -> &'static [Field] {
        static FIELDS: OnceLock<Vec<Field>> = OnceLock::new();
        FIELDS
            .get_or_init(|| {
                Self::fields_opt()
                    .iter()
                    .map(|(field_name, field_type)| {
                        Field::new(field_name.into(), field_type.to_data_type())
                    })
                    .collect()
            })
            .as_slice()
    }

    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue, DataTypeOptError> {
        let path_component = path.components[0].clone();
        match path_component {
            PathComponent::Field(name) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeOptError::FieldNotFound(name.to_string()))?;
                match name.as_str() {
                    "subf_string" => Ok(field_type.to_any_value(&self.subf_string)),
                    _ => Err(DataTypeOptError::FieldNotFound(name.to_string())),
                }
            }
            PathComponent::ArrayIndex(name, _) => {
                Err(DataTypeOptError::FieldNotFound(name.to_string()))
            }
        }
    }
}

impl IntoAnyValueWith<SampleSubstruct> for DataTypeOpt
where
    SampleSubstruct: StructPath,
{
    type ChunkDataType = ::polars_core::prelude::StructType;
    fn to_any_value(&self, value: &SampleSubstruct) -> AnyValue {
        let field_defs = SampleSubstruct::fields().to_vec();
        let field_values = SampleSubstruct::fields_opt()
            .iter()
            .map(|(field_name, _)| value.get_value(field_name).unwrap().into_static())
            .collect::<Vec<AnyValue>>();
        AnyValue::StructOwned(Box::new((field_values, field_defs)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SampleEnum {
    ITEM = 1,
}

impl HasDataTypeOpt for SampleEnum {
    fn data_type_opt() -> &'static DataTypeOpt {
        static DATA_TYPE_OPT: OnceLock<DataTypeOpt> = OnceLock::new();
        DATA_TYPE_OPT.get_or_init(|| DataTypeOpt::Enum(IndexMap::from([("ITEM".into(), 1)])))
    }

    fn data_type() -> &'static DataType {
        static DATA_TYPE: OnceLock<DataType> = OnceLock::new();
        DATA_TYPE.get_or_init(|| Self::data_type_opt().to_data_type())
    }
}

impl EnumPath for SampleEnum
where
    SampleEnum: HasDataTypeOpt,
{
    fn mapping() -> &'static Arc<CategoricalMapping> {
        match Self::data_type() {
            DataType::Enum(_, mapping) => mapping,
            _ => unreachable!(),
        }
    }
}

impl IntoAnyValueWith<SampleEnum> for DataTypeOpt
where
    SampleEnum: EnumPath,
{
    type ChunkDataType = ::polars_core::prelude::CategoricalType;

    fn to_any_value(&self, value: &SampleEnum) -> AnyValue {
        match self {
            DataTypeOpt::Enum(_) => match value {
                SampleEnum::ITEM => AnyValue::Enum(0, SampleEnum::mapping()),
            },
            _ => panic!("Unsupported DataTypeOpt for SampleEnum: {:?}", self),
        }
    }
}

pub fn fields_opt() -> IndexMap<String, DataTypeOpt> {
    IndexMap::from([
        // Required scalar fields
        field_type_opt!("req_string", String),
        field_type_opt!("req_i32", Int32),
        field_type_opt!("req_i64", Int64),
        field_type_opt!("req_f64", Float64),
        field_type_opt!("req_bool", Boolean),
        field_type_opt!("req_struct", Struct([("subf_string", DataTypeOpt::String)])),
        field_type_opt!("req_enum", Enum([("ITEM", 1)])),
        // Optional scalar fields
        field_type_opt!("opt_string", Option, String),
        field_type_opt!("opt_i32", Option, Int32),
        field_type_opt!("opt_i64", Option, Int64),
        field_type_opt!("opt_f64", Option, Float64),
        field_type_opt!("opt_bool", Option, Boolean),
        field_type_opt!(
            "opt_struct",
            Option,
            Struct([("subf_string", DataTypeOpt::String)])
        ),
        field_type_opt!("opt_enum", Option, Enum([("ITEM", 1)])),
        // Required vector fields with required items
        field_type_opt!("req_vec_req_item_string", List, String),
        field_type_opt!("req_vec_req_item_i32", List, Int32),
        field_type_opt!("req_vec_req_item_i64", List, Int64),
        field_type_opt!("req_vec_req_item_f64", List, Float64),
        field_type_opt!("req_vec_req_item_bool", List, Boolean),
        field_type_opt!(
            "req_vec_req_item_struct",
            List,
            Struct([("subf_string", DataTypeOpt::String)])
        ),
        field_type_opt!("req_vec_req_item_enum", List, Enum([("ITEM", 1)])),
        // Optional vector fields with required items
        field_type_opt!("opt_vec_req_item_string", Option, List, String),
        field_type_opt!("opt_vec_req_item_i32", Option, List, Int32),
        field_type_opt!("opt_vec_req_item_i64", Option, List, Int64),
        field_type_opt!("opt_vec_req_item_f64", Option, List, Float64),
        field_type_opt!("opt_vec_req_item_bool", Option, List, Boolean),
        field_type_opt!(
            "opt_vec_req_item_struct",
            Option,
            List,
            Struct([("subf_string", DataTypeOpt::String)])
        ),
        field_type_opt!("opt_vec_req_item_enum", Option, List, Enum([("ITEM", 1)])),
        // Required vector fields with optional items
        field_type_opt!("req_vec_opt_item_string", List, Option, String),
        field_type_opt!("req_vec_opt_item_i32", List, Option, Int32),
        field_type_opt!("req_vec_opt_item_i64", List, Option, Int64),
        field_type_opt!("req_vec_opt_item_f64", List, Option, Float64),
        field_type_opt!("req_vec_opt_item_bool", List, Option, Boolean),
        field_type_opt!(
            "req_vec_opt_item_struct",
            List,
            Option,
            Struct([("subf_string", DataTypeOpt::String)])
        ),
        field_type_opt!("req_vec_opt_item_enum", List, Option, Enum([("ITEM", 1)])),
        // Optional vector fields with optional items
        field_type_opt!("opt_vec_opt_item_string", Option, List, Option, String),
        field_type_opt!("opt_vec_opt_item_i32", Option, List, Option, Int32),
        field_type_opt!("opt_vec_opt_item_i64", Option, List, Option, Int64),
        field_type_opt!("opt_vec_opt_item_f64", Option, List, Option, Float64),
        field_type_opt!("opt_vec_opt_item_bool", Option, List, Option, Boolean),
        field_type_opt!(
            "opt_vec_opt_item_struct",
            Option,
            List,
            Option,
            Struct([("subf_string", DataTypeOpt::String)])
        ),
        field_type_opt!(
            "opt_vec_opt_item_enum",
            Option,
            List,
            Option,
            Enum([("ITEM", 1)])
        ),
    ])
}

#[allow(dead_code)]
pub fn fields_polars() -> Vec<Field> {
    Vec::from([
        // Required scalar fields
        field_type!("req_string", String),
        field_type!("req_i32", Int32),
        field_type!("req_i64", Int64),
        field_type!("req_f64", Float64),
        field_type!("req_bool", Boolean),
        field_type!("req_struct", Struct(subfields_polars())),
        field_type!("req_enum", Enum([("ITEM", 1)])),
        // Optional scalar fields
        field_type!("opt_string", String),
        field_type!("opt_i32", Int32),
        field_type!("opt_i64", Int64),
        field_type!("opt_f64", Float64),
        field_type!("opt_bool", Boolean),
        field_type!("opt_struct", Struct(subfields_polars())),
        field_type!("opt_enum", Enum([("ITEM", 1)])),
        // Required vector fields with required items
        field_type!("req_vec_req_item_string", List, String),
        field_type!("req_vec_req_item_i32", List, Int32),
        field_type!("req_vec_req_item_i64", List, Int64),
        field_type!("req_vec_req_item_f64", List, Float64),
        field_type!("req_vec_req_item_bool", List, Boolean),
        field_type!("req_vec_req_item_struct", List, Struct(subfields_polars())),
        field_type!("req_vec_req_item_enum", List, Enum([("ITEM", 1)])),
        // Optional vector fields with required items
        field_type!("opt_vec_req_item_string", List, String),
        field_type!("opt_vec_req_item_i32", List, Int32),
        field_type!("opt_vec_req_item_i64", List, Int64),
        field_type!("opt_vec_req_item_f64", List, Float64),
        field_type!("opt_vec_req_item_bool", List, Boolean),
        field_type!("opt_vec_req_item_struct", List, Struct(subfields_polars())),
        field_type!("opt_vec_req_item_enum", List, Enum([("ITEM", 1)])),
        // Required vector fields with optional items
        field_type!("req_vec_opt_item_string", List, String),
        field_type!("req_vec_opt_item_i32", List, Int32),
        field_type!("req_vec_opt_item_i64", List, Int64),
        field_type!("req_vec_opt_item_f64", List, Float64),
        field_type!("req_vec_opt_item_bool", List, Boolean),
        field_type!("req_vec_opt_item_struct", List, Struct(subfields_polars())),
        field_type!("req_vec_opt_item_enum", List, Enum([("ITEM", 1)])),
        // Optional vector fields with optional items
        field_type!("opt_vec_opt_item_string", List, String),
        field_type!("opt_vec_opt_item_i32", List, Int32),
        field_type!("opt_vec_opt_item_i64", List, Int64),
        field_type!("opt_vec_opt_item_f64", List, Float64),
        field_type!("opt_vec_opt_item_bool", List, Boolean),
        field_type!("opt_vec_opt_item_struct", List, Struct(subfields_polars())),
        field_type!("opt_vec_opt_item_enum", List, Enum([("ITEM", 1)])),
    ])
}

#[derive(Debug, Clone)]
pub struct SampleStruct {
    pub req_string: String,
    pub req_i32: i32,
    pub req_i64: i64,
    pub req_f64: f64,
    pub req_bool: bool,
    pub req_struct: SampleSubstruct,
    pub req_enum: SampleEnum,

    pub opt_string: Option<String>,
    pub opt_i32: Option<i32>,
    pub opt_i64: Option<i64>,
    pub opt_f64: Option<f64>,
    pub opt_bool: Option<bool>,
    pub opt_struct: Option<SampleSubstruct>,
    pub opt_enum: Option<SampleEnum>,

    pub req_vec_req_item_string: Vec<String>,
    pub req_vec_req_item_i32: Vec<i32>,
    pub req_vec_req_item_i64: Vec<i64>,
    pub req_vec_req_item_f64: Vec<f64>,
    pub req_vec_req_item_bool: Vec<bool>,
    pub req_vec_req_item_struct: Vec<SampleSubstruct>,
    pub req_vec_req_item_enum: Vec<SampleEnum>,

    pub opt_vec_req_item_string: Option<Vec<String>>,
    pub opt_vec_req_item_i32: Option<Vec<i32>>,
    pub opt_vec_req_item_i64: Option<Vec<i64>>,
    pub opt_vec_req_item_f64: Option<Vec<f64>>,
    pub opt_vec_req_item_bool: Option<Vec<bool>>,
    pub opt_vec_req_item_struct: Option<Vec<SampleSubstruct>>,
    pub opt_vec_req_item_enum: Option<Vec<SampleEnum>>,

    pub req_vec_opt_item_string: Vec<Option<String>>,
    pub req_vec_opt_item_i32: Vec<Option<i32>>,
    pub req_vec_opt_item_i64: Vec<Option<i64>>,
    pub req_vec_opt_item_f64: Vec<Option<f64>>,
    pub req_vec_opt_item_bool: Vec<Option<bool>>,
    pub req_vec_opt_item_struct: Vec<Option<SampleSubstruct>>,
    pub req_vec_opt_item_enum: Vec<Option<SampleEnum>>,

    pub opt_vec_opt_item_string: Option<Vec<Option<String>>>,
    pub opt_vec_opt_item_i32: Option<Vec<Option<i32>>>,
    pub opt_vec_opt_item_i64: Option<Vec<Option<i64>>>,
    pub opt_vec_opt_item_f64: Option<Vec<Option<f64>>>,
    pub opt_vec_opt_item_bool: Option<Vec<Option<bool>>>,
    pub opt_vec_opt_item_struct: Option<Vec<Option<SampleSubstruct>>>,
    pub opt_vec_opt_item_enum: Option<Vec<Option<SampleEnum>>>,
}

impl HasDataTypeOpt for SampleStruct {
    fn data_type_opt() -> &'static DataTypeOpt {
        static DATA_TYPE_OPT: OnceLock<DataTypeOpt> = OnceLock::new();
        DATA_TYPE_OPT.get_or_init(|| DataTypeOpt::Struct(Self::fields_opt().clone()))
    }

    fn data_type() -> &'static DataType {
        static DATA_TYPE: OnceLock<DataType> = OnceLock::new();
        DATA_TYPE.get_or_init(|| Self::data_type_opt().to_data_type())
    }
}

impl StructPath for SampleStruct {
    fn fields_opt() -> &'static IndexMap<String, DataTypeOpt> {
        static FIELDS_OPT: OnceLock<IndexMap<String, DataTypeOpt>> = OnceLock::new();
        FIELDS_OPT.get_or_init(|| fields_opt())
    }

    fn fields() -> &'static [Field] {
        static FIELDS: OnceLock<Vec<Field>> = OnceLock::new();
        FIELDS
            .get_or_init(|| {
                Self::fields_opt()
                    .iter()
                    .map(|(field_name, field_type)| {
                        Field::new(field_name.into(), field_type.to_data_type())
                    })
                    .collect()
            })
            .as_slice()
    }

    fn get_value_by_path(&self, path: &Path) -> Result<AnyValue, DataTypeOptError> {
        let path_component = path.components[0].clone();

        if path.components.len() > 1 {
            let path_component = path.components[0].clone();
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
                    _ => Err(DataTypeOptError::FieldNotFound(field)),
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
                    _ => Err(DataTypeOptError::FieldNotFound(field)),
                },
            };
        }

        match path_component {
            PathComponent::Field(name) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeOptError::FieldNotFound(name.to_string()))?;
                match name.as_str() {
                    "req_string" => Ok(field_type.to_any_value(&self.req_string)),
                    "req_i32" => Ok(field_type.to_any_value(&self.req_i32)),
                    "req_i64" => Ok(field_type.to_any_value(&self.req_i64)),
                    "req_f64" => Ok(field_type.to_any_value(&self.req_f64)),
                    "req_bool" => Ok(field_type.to_any_value(&self.req_bool)),
                    "req_struct" => Ok(field_type.to_any_value(&self.req_struct)),
                    "req_enum" => Ok(field_type.to_any_value(&self.req_enum)),
                    "opt_string" => Ok(field_type.to_any_value(&self.opt_string)),
                    "opt_i32" => Ok(field_type.to_any_value(&self.opt_i32)),
                    "opt_i64" => Ok(field_type.to_any_value(&self.opt_i64)),
                    "opt_f64" => Ok(field_type.to_any_value(&self.opt_f64)),
                    "opt_bool" => Ok(field_type.to_any_value(&self.opt_bool)),
                    "opt_struct" => Ok(field_type.to_any_value(&self.opt_struct)),
                    "opt_enum" => Ok(field_type.to_any_value(&self.opt_enum)),
                    "req_vec_req_item_string" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_string))
                    }
                    "req_vec_req_item_i32" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_i32))
                    }
                    "req_vec_req_item_i64" => {
                        Ok(field_type.to_any_value(&self.req_vec_req_item_i64))
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
                    "opt_vec_req_item_string" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_string))
                    }
                    "opt_vec_req_item_i32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_i32))
                    }
                    "opt_vec_req_item_i64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_req_item_i64))
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
                    "req_vec_opt_item_string" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_string))
                    }
                    "req_vec_opt_item_i32" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_i32))
                    }
                    "req_vec_opt_item_i64" => {
                        Ok(field_type.to_any_value(&self.req_vec_opt_item_i64))
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
                    "opt_vec_opt_item_string" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_string))
                    }
                    "opt_vec_opt_item_i32" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_i32))
                    }
                    "opt_vec_opt_item_i64" => {
                        Ok(field_type.to_any_value(&self.opt_vec_opt_item_i64))
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
                    _ => Err(DataTypeOptError::FieldNotFound(name.to_string())),
                }
            }
            PathComponent::ArrayIndex(name, index) => {
                let field_type = Self::fields_opt()
                    .get(&name)
                    .ok_or(DataTypeOptError::FieldNotFound(name.to_string()))?;
                let field_inner_type = match field_type {
                    DataTypeOpt::List(inner_type) => &**inner_type,
                    DataTypeOpt::Option(mid_ty) if matches!(**mid_ty, DataTypeOpt::List(_)) => {
                        if let DataTypeOpt::List(inner_type) = &**mid_ty {
                            inner_type
                        } else {
                            return Err(DataTypeOptError::FieldNotFound(name.to_string()));
                        }
                    }
                    _ => return Err(DataTypeOptError::FieldNotFound(name.to_string())),
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
                    "req_vec_opt_item_string" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_string[index]))
                    }
                    "req_vec_opt_item_i32" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_i32[index]))
                    }
                    "req_vec_opt_item_i64" => {
                        Ok(field_inner_type.to_any_value(&self.req_vec_opt_item_i64[index]))
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
                    _ => Err(DataTypeOptError::FieldNotFound(name.to_string())),
                }
            }
        }
    }
}

impl IntoAnyValueWith<SampleStruct> for DataTypeOpt
where
    SampleStruct: StructPath,
{
    type ChunkDataType = ::polars_core::prelude::StructType;
    fn to_any_value(&self, value: &SampleStruct) -> AnyValue {
        let field_defs = SampleStruct::fields().to_vec();
        let field_values = SampleStruct::fields_opt()
            .iter()
            .map(|(field_name, _)| value.get_value(field_name).unwrap().into_static())
            .collect::<Vec<AnyValue>>();
        AnyValue::StructOwned(Box::new((field_values, field_defs)))
    }
}
