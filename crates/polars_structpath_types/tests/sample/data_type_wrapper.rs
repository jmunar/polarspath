use crate::sample::{SampleEnum, SampleStruct, SampleSubstruct};
use polars_structpath_types::{
    data_type_wrapper, DataTypeOpt, DataTypeWrapper, EnumOptInfo, HasDataTypeWrapper,
};

use std::sync::OnceLock;

impl HasDataTypeWrapper for SampleSubstruct {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_OPT: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_OPT.get_or_init(|| data_type_wrapper!(Struct([("subf_string", String)])))
    }
}

impl HasDataTypeWrapper for SampleEnum {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_WRAPPER: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_WRAPPER.get_or_init(|| {
            DataTypeWrapper::new(DataTypeOpt::Enum(EnumOptInfo::from_iter([("ITEM", 1)])))
        })
    }
}

impl HasDataTypeWrapper for SampleStruct {
    fn data_type_wrapper() -> &'static DataTypeWrapper {
        static DATA_TYPE_WRAPPER: OnceLock<DataTypeWrapper> = OnceLock::new();
        DATA_TYPE_WRAPPER.get_or_init(|| {
            data_type_wrapper!(Struct([
                // Required scalar fields
                ("req_string", String),
                ("req_bytes", Bytes),
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
                ("opt_bytes", Option(Bytes)),
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
                ("req_vec_req_item_bytes", List(Bytes)),
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
                ("opt_vec_req_item_bytes", Option(List(Bytes))),
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
                ("req_vec_opt_item_bytes", List(Option(Bytes))),
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
                ("opt_vec_opt_item_bytes", Option(List(Option(Bytes)))),
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
