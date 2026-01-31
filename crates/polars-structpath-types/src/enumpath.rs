use polars_arrow::array::{Array, DictionaryArray, PrimitiveArray};

/// Helper trait for extracting keys from dictionary arrays with different key types.
/// This allows us to handle DictionaryArray<u8>, DictionaryArray<u16>, and DictionaryArray<u32>.
pub trait DictionaryKeyExtractor {
    /// Extract keys as u32 values (the common denominator for all key types)
    fn extract_keys(array: &dyn Array) -> Option<Vec<Option<u32>>>;
}

impl DictionaryKeyExtractor for u8 {
    fn extract_keys(array: &dyn Array) -> Option<Vec<Option<u32>>> {
        array
            .as_any()
            .downcast_ref::<DictionaryArray<u8>>()
            .map(|dict| {
                dict.keys()
                    .iter()
                    .map(|opt| opt.map(|k| *k as u32))
                    .collect()
            })
    }
}

impl DictionaryKeyExtractor for u16 {
    fn extract_keys(array: &dyn Array) -> Option<Vec<Option<u32>>> {
        array
            .as_any()
            .downcast_ref::<DictionaryArray<u16>>()
            .map(|dict| {
                dict.keys()
                    .iter()
                    .map(|opt| opt.map(|k| *k as u32))
                    .collect()
            })
    }
}

impl DictionaryKeyExtractor for u32 {
    fn extract_keys(array: &dyn Array) -> Option<Vec<Option<u32>>> {
        array
            .as_any()
            .downcast_ref::<DictionaryArray<u32>>()
            .map(|dict| dict.keys().iter().map(|opt| opt.copied()).collect())
    }
}

/// Extract dictionary keys from an array, trying different key types (u8, u16, u32).
/// Returns the keys as Vec<Option<u32>> for uniform handling.
pub fn extract_dictionary_keys(array: &dyn Array) -> Vec<Option<u32>> {
    // Try u32 first (most common)
    if let Some(keys) = <u32 as DictionaryKeyExtractor>::extract_keys(array) {
        return keys;
    }
    // Try u8
    if let Some(keys) = <u8 as DictionaryKeyExtractor>::extract_keys(array) {
        return keys;
    }
    // Try u16
    if let Some(keys) = <u16 as DictionaryKeyExtractor>::extract_keys(array) {
        return keys;
    }
    // Also try PrimitiveArray<u32> directly (polars might return just the keys)
    if let Some(prim_array) = array.as_any().downcast_ref::<PrimitiveArray<u32>>() {
        return prim_array.iter().map(|opt| opt.copied()).collect();
    }
    if let Some(prim_array) = array.as_any().downcast_ref::<PrimitiveArray<u8>>() {
        return prim_array
            .iter()
            .map(|opt| opt.map(|k| *k as u32))
            .collect();
    }
    if let Some(prim_array) = array.as_any().downcast_ref::<PrimitiveArray<u16>>() {
        return prim_array
            .iter()
            .map(|opt| opt.map(|k| *k as u32))
            .collect();
    }
    panic!(
        "Unsupported dictionary array type: {:?}. Expected DictionaryArray<u8/u16/u32> or PrimitiveArray<u8/u16/u32>",
        array.dtype()
    );
}

/// Try to extract dictionary string values from an array, resolving keys to their string representations.
/// This function handles Polars' categorical encoding which may re-index dictionary keys.
/// Returns Some(Vec<Option<String>>) if successful, None if the array is not a dictionary.
pub fn try_extract_dictionary_values(array: &dyn Array) -> Option<Vec<Option<String>>> {
    use polars_arrow::array::Utf8Array;
    use polars_arrow::array::Utf8ViewArray;

    // Helper to resolve keys to strings for u32 keys
    fn resolve_u32_keys_to_strings(dict: &DictionaryArray<u32>) -> Option<Vec<Option<String>>> {
        let values = dict.values();

        // Try Utf8Array<i32> first
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i32>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }

        // Try Utf8Array<i64>
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i64>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }

        // Try Utf8ViewArray (polars internal)
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8ViewArray>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }

        None
    }

    fn resolve_u8_keys_to_strings(dict: &DictionaryArray<u8>) -> Option<Vec<Option<String>>> {
        let values = dict.values();

        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i32>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i64>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8ViewArray>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        None
    }

    fn resolve_u16_keys_to_strings(dict: &DictionaryArray<u16>) -> Option<Vec<Option<String>>> {
        let values = dict.values();

        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i32>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8Array<i64>>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        if let Some(utf8_arr) = values.as_any().downcast_ref::<Utf8ViewArray>() {
            return Some(
                dict.keys()
                    .iter()
                    .map(|opt_key| {
                        opt_key.and_then(|k| utf8_arr.get(*k as usize).map(|s| s.to_string()))
                    })
                    .collect(),
            );
        }
        None
    }

    // Try DictionaryArray<u32>
    if let Some(dict) = array.as_any().downcast_ref::<DictionaryArray<u32>>() {
        return resolve_u32_keys_to_strings(dict);
    }
    // Try DictionaryArray<u8>
    if let Some(dict) = array.as_any().downcast_ref::<DictionaryArray<u8>>() {
        return resolve_u8_keys_to_strings(dict);
    }
    // Try DictionaryArray<u16>
    if let Some(dict) = array.as_any().downcast_ref::<DictionaryArray<u16>>() {
        return resolve_u16_keys_to_strings(dict);
    }

    // Not a dictionary array
    None
}

/// Extract dictionary string values from an array, resolving keys to their string representations.
/// This function handles Polars' categorical encoding which may re-index dictionary keys.
/// Returns Vec<Option<String>> where each element is the string value for that row.
/// Panics if the array is not a dictionary type.
pub fn extract_dictionary_values(array: &dyn Array) -> Vec<Option<String>> {
    try_extract_dictionary_values(array).unwrap_or_else(|| {
        panic!(
            "Unsupported array type for dictionary value extraction: {:?}",
            array.dtype()
        )
    })
}

/// Internal macro for converting Rust enum discriminant indices to Arrow dictionary indices.
///
/// This macro is used internally by `impl_enum_buffer!` to handle the mapping between
/// Rust enum discriminants (which may have gaps) and Arrow dictionary indices (which are dense).
#[macro_export]
#[doc(hidden)]
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

/// Internal macro for converting Arrow dictionary indices back to Rust enum discriminant indices.
///
/// This macro is used internally by `impl_enum_buffer!` to handle the reverse mapping from
/// Arrow dictionary indices to Rust enum discriminants.
#[macro_export]
#[doc(hidden)]
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
/// This macro generates a complete Arrow buffer implementation for Rust enums, converting
/// them to Arrow `DictionaryArray` with UTF-8 string keys representing enum variant names.
///
/// # Usage
///
/// ```rust
/// use polars_structpath_types::impl_enum_buffer;
///
/// pub enum Status {
///     Active = 1,
///     Inactive = 2,
/// }
///
/// impl_enum_buffer!(Status, [(Active, 1), (Inactive, 2)]);
/// ```
///
/// # Generated Code
///
/// This macro generates:
/// - `StatusBuffer` struct implementing `ArrowBuffer`
/// - Helper methods: `from_arrow_idx()`, `rust_idx_to_arrow_idx()`
/// - `IntoArrow` implementation for `Status`
/// - `FromArrow` implementation for `Status`
///
/// # Parameters
///
/// - `$element_type`: The enum type name
/// - `[($identifier, $index), ...]`: List of enum variants with their discriminant values
///
/// # Notes
///
/// - Enum discriminants can have gaps (e.g., `ITEM1 = 1, ITEM3 = 3`)
/// - The macro handles mapping between Rust discriminants and dense Arrow dictionary indices
/// - Enum variant names are used as dictionary keys in the Arrow array
#[macro_export]
macro_rules! impl_enum_buffer {
    (
        $element_type:ident,
        [$(($identifier:ident, $index:expr)),* $(,)?]
    ) => {
        $crate::paste::paste! {

            impl $element_type {
                // First variant's discriminant, used as default for null values
                const FIRST_VARIANT_IDX: i32 = {
                    let discriminants: &[i32] = &[$($index),*];
                    discriminants[0]
                };

                pub fn from_rust_idx(rust_idx: i32) -> Self {
                    match rust_idx {
                        $($index => Self::$identifier,)*
                        _ => panic!("Invalid rust index: {}", rust_idx),
                    }
                }

                pub fn from_arrow_idx(arrow_idx: u32) -> Self {
                    Self::from_rust_idx(Self::arrow_idx_to_rust_idx(arrow_idx))
                }

                pub fn from_name(name: &str) -> Self {
                    match name {
                        $(stringify!($identifier) => Self::$identifier,)*
                        _ => panic!("Invalid enum variant name: {}", name),
                    }
                }

                pub fn rust_idx_to_arrow_idx(rust_idx: i32) -> u32 {
                    $crate::rust_idx_to_arrow_idx!(rust_idx, [$($index),*])
                }

                pub fn arrow_idx_to_rust_idx(arrow_idx: u32) -> i32 {
                    $crate::arrow_idx_to_rust_idx!(arrow_idx, [$($index),*])
                }
            }

            pub struct [<$element_type Buffer>] {
                values: Vec<Option<i32>>,
                _validity: Vec<bool>,
                _data_type: $crate::polars_arrow::datatypes::ArrowDataType,
            }

            impl $crate::ArrowBuffer for [<$element_type Buffer>] {
                type Element = $element_type;
                // Use PrimitiveArray<i32> to store rust discriminant directly
                // This avoids Polars Categorical re-indexing issues
                type Arrow = $crate::polars_arrow::array::PrimitiveArray<i32>;

                fn data_type(&self) -> &$crate::polars_arrow::datatypes::ArrowDataType {
                    &self._data_type
                }

                fn validity(&self) -> &Vec<bool> {
                    &self._validity
                }

                fn new(nrows: usize) -> Self {
                    Self {
                        values: Vec::with_capacity(nrows),
                        _validity: Vec::with_capacity(nrows),
                        _data_type: $crate::polars_arrow::datatypes::ArrowDataType::Int32,
                    }
                }

                fn push(&mut self, value: impl Into<Self::Element>) {
                    let value = value.into();
                    // Store the rust discriminant directly
                    self.values.push(Some(value as i32));
                    self._validity.push(true);
                }

                fn push_null(&mut self) {
                    self.values.push(None);
                    self._validity.push(false);
                }

                fn to_arrow(self) -> $crate::polars_core::prelude::PolarsResult<Self::Arrow> {
                    // Return PrimitiveArray<i32> with rust discriminant values
                    Ok($crate::polars_arrow::array::PrimitiveArray::<i32>::from(self.values))
                }
            }

            impl $crate::IntoArrow for $element_type {
                type Buffer = [<$element_type Buffer>];
            }

            impl $crate::FromArrow for $element_type {

                fn from_arrow(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Self> {
                    // Extract i32 values (rust discriminants)
                    if let Some(prim) = array.as_any().downcast_ref::<$crate::polars_arrow::array::PrimitiveArray<i32>>() {
                        return prim.iter()
                            .map(|opt| $element_type::from_rust_idx(opt.copied().unwrap_or($element_type::FIRST_VARIANT_IDX)))
                            .collect();
                    }
                    // Also try u32 (Polars may convert i32 to u32)
                    if let Some(prim) = array.as_any().downcast_ref::<$crate::polars_arrow::array::PrimitiveArray<u32>>() {
                        return prim.iter()
                            .map(|opt| $element_type::from_rust_idx(opt.copied().unwrap_or($element_type::FIRST_VARIANT_IDX as u32) as i32))
                            .collect();
                    }
                    // Fall back to dictionary string values for backwards compatibility
                    if let Some(values) = $crate::try_extract_dictionary_values(array.as_ref()) {
                        return values.into_iter()
                            .map(|opt| {
                                match opt {
                                    Some(name) => $element_type::from_name(&name),
                                    None => $element_type::from_rust_idx($element_type::FIRST_VARIANT_IDX),
                                }
                            })
                            .collect();
                    }
                    panic!("Unsupported array type for enum: {:?}", array.dtype());
                }

                fn from_arrow_opt(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Option<Self>> {
                    // Extract i32 values (rust discriminants)
                    if let Some(prim) = array.as_any().downcast_ref::<$crate::polars_arrow::array::PrimitiveArray<i32>>() {
                        return prim.iter()
                            .map(|opt| opt.map(|v| $element_type::from_rust_idx(*v)))
                            .collect();
                    }
                    // Also try u32 (Polars may convert i32 to u32)
                    if let Some(prim) = array.as_any().downcast_ref::<$crate::polars_arrow::array::PrimitiveArray<u32>>() {
                        return prim.iter()
                            .map(|opt| opt.map(|v| $element_type::from_rust_idx(*v as i32)))
                            .collect();
                    }
                    // Fall back to dictionary string values for backwards compatibility
                    if let Some(values) = $crate::try_extract_dictionary_values(array.as_ref()) {
                        return values.into_iter()
                            .map(|opt| opt.map(|name| $element_type::from_name(&name)))
                            .collect();
                    }
                    panic!("Unsupported array type for enum: {:?}", array.dtype());
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{ArrowBuffer, FromArrow, IntoArrow};

    #[test]
    fn test_impl_enum_buffer() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SampleEnum {
            ITEM1 = -1,
            ITEM2 = 2,
        }
        impl_enum_buffer!(SampleEnum, [(ITEM1, -1), (ITEM2, 2)]);

        // Test buffer creation and push
        let mut buffer = SampleEnum::new_buffer(3);
        buffer.push(SampleEnum::ITEM1);
        buffer.push(SampleEnum::ITEM2);
        buffer.push_null();

        // Convert to arrow array (now PrimitiveArray<i32>)
        let prim_array = buffer.to_arrow().unwrap();
        assert_eq!(prim_array.len(), 3);

        // Verify the values are the rust discriminants
        let values: Vec<Option<i32>> = prim_array.iter().map(|opt| opt.copied()).collect();
        assert_eq!(values, vec![Some(-1), Some(2), None]);

        // Test roundtrip via FromArrow
        let recovered = SampleEnum::from_arrow_opt(Box::new(prim_array));
        assert_eq!(
            recovered,
            vec![Some(SampleEnum::ITEM1), Some(SampleEnum::ITEM2), None]
        );
    }
}
