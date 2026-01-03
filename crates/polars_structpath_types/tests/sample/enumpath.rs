use crate::sample::SampleEnum;
use polars_structpath_types::impl_enum_buffer;

impl_enum_buffer!(SampleEnum, [(ITEM, 1)]);
