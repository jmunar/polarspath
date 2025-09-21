/// A macro to simplify the construction of nested DataType types.
///
/// This macro allows you to build complex nested DataType structures
/// without the verbose boxing and nesting syntax. Unlike `data_type_opt!`,
/// this macro doesn't handle `Option` types and returns `::polars_core::prelude::DataType`.
///
/// # Examples
///
/// ```rust
/// use structpath_types::data_type;
/// use polars_core::prelude::DataType;
/// use indexmap::IndexMap;
///
/// // Simple types
/// let t1 = data_type!(String);
/// assert_eq!(t1, DataType::String);
///
/// // List types
/// let t2 = data_type!(List, String);
/// assert_eq!(t2, DataType::List(Box::new(DataType::String)));
///
/// // Complex nested types
/// let t3 = data_type!(List, List, String);
/// assert_eq!(t3, DataType::List(Box::new(DataType::List(Box::new(DataType::String)))));
/// ```
#[macro_export]
macro_rules! data_type {
    // Base case: simple types without parameters
    (String) => { ::polars_core::prelude::DataType::String };
    (Int32) => { ::polars_core::prelude::DataType::Int32 };
    (Int64) => { ::polars_core::prelude::DataType::Int64 };
    (Float64) => { ::polars_core::prelude::DataType::Float64 };
    (Boolean) => { ::polars_core::prelude::DataType::Boolean };
    (Object($name:expr)) => { ::polars_core::prelude::DataType::Object($name) };

    // Special cases with parameters
    (Struct($fields:expr)) => { ::polars_core::prelude::DataType::Struct($fields) };

    // Recursive cases for wrapping types (no Option support)
    (List, $($rest:tt)*) => {
        ::polars_core::prelude::DataType::List(Box::new($crate::data_type!($($rest)*)))
    };
}

/// A macro to simplify the construction of polars Field objects for struct fields.
///
/// This macro combines a field name with a DataType type specification,
/// automatically converting the field name to a String and applying the data_type! macro
/// to the type specification.
///
/// # Examples
///
/// ```rust
/// use structpath_types::{data_type, field_type};
/// use polars_core::prelude::DataType;
///
/// // Simple field
/// let field1 = field_type!("username", String);
/// assert_eq!(field1, ::polars_core::prelude::Field::new("username".into(), DataType::String));
///
/// // List field
/// let field2 = field_type!("tags", List, String);
/// assert_eq!(field2, ::polars_core::prelude::Field::new("tags".into(), DataType::List(Box::new(DataType::String))));
///
/// // Complex nested field
/// let field3 = field_type!("nested_tags", List, List, String);
/// assert_eq!(field3, ::polars_core::prelude::Field::new("nested_tags".into(),
///     DataType::List(Box::new(DataType::List(Box::new(DataType::String))))));
/// ```
#[macro_export]
macro_rules! field_type {
    ($field_name:expr, $($type_spec:tt)*) => {
        ::polars_core::prelude::Field::new($field_name.into(), $crate::data_type!($($type_spec)*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_data_type_macro_simple_types() {
        use polars_core::prelude::DataType;

        assert_eq!(data_type!(String), DataType::String);
        assert_eq!(data_type!(Int32), DataType::Int32);
        assert_eq!(data_type!(Int64), DataType::Int64);
        assert_eq!(data_type!(Float64), DataType::Float64);
        assert_eq!(data_type!(Boolean), DataType::Boolean);
    }

    #[test]
    fn test_data_type_macro_list_types() {
        use polars_core::prelude::DataType;

        assert_eq!(
            data_type!(List, String),
            DataType::List(Box::new(DataType::String))
        );
        assert_eq!(
            data_type!(List, Int32),
            DataType::List(Box::new(DataType::Int32))
        );
        assert_eq!(
            data_type!(List, Int64),
            DataType::List(Box::new(DataType::Int64))
        );
    }

    #[test]
    fn test_data_type_macro_nested_lists() {
        use polars_core::prelude::DataType;

        // List(List(String))
        assert_eq!(
            data_type!(List, List, String),
            DataType::List(Box::new(DataType::List(Box::new(DataType::String))))
        );

        // List(List(List(Int64)))
        assert_eq!(
            data_type!(List, List, List, Int64),
            DataType::List(Box::new(DataType::List(Box::new(DataType::List(
                Box::new(DataType::Int64)
            )))))
        );
    }

    #[test]
    fn test_data_type_macro_with_struct() {
        use polars_core::prelude::DataType;

        // Simple Struct
        let fields = vec![polars_core::prelude::Field::new(
            "field1".into(),
            DataType::String,
        )];
        assert_eq!(data_type!(Struct(fields.clone())), DataType::Struct(fields));
    }

    #[test]
    fn test_field_type_macro() {
        use polars_core::prelude::DataType;

        // Test simple types
        let result1 = field_type!("name", String);
        assert_eq!(
            result1,
            ::polars_core::prelude::Field::new("name".into(), DataType::String)
        );

        let result2 = field_type!("age", Int32);
        assert_eq!(
            result2,
            ::polars_core::prelude::Field::new("age".into(), DataType::Int32)
        );

        let result3 = field_type!("big_age", Int64);
        assert_eq!(
            result3,
            ::polars_core::prelude::Field::new("big_age".into(), DataType::Int64)
        );

        // Test list types
        let result4 = field_type!("tags", List, String);
        assert_eq!(
            result4,
            ::polars_core::prelude::Field::new(
                "tags".into(),
                DataType::List(Box::new(DataType::String))
            )
        );

        // Test complex nested types
        let result5 = field_type!("nested_lists", List, List, Int32);
        assert_eq!(
            result5,
            ::polars_core::prelude::Field::new(
                "nested_lists".into(),
                DataType::List(Box::new(DataType::List(Box::new(DataType::Int32))))
            )
        );

        // Test with struct
        let fields = vec![polars_core::prelude::Field::new(
            "subfield".into(),
            DataType::String,
        )];
        let result6 = field_type!("nested", Struct(fields.clone()));
        assert_eq!(
            result6,
            ::polars_core::prelude::Field::new("nested".into(), DataType::Struct(fields))
        );
    }
}
