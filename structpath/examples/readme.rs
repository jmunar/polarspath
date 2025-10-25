use polars_core::prelude::{AnyValue, DataType};
use structpath::StructPath;

#[derive(StructPath, Debug, Clone)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone)]
struct User {
    name: String,
    age: i64,
    #[type_hint("struct")]
    parents: Vec<Parent>,
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parents: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
    };

    // Access nested values using path notation
    let father_name = user.get_value("parents[0].name").unwrap();
    assert_eq!(father_name, AnyValue::String("Joseph"));

    // Get type information
    let name_type = User::get_type("name").unwrap().to_data_type();
    assert_eq!(name_type, DataType::String);
}
