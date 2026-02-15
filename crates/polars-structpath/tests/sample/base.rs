#[derive(Debug, Clone, PartialEq)]
pub struct SampleSubstruct {
    pub subf_string: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SampleEnum {
    #[allow(clippy::upper_case_acronyms)]
    ITEM = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleStruct {
    pub req_string: String,
    pub req_bytes: Vec<u8>,
    pub req_i32: i32,
    pub req_i64: i64,
    pub req_u32: u32,
    pub req_u64: u64,
    pub req_f32: f32,
    pub req_f64: f64,
    pub req_bool: bool,
    pub req_struct: SampleSubstruct,
    pub req_enum: SampleEnum,

    pub opt_string: Option<String>,
    pub opt_bytes: Option<Vec<u8>>,
    pub opt_i32: Option<i32>,
    pub opt_i64: Option<i64>,
    pub opt_u32: Option<u32>,
    pub opt_u64: Option<u64>,
    pub opt_f32: Option<f32>,
    pub opt_f64: Option<f64>,
    pub opt_bool: Option<bool>,
    pub opt_struct: Option<SampleSubstruct>,
    pub opt_enum: Option<SampleEnum>,

    pub req_vec_req_item_string: Vec<String>,
    pub req_vec_req_item_bytes: Vec<Vec<u8>>,
    pub req_vec_req_item_i32: Vec<i32>,
    pub req_vec_req_item_i64: Vec<i64>,
    pub req_vec_req_item_u32: Vec<u32>,
    pub req_vec_req_item_u64: Vec<u64>,
    pub req_vec_req_item_f32: Vec<f32>,
    pub req_vec_req_item_f64: Vec<f64>,
    pub req_vec_req_item_bool: Vec<bool>,
    pub req_vec_req_item_struct: Vec<SampleSubstruct>,
    pub req_vec_req_item_enum: Vec<SampleEnum>,

    pub opt_vec_req_item_string: Option<Vec<String>>,
    pub opt_vec_req_item_bytes: Option<Vec<Vec<u8>>>,
    pub opt_vec_req_item_i32: Option<Vec<i32>>,
    pub opt_vec_req_item_i64: Option<Vec<i64>>,
    pub opt_vec_req_item_u32: Option<Vec<u32>>,
    pub opt_vec_req_item_u64: Option<Vec<u64>>,
    pub opt_vec_req_item_f32: Option<Vec<f32>>,
    pub opt_vec_req_item_f64: Option<Vec<f64>>,
    pub opt_vec_req_item_bool: Option<Vec<bool>>,
    pub opt_vec_req_item_struct: Option<Vec<SampleSubstruct>>,
    pub opt_vec_req_item_enum: Option<Vec<SampleEnum>>,

    pub req_vec_opt_item_string: Vec<Option<String>>,
    pub req_vec_opt_item_bytes: Vec<Option<Vec<u8>>>,
    pub req_vec_opt_item_i32: Vec<Option<i32>>,
    pub req_vec_opt_item_i64: Vec<Option<i64>>,
    pub req_vec_opt_item_u32: Vec<Option<u32>>,
    pub req_vec_opt_item_u64: Vec<Option<u64>>,
    pub req_vec_opt_item_f32: Vec<Option<f32>>,
    pub req_vec_opt_item_f64: Vec<Option<f64>>,
    pub req_vec_opt_item_bool: Vec<Option<bool>>,
    pub req_vec_opt_item_struct: Vec<Option<SampleSubstruct>>,
    pub req_vec_opt_item_enum: Vec<Option<SampleEnum>>,

    pub opt_vec_opt_item_string: Option<Vec<Option<String>>>,
    pub opt_vec_opt_item_bytes: Option<Vec<Option<Vec<u8>>>>,
    pub opt_vec_opt_item_i32: Option<Vec<Option<i32>>>,
    pub opt_vec_opt_item_i64: Option<Vec<Option<i64>>>,
    pub opt_vec_opt_item_u32: Option<Vec<Option<u32>>>,
    pub opt_vec_opt_item_u64: Option<Vec<Option<u64>>>,
    pub opt_vec_opt_item_f32: Option<Vec<Option<f32>>>,
    pub opt_vec_opt_item_f64: Option<Vec<Option<f64>>>,
    pub opt_vec_opt_item_bool: Option<Vec<Option<bool>>>,
    pub opt_vec_opt_item_struct: Option<Vec<Option<SampleSubstruct>>>,
    pub opt_vec_opt_item_enum: Option<Vec<Option<SampleEnum>>>,
}
