use crate::sample::{SampleEnum, SampleStruct, SampleSubstruct};
use polars_core::prelude::AnyValue;
use polars_structpath_types::{
    DataTypeOpt, DataTypeWrapper, EnumPath, IntoAnyValueWith, StructPath,
};

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
