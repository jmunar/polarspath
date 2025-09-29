/// A macro to simplify the construction of nested DataTypeOpt types.
///
/// This macro allows you to build complex nested DataTypeOpt structures
/// without the verbose boxing and nesting syntax.
///
/// # Examples
///
/// ```rust
/// use structpath_types::{DataTypeOpt, data_type_opt};
/// use indexmap::IndexMap;
///
/// // Simple types
/// let t1 = data_type_opt!(String);
/// assert_eq!(t1, DataTypeOpt::String);
///
/// // Optional types
/// let t2 = data_type_opt!(Option, String);
/// assert_eq!(t2, DataTypeOpt::Option(Box::new(DataTypeOpt::String)));
///
/// // List types
/// let t3 = data_type_opt!(List, String);
/// assert_eq!(t3, DataTypeOpt::List(Box::new(DataTypeOpt::String)));
///
/// // Struct types
/// let t4 = data_type_opt!(Struct([("field1", DataTypeOpt::String)]));
/// assert_eq!(t4, DataTypeOpt::Struct(IndexMap::from([("field1".into(), DataTypeOpt::String)])));
///
/// // Complex nested types
/// let t5 = data_type_opt!(Option, List, Option, String);
/// assert_eq!(t5, DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(Box::new(DataTypeOpt::String)))))));
///
/// // Enums of string literals
/// let t6 = data_type_opt!(Enum([("SampleEnum", 0)]));
/// assert_eq!(t6, DataTypeOpt::Enum(IndexMap::from([("SampleEnum".into(), 0)])));
/// ```
#[macro_export]
macro_rules! data_type_opt {
    // Base case: simple types without parameters
    (String) => { $crate::DataTypeOpt::String };
    (Int32) => { $crate::DataTypeOpt::Int32 };
    (Int64) => { $crate::DataTypeOpt::Int64 };
    (Float64) => { $crate::DataTypeOpt::Float64 };
    (Boolean) => { $crate::DataTypeOpt::Boolean };

    // Special cases with parameters
    (StructFuture($name:expr)) => { $crate::DataTypeOpt::StructFuture($name) };
    (Struct($fields:expr)) => {
        $crate::DataTypeOpt::Struct(::indexmap::IndexMap::from_iter($fields.into_iter().map(|(k, v)| (k.into(), v))))
    };
    (Enum($items:expr)) => {
        $crate::DataTypeOpt::Enum(::indexmap::IndexMap::from_iter($items.into_iter().map(|(k, v)| (k.into(), v))))
    };

    // Recursive cases for wrapping types
    (Option, $($rest:tt)*) => {
        $crate::DataTypeOpt::Option(Box::new($crate::data_type_opt!($($rest)*)))
    };
    (List, $($rest:tt)*) => {
        $crate::DataTypeOpt::List(Box::new($crate::data_type_opt!($($rest)*)))
    };
}

/// A macro to simplify the construction of (String, DataTypeOpt) tuples for struct fields.
///
/// This macro combines a field name with a DataTypeOpt type specification,
/// automatically converting the field name to a String and applying the data_type_opt! macro
/// to the type specification.
///
/// # Examples
///
/// ```rust
/// use structpath_types::{DataTypeOpt, data_type_opt, field_type_opt};
/// use indexmap::IndexMap;
///
/// // Simple field
/// let field1: (String, DataTypeOpt) = field_type_opt!("username", String);
/// assert_eq!(field1, ("username".into(), DataTypeOpt::String));
///
/// // Optional field
/// let field2: (String, DataTypeOpt) = field_type_opt!("age", Option, Int64);
/// assert_eq!(field2, ("age".into(), DataTypeOpt::Option(Box::new(DataTypeOpt::Int64))));
///
/// // List field
/// let field3: (String, DataTypeOpt) = field_type_opt!("tags", List, String);
/// assert_eq!(field3, ("tags".into(), DataTypeOpt::List(Box::new(DataTypeOpt::String))));
///
/// // Complex nested field
/// let field4: (String, DataTypeOpt) = field_type_opt!("optional_tags", Option, List, Option, String);
/// assert_eq!(field4, ("optional_tags".into(),
///     DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(Box::new(DataTypeOpt::String))))))));
/// ```
#[macro_export]
macro_rules! field_type_opt {
    ($field_name:expr, $($type_spec:tt)*) => {
        ($field_name.into(), $crate::data_type_opt!($($type_spec)*))
    };
}

#[cfg(test)]
mod tests {
    use super::super::DataTypeOpt;
    use indexmap::IndexMap;

    #[test]
    fn test_data_type_opt_macro_simple_types() {
        assert_eq!(data_type_opt!(String), DataTypeOpt::String);
        assert_eq!(data_type_opt!(Int32), DataTypeOpt::Int32);
        assert_eq!(data_type_opt!(Int64), DataTypeOpt::Int64);
        assert_eq!(data_type_opt!(Float64), DataTypeOpt::Float64);
        assert_eq!(data_type_opt!(Boolean), DataTypeOpt::Boolean);
    }

    #[test]
    fn test_data_type_opt_macro_special_types() {
        assert_eq!(
            data_type_opt!(Enum([("SampleEnum", 0)])),
            DataTypeOpt::Enum(IndexMap::from([("SampleEnum".into(), 0)]))
        );
        assert_eq!(
            data_type_opt!(StructFuture("SomeStruct")),
            DataTypeOpt::StructFuture("SomeStruct")
        );
    }

    #[test]
    fn test_data_type_opt_macro_enum_multiple_values() {
        assert_eq!(
            data_type_opt!(Enum([("Option1", 1), ("Option2", 2), ("Option3", 3)])),
            DataTypeOpt::Enum(IndexMap::from([
                ("Option1".into(), 1),
                ("Option2".into(), 2),
                ("Option3".into(), 3)
            ]))
        );
    }

    #[test]
    fn test_data_type_opt_macro_single_wrapper() {
        assert_eq!(
            data_type_opt!(Option, String),
            DataTypeOpt::Option(Box::new(DataTypeOpt::String))
        );
        assert_eq!(
            data_type_opt!(List, String),
            DataTypeOpt::List(Box::new(DataTypeOpt::String))
        );
    }

    #[test]
    fn test_data_type_opt_macro_nested_types() {
        // Option(List(String))
        assert_eq!(
            data_type_opt!(Option, List, String),
            DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::String))))
        );

        // List(Option(String))
        assert_eq!(
            data_type_opt!(List, Option, String),
            DataTypeOpt::List(Box::new(DataTypeOpt::Option(Box::new(DataTypeOpt::String))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_complex_nesting() {
        // The exact example from the user's request: Option(List(Option(String)))
        assert_eq!(
            data_type_opt!(Option, List, Option, String),
            DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(
                Box::new(DataTypeOpt::String)
            )))))
        );

        // Test with other types too
        assert_eq!(
            data_type_opt!(Option, List, Option, Int64),
            DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(
                Box::new(DataTypeOpt::Int64)
            )))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_with_enum() {
        // Option(List(Enum([("opt1", 1), ("opt2", 3)])))
        assert_eq!(
            data_type_opt!(Option, List, Enum([("opt1", 1), ("opt2", 3)])),
            DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Enum(
                IndexMap::from_iter([("opt1".into(), 1), ("opt2".into(), 3)])
            )))))
        );
    }

    #[test]
    fn test_data_type_opt_macro_with_struct() {
        use indexmap::IndexMap;

        // Simple Struct
        assert_eq!(
            data_type_opt!(Struct([("field1", DataTypeOpt::String)])),
            DataTypeOpt::Struct(IndexMap::from([("field1".into(), DataTypeOpt::String)]))
        );

        // Nested with Struct
        assert_eq!(
            data_type_opt!(
                Option,
                List,
                Option,
                Struct([("subf_string", DataTypeOpt::String)])
            ),
            DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(
                Box::new(DataTypeOpt::Struct(IndexMap::from([(
                    "subf_string".into(),
                    DataTypeOpt::String
                )])))
            )))))
        );
    }

    #[test]
    fn test_field_type_opt_macro() {
        // Test simple types
        let result1: (String, DataTypeOpt) = field_type_opt!("name", String);
        assert_eq!(result1, ("name".into(), DataTypeOpt::String));

        let result2: (String, DataTypeOpt) = field_type_opt!("age", Int32);
        assert_eq!(result2, ("age".into(), DataTypeOpt::Int32));

        let result3: (String, DataTypeOpt) = field_type_opt!("big_age", Int64);
        assert_eq!(result3, ("big_age".into(), DataTypeOpt::Int64));

        // Test optional types
        let result4: (String, DataTypeOpt) = field_type_opt!("optional_name", Option, String);
        assert_eq!(
            result4,
            (
                "optional_name".into(),
                DataTypeOpt::Option(Box::new(DataTypeOpt::String))
            )
        );

        // Test list types
        let result5: (String, DataTypeOpt) = field_type_opt!("tags", List, String);
        assert_eq!(
            result5,
            (
                "tags".into(),
                DataTypeOpt::List(Box::new(DataTypeOpt::String))
            )
        );

        // Test complex nested types
        let result6: (String, DataTypeOpt) =
            field_type_opt!("complex", Option, List, Option, Int32);
        assert_eq!(
            result6,
            (
                "complex".into(),
                DataTypeOpt::Option(Box::new(DataTypeOpt::List(Box::new(DataTypeOpt::Option(
                    Box::new(DataTypeOpt::Int32)
                )))))
            )
        );

        // Test with struct
        use indexmap::IndexMap;
        let result7: (String, DataTypeOpt) =
            field_type_opt!("nested", Struct([("subfield", DataTypeOpt::String)]));
        assert_eq!(
            result7,
            (
                "nested".into(),
                DataTypeOpt::Struct(IndexMap::from([("subfield".into(), DataTypeOpt::String)]))
            )
        );

        // Test with enum types
        let result8: (String, DataTypeOpt) =
            field_type_opt!("enum_field", Enum([("Value1", 1), ("Value2", 2)]));
        assert_eq!(
            result8,
            (
                "enum_field".into(),
                DataTypeOpt::Enum(IndexMap::from_iter([
                    ("Value1".into(), 1),
                    ("Value2".into(), 2)
                ]))
            )
        );
    }
}
