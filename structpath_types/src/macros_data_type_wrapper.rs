/// A macro to simplify the construction of nested DataTypeWrapper types.
///
/// This macro allows you to build complex nested DataTypeWrapper structures
/// without the verbose boxing and nesting syntax.
///
/// # Examples
///
/// ```rust
/// use structpath_types::{DataTypeOpt, DataTypeWrapper, data_type_wrapper};
/// use indexmap::IndexMap;
///
/// // Simple types
/// let t1 = data_type_wrapper!(String);
/// assert_eq!(t1, DataTypeWrapper::new(DataTypeOpt::String));
///
/// // Optional types
/// let t2 = data_type_wrapper!(Option(String));
/// assert_eq!(t2, DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(DataTypeOpt::String)))));
///
/// // List types
/// let t3 = data_type_wrapper!(List(String));
/// assert_eq!(t3, DataTypeWrapper::new(DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::String)))));
///
/// // Struct types
/// let t4 = data_type_wrapper!(Struct([("field1", String)]));
/// assert_eq!(t4, DataTypeWrapper::new(DataTypeOpt::Struct(IndexMap::from([("field1".into(), DataTypeWrapper::new(DataTypeOpt::String))]))));
///
/// // Complex nested types
/// let t5 = data_type_wrapper!(Option(List(Option(String))));
/// assert_eq!(t5, DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(DataTypeOpt::String)))))))))));
///
/// // Enums of string literals
/// let t6 = data_type_wrapper!(Enum([("SampleEnum", 0)]));
/// assert_eq!(t6, DataTypeWrapper::new(DataTypeOpt::Enum(IndexMap::from([("SampleEnum".into(), 0)]))));
/// ```
#[macro_export]
macro_rules! data_type_wrapper {
    // Base case: simple types without parameters
    (String) => { $crate::DataTypeWrapper::new($crate::DataTypeOpt::String) };
    (Int32) => { $crate::DataTypeWrapper::new($crate::DataTypeOpt::Int32) };
    (Int64) => { $crate::DataTypeWrapper::new($crate::DataTypeOpt::Int64) };
    (Float64) => { $crate::DataTypeWrapper::new($crate::DataTypeOpt::Float64) };
    (Boolean) => { $crate::DataTypeWrapper::new($crate::DataTypeOpt::Boolean) };

    // Special cases with parameters
    (StructFuture($name:expr)) => {
        $crate::DataTypeWrapper::new($crate::DataTypeOpt::StructFuture($name))
    };
    (Struct([$(($field_name:expr, $($field_type:tt)*)),*])) => {
        $crate::DataTypeWrapper::new($crate::DataTypeOpt::Struct(::indexmap::IndexMap::from_iter([
            $(($field_name.into(), $crate::data_type_wrapper!($($field_type)*))),*
        ])))
    };
    (Enum($items:expr)) => {
        $crate::DataTypeWrapper::new($crate::DataTypeOpt::Enum(::indexmap::IndexMap::from_iter($items.into_iter().map(|(k, v)| (k.into(), v)))))
    };

    // Recursive cases for wrapping types
    (Option($($rest:tt)*)) => {
        $crate::DataTypeWrapper::new($crate::DataTypeOpt::Option(Box::new($crate::data_type_wrapper!($($rest)*))))
    };
    (List($($rest:tt)*)) => {
        $crate::DataTypeWrapper::new($crate::DataTypeOpt::List(Box::new($crate::data_type_wrapper!($($rest)*))))
    };
}

#[cfg(test)]
mod tests {
    use super::super::{DataTypeOpt, DataTypeWrapper};
    use indexmap::IndexMap;

    #[test]
    fn test_data_type_opt_macro_simple_types() {
        assert_eq!(
            data_type_wrapper!(String),
            DataTypeWrapper::new(DataTypeOpt::String)
        );
        assert_eq!(
            data_type_wrapper!(Int32),
            DataTypeWrapper::new(DataTypeOpt::Int32)
        );
        assert_eq!(
            data_type_wrapper!(Int64),
            DataTypeWrapper::new(DataTypeOpt::Int64)
        );
        assert_eq!(
            data_type_wrapper!(Float64),
            DataTypeWrapper::new(DataTypeOpt::Float64)
        );
        assert_eq!(
            data_type_wrapper!(Boolean),
            DataTypeWrapper::new(DataTypeOpt::Boolean)
        );
    }

    #[test]
    fn test_data_type_opt_macro_special_types() {
        assert_eq!(
            data_type_wrapper!(Enum([("SampleEnum", 0)])),
            DataTypeWrapper::new(DataTypeOpt::Enum(IndexMap::from([(
                "SampleEnum".into(),
                0
            )])))
        );
        assert_eq!(
            data_type_wrapper!(StructFuture("SomeStruct")),
            DataTypeWrapper::new(DataTypeOpt::StructFuture("SomeStruct"))
        );
    }

    #[test]
    fn test_data_type_opt_macro_enum_multiple_values() {
        assert_eq!(
            data_type_wrapper!(Enum([("Option1", 1), ("Option2", 2), ("Option3", 3)])),
            DataTypeWrapper::new(DataTypeOpt::Enum(IndexMap::from([
                ("Option1".into(), 1),
                ("Option2".into(), 2),
                ("Option3".into(), 3)
            ])))
        );
    }

    #[test]
    fn test_data_type_opt_macro_single_wrapper() {
        assert_eq!(
            data_type_wrapper!(Option(String)),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::String
            ))))
        );
        assert_eq!(
            data_type_wrapper!(List(String)),
            DataTypeWrapper::new(DataTypeOpt::List(Box::new(DataTypeWrapper::new(
                DataTypeOpt::String
            ))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_nested_types() {
        // Option(List(String))
        assert_eq!(
            data_type_wrapper!(Option(List(String))),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::String)))
            ))))
        );

        // List(Option(String))
        assert_eq!(
            data_type_wrapper!(List(Option(String))),
            DataTypeWrapper::new(DataTypeOpt::List(Box::new(DataTypeWrapper::new(
                DataTypeOpt::Option(Box::new(DataTypeWrapper::new(DataTypeOpt::String)))
            ))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_complex_nesting() {
        // The exact example from the user's request: Option(List(Option(String)))
        assert_eq!(
            data_type_wrapper!(Option(List(Option(String)))),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::Option(
                    Box::new(DataTypeWrapper::new(DataTypeOpt::String))
                ))))
            ))))
        );

        // Test with other types too
        assert_eq!(
            data_type_wrapper!(Option(List(Option(Int64)))),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::Option(
                    Box::new(DataTypeWrapper::new(DataTypeOpt::Int64))
                ))))
            ))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_with_enum() {
        assert_eq!(
            data_type_wrapper!(Option(List(Enum([("opt1", 1), ("opt2", 3)])))),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::Enum(
                    IndexMap::from_iter([("opt1".into(), 1), ("opt2".into(), 3)])
                ))))
            ))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_with_struct() {
        use indexmap::IndexMap;

        // Simple Struct
        assert_eq!(
            data_type_wrapper!(Struct([("field1", String)])),
            DataTypeWrapper::new(DataTypeOpt::Struct(IndexMap::from([(
                "field1".into(),
                DataTypeWrapper::new(DataTypeOpt::String)
            )])))
        );

        // Nested with Struct
        assert_eq!(
            data_type_wrapper!(Option(List(Option(Struct([("subf_string", String)]))))),
            DataTypeWrapper::new(DataTypeOpt::Option(Box::new(DataTypeWrapper::new(
                DataTypeOpt::List(Box::new(DataTypeWrapper::new(DataTypeOpt::Option(
                    Box::new(DataTypeWrapper::new(DataTypeOpt::Struct(IndexMap::from([
                        (
                            "subf_string".into(),
                            DataTypeWrapper::new(DataTypeOpt::String)
                        )
                    ]))))
                ))))
            ))))
        );
    }
}
