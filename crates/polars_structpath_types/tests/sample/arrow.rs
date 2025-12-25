use crate::sample::{SampleEnum, SampleStruct, SampleSubstruct};
use polars_structpath_types::{
    impl_enum_buffer, impl_struct_buffer, BoolBuffer, BufferVecOpt, BufferVecReq, F32Buffer, F64Buffer,
    I32Buffer, I64Buffer, StringBuffer, U32Buffer, U64Buffer, U8Buffer,
};

impl_enum_buffer!(SampleEnum, [("ITEM", 1)]);

impl_struct_buffer!(SampleSubstruct, [(subf_string, StringBuffer)]);

impl_struct_buffer!(
    SampleStruct,
    [
        (req_string, StringBuffer),
        (req_bytes, BufferVecReq::<U8Buffer>),
        (req_i32, I32Buffer),
        (req_i64, I64Buffer),
        (req_u32, U32Buffer),
        (req_u64, U64Buffer),
        (req_f32, F32Buffer),
        (req_f64, F64Buffer),
        (req_bool, BoolBuffer),
        (req_struct, SampleSubstructBuffer),
        (req_enum, SampleEnumBuffer),
        (req_enum2, I32Buffer),
        (opt_string, StringBuffer),
        (opt_bytes, BufferVecReq::<U8Buffer>),
        (opt_i32, I32Buffer),
        (opt_i64, I64Buffer),
        (opt_u32, U32Buffer),
        (opt_u64, U64Buffer),
        (opt_f32, F32Buffer),
        (opt_f64, F64Buffer),
        (opt_bool, BoolBuffer),
        (opt_struct, SampleSubstructBuffer),
        (opt_enum, SampleEnumBuffer),
        (opt_enum2, I32Buffer),
        (req_vec_req_item_string, BufferVecReq::<StringBuffer>),
        (
            req_vec_req_item_bytes,
            BufferVecReq::<BufferVecReq::<U8Buffer>>
        ),
        (req_vec_req_item_i32, BufferVecReq::<I32Buffer>),
        (req_vec_req_item_i64, BufferVecReq::<I64Buffer>),
        (req_vec_req_item_u32, BufferVecReq::<U32Buffer>),
        (req_vec_req_item_u64, BufferVecReq::<U64Buffer>),
        (req_vec_req_item_f32, BufferVecReq::<F32Buffer>),
        (req_vec_req_item_f64, BufferVecReq::<F64Buffer>),
        (req_vec_req_item_bool, BufferVecReq::<BoolBuffer>),
        (
            req_vec_req_item_struct,
            BufferVecReq::<SampleSubstructBuffer>
        ),
        (req_vec_req_item_enum, BufferVecReq::<SampleEnumBuffer>),
        (req_vec_req_item_enum2, BufferVecReq::<I32Buffer>),
        (opt_vec_req_item_string, BufferVecReq::<StringBuffer>),
        (
            opt_vec_req_item_bytes,
            BufferVecReq::<BufferVecReq::<U8Buffer>>
        ),
        (opt_vec_req_item_i32, BufferVecReq::<I32Buffer>),
        (opt_vec_req_item_i64, BufferVecReq::<I64Buffer>),
        (opt_vec_req_item_u32, BufferVecReq::<U32Buffer>),
        (opt_vec_req_item_u64, BufferVecReq::<U64Buffer>),
        (opt_vec_req_item_f32, BufferVecReq::<F32Buffer>),
        (opt_vec_req_item_f64, BufferVecReq::<F64Buffer>),
        (opt_vec_req_item_bool, BufferVecReq::<BoolBuffer>),
        (
            opt_vec_req_item_struct,
            BufferVecReq::<SampleSubstructBuffer>
        ),
        (opt_vec_req_item_enum, BufferVecReq::<SampleEnumBuffer>),
        (opt_vec_req_item_enum2, BufferVecReq::<I32Buffer>),
        (req_vec_opt_item_string, BufferVecOpt::<StringBuffer>),
        (
            req_vec_opt_item_bytes,
            BufferVecOpt::<BufferVecReq::<U8Buffer>>
        ),
        (req_vec_opt_item_i32, BufferVecOpt::<I32Buffer>),
        (req_vec_opt_item_i64, BufferVecOpt::<I64Buffer>),
        (req_vec_opt_item_u32, BufferVecOpt::<U32Buffer>),
        (req_vec_opt_item_u64, BufferVecOpt::<U64Buffer>),
        (req_vec_opt_item_f32, BufferVecOpt::<F32Buffer>),
        (req_vec_opt_item_f64, BufferVecOpt::<F64Buffer>),
        (req_vec_opt_item_bool, BufferVecOpt::<BoolBuffer>),
        (
            req_vec_opt_item_struct,
            BufferVecOpt::<SampleSubstructBuffer>
        ),
        (req_vec_opt_item_enum, BufferVecOpt::<SampleEnumBuffer>),
        (req_vec_opt_item_enum2, BufferVecOpt::<I32Buffer>),
        (opt_vec_opt_item_string, BufferVecOpt::<StringBuffer>),
        (
            opt_vec_opt_item_bytes,
            BufferVecOpt::<BufferVecReq::<U8Buffer>>
        ),
        (opt_vec_opt_item_i32, BufferVecOpt::<I32Buffer>),
        (opt_vec_opt_item_i64, BufferVecOpt::<I64Buffer>),
        (opt_vec_opt_item_u32, BufferVecOpt::<U32Buffer>),
        (opt_vec_opt_item_u64, BufferVecOpt::<U64Buffer>),
        (opt_vec_opt_item_f32, BufferVecOpt::<F32Buffer>),
        (opt_vec_opt_item_f64, BufferVecOpt::<F64Buffer>),
        (opt_vec_opt_item_bool, BufferVecOpt::<BoolBuffer>),
        (
            opt_vec_opt_item_struct,
            BufferVecOpt::<SampleSubstructBuffer>
        ),
        (opt_vec_opt_item_enum, BufferVecOpt::<SampleEnumBuffer>),
        (opt_vec_opt_item_enum2, BufferVecOpt::<I32Buffer>),
    ]
);
