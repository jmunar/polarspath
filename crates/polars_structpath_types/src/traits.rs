use polars_arrow::array::Array;
use polars_arrow::datatypes::ArrowDataType;
use polars_core::prelude::*;

pub trait ArrowBuffer {
    type ElementType;

    fn data_type(&self) -> &ArrowDataType;
    fn validity(&self) -> &Vec<bool>;
    fn new(nrows: usize) -> Self;
    fn push(&mut self, value: impl Into<Self::ElementType>);
    fn push_null(&mut self);
    fn to_arrow(self) -> PolarsResult<Box<dyn Array>>;
}

pub trait HasArrowBuffer {
    type BufferType: ArrowBuffer;

    fn new_buffer(nrows: usize) -> Self::BufferType {
        Self::BufferType::new(nrows)
    }
}
