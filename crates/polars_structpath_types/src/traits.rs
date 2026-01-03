use polars_arrow::array::Array;
use polars_arrow::datatypes::ArrowDataType;
use polars_core::prelude::*;

pub trait ArrowBuffer {
    type Element;
    type Arrow: Array;

    fn data_type(&self) -> &ArrowDataType;
    fn validity(&self) -> &Vec<bool>;
    fn new(nrows: usize) -> Self;
    fn push(&mut self, value: impl Into<Self::Element>);
    fn push_null(&mut self);
    fn to_arrow(self) -> PolarsResult<Self::Arrow>;
}

pub trait IntoArrow: Sized {
    type Buffer: ArrowBuffer;

    fn new_buffer(nrows: usize) -> Self::Buffer {
        Self::Buffer::new(nrows)
    }
}

pub trait FromArrow
where
    Self: Sized,
{
    fn from_arrow(array: Box<dyn Array>) -> Vec<Self>;
    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>>;
}
