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

            impl $crate::ArrowBuffer for [<$element_type Buffer>] {
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
                    let _ = dict_index; // Suppress unused assignment warning (value used in macro expansion)
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

            impl $crate::HasArrowBuffer for $element_type {
                type BufferType = [<$element_type Buffer>];
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{ArrowBuffer, HasArrowBuffer};
    use polars_arrow::array::{DictionaryArray, Utf8Array};

    #[test]
    fn test_impl_enum_buffer() {
        pub enum SampleEnum {
            ITEM1 = 1,
            ITEM2 = 2,
        }
        impl_enum_buffer!(SampleEnum, [("ITEM1", 1), ("ITEM2", 2)]);
        let mut buffer = SampleEnum::new_buffer(1);
        buffer.push(SampleEnum::ITEM1);
        buffer.push(SampleEnum::ITEM2);
        buffer.push_null();
        let array = buffer.to_arrow().unwrap();
        assert_eq!(array.len(), 3);

        let dict_array = array
            .as_any()
            .downcast_ref::<DictionaryArray<u32>>()
            .unwrap();
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
