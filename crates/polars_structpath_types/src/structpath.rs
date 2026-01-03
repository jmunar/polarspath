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
        <$other as $crate::IntoArrow>::Buffer
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
        $crate::impl_struct_buffer!(
            @inner
            ($struct_type, 0usize, [$(($field_name, $field_type))*], [])
        );
    };
    (
        @inner
        ($struct_type:ty, $current_index:expr, [], [$(($index:expr, $field_name:ident, $field_type:ty))*])
    ) => {
        $crate::impl_struct_buffer!(
            @generate
            $struct_type
            [$(($index, $field_name, $field_type))*]
        );
    };
    (
        @inner
        ($struct_type:ty, $current_index:expr, [($field_name:ident, $field_type:ty) $(($rest_field_name:ident, $rest_field_type:ty))*], [$($accum:tt)*])
    ) => {
        $crate::impl_struct_buffer!(
            @inner
            ($struct_type, ($current_index + 1usize), [$(($rest_field_name, $rest_field_type))*], [$($accum)* ($current_index, $field_name, $field_type)])
        );
    };
    (
        @generate
        $struct_type:ty
        [$(($index:expr, $field_name:ident, $field_type:ty))*]
    ) => {
        $crate::paste::paste! {

            pub struct [<$struct_type Buffer>] {
                $(
                    [<buffer_ $field_name>]: $crate::impl_struct_field_buffer_type!($field_type),
                )*
                _validity: Vec<bool>,
                _data_type: $crate::polars_arrow::datatypes::ArrowDataType,
            }

            impl $crate::ArrowBuffer for [<$struct_type Buffer>] {
                type Element = $struct_type;
                type Arrow = $crate::polars_arrow::array::StructArray;

                fn data_type(&self) -> &$crate::polars_arrow::datatypes::ArrowDataType {
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
                            $crate::polars_arrow::datatypes::Field::new(
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
                        _data_type: $crate::polars_arrow::datatypes::ArrowDataType::Struct(fields),
                    }
                }

                fn push(&mut self, value: impl Into<Self::Element>) {
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

                fn to_arrow(self) -> $crate::polars_core::prelude::PolarsResult<Self::Arrow> {
                    let data_type = self.data_type().clone();
                    let arrays: Vec<Box<dyn $crate::polars_arrow::array::Array>> = vec![
                        $(
                            Box::new(self.[<buffer_ $field_name>].to_arrow()?),
                        )*
                    ];

                    $crate::polars_arrow::array::StructArray::try_new(
                        data_type,
                        self._validity.len(),
                        arrays,
                        Some($crate::polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
                    )
                }
            }

            impl $crate::IntoArrow for $struct_type {
                type Buffer = [<$struct_type Buffer>];
            }

            impl $crate::FromArrow for $struct_type {

                fn from_arrow(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Self> {
                    let struct_array = array
                        .as_any()
                        .downcast_ref::<$crate::polars_arrow::array::StructArray>()
                        .unwrap();
                    let (_, _, arrays, _) = struct_array.clone().into_data();
                    $(
                        let [<field_ $field_name>]: Vec<$field_type> = <$field_type as $crate::FromArrow>::from_arrow(arrays[$index].clone());
                    )*
                    let len = struct_array.len();
                    (0..len)
                        .map(|i| {
                            $struct_type {
                                $(
                                    $field_name: [<field_ $field_name>][i].clone(),
                                )*
                            }
                        })
                        .collect()
                }

                fn from_arrow_opt(array: Box<dyn $crate::polars_arrow::array::Array>) -> Vec<Option<Self>> {
                    let struct_array = array
                        .as_any()
                        .downcast_ref::<$crate::polars_arrow::array::StructArray>()
                        .unwrap();
                    let (_, _, arrays, validity) = struct_array.clone().into_data();
                    $(
                        let [<field_ $field_name>]: Vec<$field_type> = <$field_type as $crate::FromArrow>::from_arrow(arrays[$index].clone());
                    )*
                    let len = struct_array.len();
                    (0..len)
                        .map(|i| {
                            match validity {
                                Some(ref validity) => {
                                    if validity.get(i).unwrap() {
                                        Some($struct_type {
                                            $(
                                                $field_name: [<field_ $field_name>][i].clone(),
                                            )*
                                        })
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    Some($struct_type {
                                        $(
                                            $field_name: [<field_ $field_name>][i].clone(),
                                        )*
                                    })
                                }
                            }
                        })
                        .collect()
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{ArrowBuffer, IntoArrow};
    use polars_arrow::array::{ListArray, Utf8Array};

    #[test]
    fn test_impl_struct_buffer() {
        #[derive(Debug, Clone, PartialEq)]
        pub struct SampleStruct {
            req: String,
            opt: Option<String>,
            req_vec_req_item: Vec<String>,
            req_vec_opt_item: Vec<Option<String>>,
            opt_vec_req_item: Option<Vec<String>>,
            opt_vec_opt_item: Option<Vec<Option<String>>>,
        }

        impl_struct_buffer!(
            SampleStruct,
            [
                (req, String),
                (opt, Option<String>),
                (req_vec_req_item, Vec<String>),
                (req_vec_opt_item, Vec<Option<String>>),
                (opt_vec_req_item, Option<Vec<String>>),
                (opt_vec_opt_item, Option<Vec<Option<String>>>),
            ]
        );
        let mut buffer = SampleStruct::new_buffer(1);
        buffer.push(SampleStruct {
            req: "test1a".to_string(),
            opt: Some("test2a".to_string()),
            req_vec_req_item: vec!["test3a".to_string()],
            req_vec_opt_item: vec![Some("test4a".to_string())],
            opt_vec_req_item: Some(vec!["test5a".to_string()]),
            opt_vec_opt_item: Some(vec![Some("test6a".to_string())]),
        });
        buffer.push(SampleStruct {
            req: "test1b".to_string(),
            opt: None,
            req_vec_req_item: vec!["test3b".to_string()],
            req_vec_opt_item: vec![None],
            opt_vec_req_item: None,
            opt_vec_opt_item: None,
        });

        let struct_array = buffer.to_arrow().unwrap();
        assert_eq!(struct_array.len(), 2);

        let field_names: Vec<&str> = struct_array
            .fields()
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        assert_eq!(
            field_names,
            vec![
                "req",
                "opt",
                "req_vec_req_item",
                "req_vec_opt_item",
                "opt_vec_req_item",
                "opt_vec_opt_item"
            ]
        );

        let req_arr = struct_array.values()[0]
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();
        let req_values: Vec<String> = req_arr.iter().map(|opt| opt.unwrap().to_string()).collect();
        assert_eq!(req_values.get(0).unwrap(), "test1a");
        assert_eq!(req_values.get(1).unwrap(), "test1b");

        let opt_arr = struct_array.values()[1]
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();
        let opt_values: Vec<Option<String>> = opt_arr
            .iter()
            .map(|opt| opt.map(|v| v.to_string()))
            .collect();
        assert_eq!(
            opt_values.get(0).unwrap().clone(),
            Some("test2a".to_string())
        );
        assert_eq!(opt_values.get(1).unwrap().clone(), None::<String>);

        let req_vec_req_item_arr = struct_array.values()[2]
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap();
        let req_vec_req_item_values: Vec<Vec<String>> = req_vec_req_item_arr
            .iter()
            .map(|list_opt| {
                list_opt
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Utf8Array<i32>>()
                    .unwrap()
                    .iter()
                    .map(|value_opt| value_opt.unwrap().to_string())
                    .collect()
            })
            .collect();
        assert_eq!(
            req_vec_req_item_values.get(0).unwrap().clone(),
            vec!["test3a".to_string()]
        );
        assert_eq!(
            req_vec_req_item_values.get(1).unwrap().clone(),
            vec!["test3b".to_string()]
        );

        let req_vec_opt_item_arr = struct_array.values()[3]
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap();
        let req_vec_opt_item_values: Vec<Vec<Option<String>>> = req_vec_opt_item_arr
            .iter()
            .map(|list_opt| {
                list_opt
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Utf8Array<i32>>()
                    .unwrap()
                    .iter()
                    .map(|value_opt| value_opt.map(|v| v.to_string()))
                    .collect()
            })
            .collect();
        assert_eq!(
            req_vec_opt_item_values.get(0).unwrap().clone(),
            vec![Some("test4a".to_string())]
        );
        assert_eq!(req_vec_opt_item_values.get(1).unwrap().clone(), vec![None]);

        let opt_vec_req_item_arr = struct_array.values()[4]
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap();
        let opt_vec_req_item_values: Vec<Option<Vec<String>>> = opt_vec_req_item_arr
            .iter()
            .map(|list_opt| {
                list_opt.map(|list| {
                    list.as_any()
                        .downcast_ref::<Utf8Array<i32>>()
                        .unwrap()
                        .iter()
                        .map(|value_opt| value_opt.unwrap().to_string())
                        .collect()
                })
            })
            .collect();
        assert_eq!(
            opt_vec_req_item_values.get(0).unwrap().clone(),
            Some(vec!["test5a".to_string()])
        );
        assert_eq!(
            opt_vec_req_item_values.get(1).unwrap().clone(),
            None::<Vec<String>>
        );

        let opt_vec_opt_item_arr = struct_array.values()[5]
            .as_any()
            .downcast_ref::<ListArray<i32>>()
            .unwrap();
        let opt_vec_opt_item_values: Vec<Option<Vec<Option<String>>>> = opt_vec_opt_item_arr
            .iter()
            .map(|list_opt| {
                list_opt.map(|list| {
                    list.as_any()
                        .downcast_ref::<Utf8Array<i32>>()
                        .unwrap()
                        .iter()
                        .map(|value_opt| value_opt.map(|v| v.to_string()))
                        .collect()
                })
            })
            .collect();
        assert_eq!(
            opt_vec_opt_item_values.get(0).unwrap().clone(),
            Some(vec![Some("test6a".to_string())])
        );
        assert_eq!(
            opt_vec_opt_item_values.get(1).unwrap().clone(),
            None::<Vec<Option<String>>>
        );
    }
}
