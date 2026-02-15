use crate::sample::SampleEnum;
use polars_structpath::impl_enum_buffer;

impl_enum_buffer!(SampleEnum, [(ITEM, 1)]);
