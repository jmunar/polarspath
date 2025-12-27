use polars_arrow::array::{Array, BooleanArray, ListArray, PrimitiveArray, Utf8Array};
use polars_arrow::datatypes::{ArrowDataType, Field as ArrowField};
use polars_arrow::offset::OffsetsBuffer;
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

pub struct ArrowBufferOption<T: ArrowBuffer>(T);

impl<T: ArrowBuffer> ArrowBuffer for ArrowBufferOption<T> {
    type ElementType = Option<T::ElementType>;

    fn data_type(&self) -> &ArrowDataType {
        self.0.data_type()
    }

    fn validity(&self) -> &Vec<bool> {
        self.0.validity()
    }

    fn new(nrows: usize) -> Self {
        ArrowBufferOption(T::new(nrows))
    }

    fn push(&mut self, value: impl Into<Self::ElementType>) {
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

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        self.0.to_arrow()
    }
}

pub struct ArrowBufferVec<T: ArrowBuffer> {
    subbuffer: T,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl<T: ArrowBuffer> ArrowBuffer for ArrowBufferVec<T> {
    type ElementType = Vec<T::ElementType>;

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

    fn push(&mut self, value: impl Into<Self::ElementType>) {
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

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let subarray = self.subbuffer.to_arrow()?;
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        let array = ListArray::try_new(
            data_type,
            offsets,
            subarray,
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

pub trait HasArrowBuffer {
    type BufferType: ArrowBuffer;

    fn new_buffer(nrows: usize) -> Self::BufferType {
        Self::BufferType::new(nrows)
    }
}

impl<T: HasArrowBuffer> HasArrowBuffer for Option<T> {
    type BufferType = ArrowBufferOption<T::BufferType>;
}

impl<T: HasArrowBuffer> HasArrowBuffer for Vec<T> {
    type BufferType = ArrowBufferVec<T::BufferType>;
}

pub struct StringBuffer {
    values: Vec<u8>,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl ArrowBuffer for StringBuffer {
    type ElementType = String;

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

    fn push(&mut self, value: impl Into<Self::ElementType>) {
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

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        let array = Utf8Array::<i32>::try_new(
            data_type,
            offsets,
            self.values.into(),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

impl HasArrowBuffer for String {
    type BufferType = StringBuffer;
}

/// Macro to generate buffer structs and trait implementations for numeric and boolean types.
macro_rules! impl_scalar_buffer {
    (
        $buffer_name:ident,
        $element_type:ty,
        $arrow_data_type:expr,
        $arrow_array_type:ty,
        $default_value:expr
    ) => {
        pub struct $buffer_name {
            values: Vec<$element_type>,
            _validity: Vec<bool>,
            _data_type: ArrowDataType,
        }

        impl ArrowBuffer for $buffer_name {
            type ElementType = $element_type;

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
                    _data_type: $arrow_data_type,
                }
            }

            fn push(&mut self, value: impl Into<Self::ElementType>) {
                let value = value.into();
                self.values.push(value);
                self._validity.push(true);
            }

            fn push_null(&mut self) {
                self.values.push($default_value);
                self._validity.push(false);
            }

            fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
                let data_type = self.data_type().clone();
                let array = <$arrow_array_type>::try_new(
                    data_type,
                    self.values.into(),
                    Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
                )?;
                Ok(Box::new(array) as Box<dyn Array>)
            }
        }

        impl HasArrowBuffer for $element_type {
            type BufferType = $buffer_name;
        }
    };
}

// Generate buffers for all numeric and boolean types
impl_scalar_buffer!(
    I32Buffer,
    i32,
    ArrowDataType::Int32,
    PrimitiveArray<i32>,
    0i32
);
impl_scalar_buffer!(
    I64Buffer,
    i64,
    ArrowDataType::Int64,
    PrimitiveArray<i64>,
    0i64
);
impl_scalar_buffer!(U8Buffer, u8, ArrowDataType::UInt8, PrimitiveArray<u8>, 0);
impl_scalar_buffer!(
    U32Buffer,
    u32,
    ArrowDataType::UInt32,
    PrimitiveArray<u32>,
    0u32
);
impl_scalar_buffer!(
    U64Buffer,
    u64,
    ArrowDataType::UInt64,
    PrimitiveArray<u64>,
    0u64
);
impl_scalar_buffer!(
    F32Buffer,
    f32,
    ArrowDataType::Float32,
    PrimitiveArray<f32>,
    0.0f32
);
impl_scalar_buffer!(
    F64Buffer,
    f64,
    ArrowDataType::Float64,
    PrimitiveArray<f64>,
    0.0f64
);
impl_scalar_buffer!(
    BoolBuffer,
    bool,
    ArrowDataType::Boolean,
    BooleanArray,
    false
);

#[macro_export]
#[doc(hidden)]
macro_rules! impl_struct_field_buffer_type {
    ( Option<$inner:ty> ) => {
        $crate::ArrowBufferOption<$crate::impl_struct_field_buffer_type!($inner)>
    };

    ( Vec<$inner:ty> ) => {
        $crate::ArrowBufferVec<$crate::impl_struct_field_buffer_type!($inner)>
    };
    ( $other:ty ) => {
        <$other as $crate::HasArrowBuffer>::BufferType
    }
}

/// Macro to generate struct buffer inner, buffer struct, and trait implementation.
///
/// Usage:
/// ```rust
/// use polars_structpath_types::impl_struct_buffer;
///
/// pub struct SampleSubstruct {
///     subf_string: String,
/// }
///
/// impl_struct_buffer!(
///     SampleSubstruct,
///     [(subf_string, String)]
/// );
/// ```
///
/// This generates:
/// - `SampleSubstructBuffer` struct
/// - `impl ArrowBuffer for SampleSubstructBuffer`
#[macro_export]
macro_rules! impl_struct_buffer {
    (
        $struct_type:ty,
        [$(($field_name:ident, $field_type:ty)),* $(,)?]
    ) => {
        paste::paste! {

            pub struct [<$struct_type Buffer>] {
                $(
                    [<buffer_ $field_name>]: $crate::impl_struct_field_buffer_type!($field_type),
                )*
                _validity: Vec<bool>,
                _data_type: polars_arrow::datatypes::ArrowDataType,
            }

            impl polars_structpath_types::ArrowBuffer for [<$struct_type Buffer>] {
                type ElementType = $struct_type;

                fn data_type(&self) -> &polars_arrow::datatypes::ArrowDataType {
                    &self._data_type
                }

                fn validity(&self) -> &Vec<bool> {
                    &self._validity
                }

                fn new(nrows: usize) -> Self {
                    $(
                        let [<buffer_ $field_name>] = <$crate::impl_struct_field_buffer_type!($field_type)>::new(nrows);
                    )*

                    let fields = vec![
                        $(
                            polars_arrow::datatypes::Field::new(
                                stringify!($field_name).into(),
                                [<buffer_ $field_name>].data_type().clone(),
                                true
                            ),
                        )*
                    ];

                    Self {
                        $(
                            [<buffer_ $field_name>]: [<buffer_ $field_name>],
                        )*
                        _validity: Vec::with_capacity(nrows),
                        _data_type: polars_arrow::datatypes::ArrowDataType::Struct(fields),
                    }
                }

                fn push(&mut self, value: impl Into<Self::ElementType>) {
                    let value = value.into();
                    $(
                        self.[<buffer_ $field_name>].push(value.$field_name);
                    )*
                    self._validity.push(true);
                }

                fn push_null(&mut self) {
                    $(
                        self.[<buffer_ $field_name>].push_null();
                    )*
                    self._validity.push(false);
                }

                fn to_arrow(self) -> polars_core::prelude::PolarsResult<Box<dyn polars_arrow::array::Array>> {
                    let data_type = self.data_type().clone();
                    let arrays = vec![
                        $(
                            self.[<buffer_ $field_name>].to_arrow()?,
                        )*
                    ];

                    let array = polars_arrow::array::StructArray::try_new(
                        data_type,
                        self._validity.len(),
                        arrays,
                        Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
                    )?;
                    Ok(Box::new(array) as Box<dyn polars_arrow::array::Array>)
                }
            }

            impl polars_structpath_types::HasArrowBuffer for $struct_type {
                type BufferType = [<$struct_type Buffer>];
            }
        }
    };
}

/// Macro to generate enum buffer struct and trait implementation.
///
/// Usage:
/// ```rust
/// use polars_structpath_types::impl_enum_buffer;
///
/// pub enum SampleEnum {
///     ITEM = 1,
/// }
///
/// impl_enum_buffer!(SampleEnum, [("ITEM", 1)]);
/// ```
///
/// This generates:
/// - `SampleEnumBuffer` struct
/// - `impl ArrowBuffer for SampleEnumBuffer`
#[macro_export]
macro_rules! impl_enum_buffer {
    (
        $element_type:ident,
        [$(($label:literal, $index:expr)),* $(,)?]
    ) => {
        paste::paste! {
            pub struct [<$element_type Buffer>] {
                values: Vec<Option<u32>>,
                idx: $crate::indexmap::IndexMap<u32, u32>,
                labels: polars_arrow::array::Utf8Array<i32>,
                _validity: Vec<bool>,
                _data_type: polars_arrow::datatypes::ArrowDataType,
            }

            impl polars_structpath_types::ArrowBuffer for [<$element_type Buffer>] {
                type ElementType = $element_type;

                fn data_type(&self) -> &polars_arrow::datatypes::ArrowDataType {
                    &self._data_type
                }

                fn validity(&self) -> &Vec<bool> {
                    &self._validity
                }

                fn new(nrows: usize) -> Self {
                    let mut idx = $crate::indexmap::IndexMap::new();
                    let mut labels_vec = Vec::new();
                    let mut dict_index = 0u32;
                    $(
                        idx.insert($index as u32, dict_index);
                        labels_vec.push(Some($label));
                        dict_index += 1;
                    )*
                    let labels = polars_arrow::array::Utf8Array::<i32>::from(labels_vec);
                    let dictionary_data_type = polars_arrow::datatypes::ArrowDataType::Dictionary(
                        polars_arrow::datatypes::IntegerType::UInt32,
                        Box::new(polars_arrow::datatypes::ArrowDataType::Utf8),
                        false, // ordered
                    );
                    Self {
                        values: Vec::with_capacity(nrows),
                        idx,
                        labels,
                        _validity: Vec::with_capacity(nrows),
                        _data_type: dictionary_data_type,
                    }
                }

                fn push(&mut self, value: impl Into<Self::ElementType>) {
                    let value = value.into();
                    self.values.push(Some(self.idx.get(&(value as u32)).unwrap().clone()));
                    self._validity.push(true);
                }

                fn push_null(&mut self) {
                    self.values.push(None);
                    self._validity.push(false);
                }

                fn to_arrow(self) -> polars_core::prelude::PolarsResult<Box<dyn polars_arrow::array::Array>> {
                    let array = polars_arrow::array::DictionaryArray::<u32>::try_new(
                        self._data_type,
                        polars_arrow::array::PrimitiveArray::<u32>::from(self.values),
                        Box::new(self.labels)
                    ).unwrap();
                    Ok(Box::new(array) as Box<dyn polars_arrow::array::Array>)
                }
            }

            impl polars_structpath_types::HasArrowBuffer for $element_type {
                type BufferType = [<$element_type Buffer>];
            }
        }
    };
}
