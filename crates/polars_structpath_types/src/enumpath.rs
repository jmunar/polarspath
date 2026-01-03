#[macro_export]
macro_rules! rust_idx_to_arrow_idx {
    ($rust_idx:expr, [$($rust_idx_val:expr),*]) => {
        $crate::rust_idx_to_arrow_idx!(@match $rust_idx, 0, $($rust_idx_val),*)
    };

    (@match $rust_idx:expr, $arrow_idx:expr, $head:expr $(, $tail:expr)*) => {
        match $rust_idx {
            $head => $arrow_idx,
            _ => $crate::rust_idx_to_arrow_idx!(@match $rust_idx, $arrow_idx + 1, $($tail),*),
        }
    };

    (@match $rust_idx:expr, $_arrow_idx:expr,) => {
        panic!("Invalid rust index: {}", $rust_idx)
    };
}

#[macro_export]
macro_rules! arrow_idx_to_rust_idx {
    ($arrow_idx:expr, [$($rust_idx_val:expr),*]) => {
        $crate::arrow_idx_to_rust_idx!(@match $arrow_idx, 0, $($rust_idx_val),*)
    };

    (@match $arrow_idx:expr, $pos:expr, $head:expr $(, $tail:expr)*) => {
        if $arrow_idx == $pos {
            $head
        } else {
            $crate::arrow_idx_to_rust_idx!(@match $arrow_idx, $pos + 1, $($tail),*)
        }
    };

    (@match $arrow_idx:expr, $_pos:expr,) => {
        panic!("Invalid arrow index: {}", $arrow_idx)
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
/// impl_enum_buffer!(SampleEnum, [(ITEM, 1)]);
/// ```
///
/// This generates:
/// - `SampleEnumBuffer` struct
/// - `impl ArrowBuffer for SampleEnumBuffer`
#[macro_export]
macro_rules! impl_enum_buffer {
    (
        $element_type:ident,
        [$(($identifier:ident, $index:expr)),* $(,)?]
    ) => {
        $crate::paste::paste! {

            impl $element_type {

                fn _from_rust_idx(rust_idx: u32) -> Self {
                    match rust_idx {
                        $($index => Self::$identifier,)*
                        _ => panic!("Invalid rust index: {}", rust_idx),
                    }
                }

                pub fn from_arrow_idx(arrow_idx: u32) -> Self {
                    Self::_from_rust_idx(Self::_arrow_idx_to_rust_idx(arrow_idx))
                }

                pub fn rust_idx_to_arrow_idx(rust_idx: u32) -> u32 {
                    $crate::rust_idx_to_arrow_idx!(rust_idx, [$($index),*])
                }

                fn _arrow_idx_to_rust_idx(arrow_idx: u32) -> u32 {
                    $crate::arrow_idx_to_rust_idx!(arrow_idx, [$($index),*])
                }
            }

            pub struct [<$element_type Buffer>] {
                values: Vec<Option<u32>>,
                _validity: Vec<bool>,
                _data_type: $crate::polars_arrow::datatypes::ArrowDataType,
            }

            impl $crate::ArrowBuffer for [<$element_type Buffer>] {
                type Element = $element_type;
                type Arrow = $crate::polars_arrow::array::DictionaryArray<u32>;

                fn data_type(&self) -> &$crate::polars_arrow::datatypes::ArrowDataType {
                    &self._data_type
                }

                fn validity(&self) -> &Vec<bool> {
                    &self._validity
                }

                fn new(nrows: usize) -> Self {
                    let dictionary_data_type = $crate::polars_arrow::datatypes::ArrowDataType::Dictionary(
                        $crate::polars_arrow::datatypes::IntegerType::UInt32,
                        Box::new($crate::polars_arrow::datatypes::ArrowDataType::Utf8),
                        false, // ordered
                    );
                    Self {
                        values: Vec::with_capacity(nrows),
                        _validity: Vec::with_capacity(nrows),
                        _data_type: dictionary_data_type,
                    }
                }

                fn push(&mut self, value: impl Into<Self::Element>) {
                    let value = value.into();
                    self.values.push(Some(value as u32));
                    self._validity.push(true);
                }

                fn push_null(&mut self) {
                    self.values.push(None);
                    self._validity.push(false);
                }

                fn to_arrow(self) -> $crate::polars_core::prelude::PolarsResult<Self::Arrow> {
                    let mapped_values: Vec<Option<u32>> = self.values
                        .into_iter()
                        .map(|opt| opt.map(Self::Element::rust_idx_to_arrow_idx))
                        .collect();
                    $crate::polars_arrow::array::DictionaryArray::<u32>::try_new(
                        self._data_type,
                        $crate::polars_arrow::array::PrimitiveArray::<u32>::from(mapped_values),
                        Box::new($crate::polars_arrow::array::Utf8Array::<i32>::from(
                            vec![$(Some(stringify!($identifier))),*],
                        ))
                    )
                }
            }

            impl $crate::IntoArrow for $element_type {
                type Buffer = [<$element_type Buffer>];
            }

            impl $crate::FromArrow for $element_type {

                fn from_arrow(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Self> {
                    let dict_array = array.as_any().downcast_ref::<$crate::polars_arrow::array::DictionaryArray<u32>>().unwrap();
                    dict_array.keys().iter().map(|opt| $element_type::from_arrow_idx(*opt.unwrap_or(&0))).collect()
                }

                fn from_arrow_opt(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Option<Self>> {
                    let dict_array = array.as_any().downcast_ref::<$crate::polars_arrow::array::DictionaryArray<u32>>().unwrap();
                    dict_array.keys().iter().map(|opt| opt.copied().map(|k| $element_type::from_arrow_idx(k))).collect()
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{ArrowBuffer, IntoArrow};
    use polars_arrow::array::Utf8Array;

    #[test]
    fn test_impl_enum_buffer() {
        pub enum SampleEnum {
            ITEM1 = 1,
            ITEM2 = 2,
        }
        impl_enum_buffer!(SampleEnum, [(ITEM1, 1), (ITEM2, 2)]);
        let mut buffer = SampleEnum::new_buffer(1);
        buffer.push(SampleEnum::ITEM1);
        buffer.push(SampleEnum::ITEM2);
        buffer.push_null();
        let dict_array = buffer.to_arrow().unwrap();
        assert_eq!(dict_array.len(), 3);

        let keys: Vec<Option<u32>> = dict_array.keys().iter().map(|opt| opt.copied()).collect();
        assert_eq!(keys, vec![Some(0), Some(1), None]);

        let values = dict_array
            .values()
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();
        let values_vec: Vec<String> = values.iter().map(|opt| opt.unwrap().to_string()).collect();
        assert_eq!(values_vec, vec!["ITEM1".to_string(), "ITEM2".to_string()]);
    }
}
