use crate::traits::{ArrowBuffer, FromArrow, IntoArrow};
use polars_arrow::array::{Array, BooleanArray, ListArray, PrimitiveArray, Utf8Array};
use polars_arrow::datatypes::{ArrowDataType, Field as ArrowField};
use polars_arrow::offset::OffsetsBuffer;
use polars_core::prelude::*;

/// Buffer wrapper for `Option<T>` types.
///
/// `ArrowBufferOption` wraps any `ArrowBuffer` implementation to add support for nullable values.
/// It handles the conversion between Rust's `Option<T>` and Arrow's nullable arrays.
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::{ArrowBuffer, IntoArrow};
///
/// let mut buffer = Option::<String>::new_buffer(2);
/// buffer.push(Some("hello".to_string()));
/// buffer.push(None);
/// let array = buffer.to_arrow().unwrap();
/// ```
pub struct ArrowBufferOption<T: ArrowBuffer>(T);

impl<T: ArrowBuffer> ArrowBuffer for ArrowBufferOption<T> {
    type Element = Option<T::Element>;
    type Arrow = T::Arrow;

    fn data_type(&self) -> &ArrowDataType {
        self.0.data_type()
    }

    fn validity(&self) -> &Vec<bool> {
        self.0.validity()
    }

    fn new(nrows: usize) -> Self {
        ArrowBufferOption(T::new(nrows))
    }

    fn push(&mut self, value: impl Into<Self::Element>) {
        match value.into() {
            Some(value) => {
                self.0.push(value);
            }
            None => {
                self.0.push_null();
            }
        }
    }

    fn push_null(&mut self) {
        self.0.push_null();
    }

    fn to_arrow(self) -> PolarsResult<Self::Arrow> {
        self.0.to_arrow()
    }
}

/// Buffer for `Vec<T>` types.
///
/// `ArrowBufferVec` accumulates vectors of elements and converts them to Arrow `ListArray`.
/// It maintains offsets to track the boundaries between individual vectors in the buffer.
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::{ArrowBuffer, IntoArrow};
///
/// let mut buffer = Vec::<i32>::new_buffer(2);
/// buffer.push(vec![1, 2, 3]);
/// buffer.push(vec![4, 5]);
/// let array = buffer.to_arrow().unwrap();
/// ```
pub struct ArrowBufferVec<T: ArrowBuffer> {
    subbuffer: T,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl<T: ArrowBuffer> ArrowBuffer for ArrowBufferVec<T> {
    type Element = Vec<T::Element>;
    type Arrow = ListArray<i32>;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        let subbuffer = T::new(nrows);
        let mut offsets = Vec::with_capacity(nrows + 1);
        offsets.push(0i32);

        let list_field = ArrowField::new("item".into(), subbuffer.data_type().clone(), true);
        Self {
            subbuffer,
            offsets,
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::List(Box::new(list_field)),
        }
    }

    fn push(&mut self, value: impl Into<Self::Element>) {
        let vec = value.into();
        for element in vec {
            self.subbuffer.push(element);
        }
        self._validity.push(true);
        self.offsets.push(self.subbuffer.validity().len() as i32);
    }

    fn push_null(&mut self) {
        self._validity.push(false);
        self.offsets.push(self.subbuffer.validity().len() as i32);
    }

    fn to_arrow(self) -> PolarsResult<Self::Arrow> {
        let data_type = self.data_type().clone();
        let subarray = self.subbuffer.to_arrow()?;
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        ListArray::try_new(
            data_type,
            offsets,
            Box::new(subarray),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )
    }
}

impl<T: IntoArrow> IntoArrow for Option<T> {
    type Buffer = ArrowBufferOption<T::Buffer>;
}

impl<T: FromArrow> FromArrow for Option<T> {
    fn from_arrow(array: Box<dyn Array>) -> Vec<Option<T>> {
        T::from_arrow_opt(array)
    }

    fn from_arrow_opt(_: Box<dyn Array>) -> Vec<Option<Option<T>>> {
        panic!("Not implemented")
    }
}

impl<T: IntoArrow> IntoArrow for Vec<T> {
    type Buffer = ArrowBufferVec<T::Buffer>;
}

impl<T: FromArrow> FromArrow for Vec<T> {
    fn from_arrow(array: Box<dyn Array>) -> Vec<Vec<T>> {
        array
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap()
            .iter()
            .map(|opt| opt.map(|arr| T::from_arrow(arr)).unwrap_or_default())
            .collect()
    }

    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>> {
        array
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap()
            .iter()
            .map(|opt| opt.map(|vec| T::from_arrow(vec)))
            .collect()
    }
}

/// Buffer implementation for `String` types.
///
/// `StringBuffer` accumulates UTF-8 strings and converts them to Arrow `Utf8Array`.
/// It stores string data as bytes with offsets to track string boundaries.
pub struct StringBuffer {
    values: Vec<u8>,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl ArrowBuffer for StringBuffer {
    type Element = String;
    type Arrow = Utf8Array<i32>;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        let mut offsets = Vec::with_capacity(nrows + 1);
        offsets.push(0i32);
        Self {
            values: Vec::with_capacity(nrows),
            offsets,
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::Utf8,
        }
    }

    fn push(&mut self, value: impl Into<Self::Element>) {
        let value = value.into();
        self.values.extend_from_slice(value.as_bytes());
        self._validity.push(true);
        self.offsets.push(self.values.len() as i32);
    }

    fn push_null(&mut self) {
        self.values.push(0);
        self._validity.push(false);
        self.offsets.push(self.values.len() as i32);
    }

    fn to_arrow(self) -> PolarsResult<Self::Arrow> {
        let data_type = self.data_type().clone();
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        Utf8Array::<i32>::try_new(
            data_type,
            offsets,
            self.values.into(),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )
    }
}

impl IntoArrow for String {
    type Buffer = StringBuffer;
}

impl FromArrow for String {
    fn from_arrow(array: Box<dyn Array>) -> Vec<Self> {
        array
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|opt| opt.unwrap_or_default().to_string())
            .collect()
    }

    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>> {
        array
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|opt| opt.map(|s| s.to_string()))
            .collect()
    }
}

/// Macro to generate buffer structs and trait implementations for numeric and boolean types.
///
/// This macro generates:
/// - A buffer struct (e.g., `Int32Buffer` for `i32`)
/// - `ArrowBuffer` implementation for the buffer
/// - `IntoArrow` implementation for the element type
/// - `FromArrow` implementation for the element type
///
/// # Example
///
/// The macro is used internally to generate implementations for:
/// - `i32` → `Int32Buffer`
/// - `i64` → `Int64Buffer`
/// - `f32` → `Float32Buffer`
/// - `bool` → `BoolBuffer`
/// - And other primitive types
macro_rules! impl_primitive_buffer {
    (
        $element_type:ty,
        $arrow_data_type:expr
    ) => {
        $crate::paste::paste! {
            pub struct [<$arrow_data_type Buffer>] {
                values: Vec<$element_type>,
                _validity: Vec<bool>,
                _data_type: ArrowDataType,
            }

            impl ArrowBuffer for [<$arrow_data_type Buffer>] {
                type Element = $element_type;
                type Arrow = PrimitiveArray<$element_type>;

                fn data_type(&self) -> &ArrowDataType {
                    &self._data_type
                }

                fn validity(&self) -> &Vec<bool> {
                    &self._validity
                }

                fn new(nrows: usize) -> Self {
                    Self {
                        values: Vec::with_capacity(nrows),
                        _validity: Vec::with_capacity(nrows),
                        _data_type: $crate::polars_arrow::datatypes::ArrowDataType::$arrow_data_type,
                    }
                }

                fn push(&mut self, value: impl Into<Self::Element>) {
                    let value = value.into();
                    self.values.push(value);
                    self._validity.push(true);
                }

                fn push_null(&mut self) {
                    self.values.push(0 as $element_type);
                    self._validity.push(false);
                }

                fn to_arrow(self) -> PolarsResult<Self::Arrow> {
                    let data_type = self.data_type().clone();
                    PrimitiveArray::<$element_type>::try_new(
                        data_type,
                        self.values.into(),
                        Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
                    )
                }
            }

            impl IntoArrow for $element_type {
                type Buffer = [<$arrow_data_type Buffer>];
            }

            impl FromArrow for $element_type {

                fn from_arrow(array: Box<dyn Array>) -> Vec<Self> {
                    array
                    .as_any()
                    .downcast_ref::<PrimitiveArray<$element_type>>()
                    .unwrap()
                    .iter()
                    .map(|opt| *opt.unwrap_or(&(0 as $element_type)))
                    .collect()
                }

                fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>> {
                    array
                    .as_any()
                    .downcast_ref::<PrimitiveArray<$element_type>>()
                    .unwrap()
                    .iter()
                    .map(|opt| opt.copied())
                    .collect()
                }
            }
        }
    };
}

// Generate buffers for all numeric and boolean types
impl_primitive_buffer!(i32, Int32);
impl_primitive_buffer!(i64, Int64);
impl_primitive_buffer!(u8, UInt8);
impl_primitive_buffer!(u32, UInt32);
impl_primitive_buffer!(u64, UInt64);
impl_primitive_buffer!(f32, Float32);
impl_primitive_buffer!(f64, Float64);

/// Buffer implementation for `bool` types.
///
/// `BoolBuffer` accumulates boolean values and converts them to Arrow `BooleanArray`.
pub struct BoolBuffer {
    values: Vec<bool>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl ArrowBuffer for BoolBuffer {
    type Element = bool;
    type Arrow = BooleanArray;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        Self {
            values: Vec::with_capacity(nrows),
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::Boolean,
        }
    }

    fn push(&mut self, value: impl Into<Self::Element>) {
        let value = value.into();
        self.values.push(value);
        self._validity.push(true);
    }

    fn push_null(&mut self) {
        self.values.push(false);
        self._validity.push(false);
    }

    fn to_arrow(self) -> PolarsResult<Self::Arrow> {
        let data_type = self.data_type().clone();
        BooleanArray::try_new(
            data_type,
            self.values.into(),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )
    }
}

impl IntoArrow for bool {
    type Buffer = BoolBuffer;
}

impl FromArrow for bool {
    fn from_arrow(array: Box<dyn Array>) -> Vec<Self> {
        array
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap()
            .iter()
            .map(|opt| opt.unwrap_or_default())
            .collect()
    }

    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>> {
        array
            .as_any()
            .downcast_ref::<BooleanArray>()
            .unwrap()
            .iter()
            .collect()
    }
}
