use polars_core::prelude::AnyValue;
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
    #[type_hint = "struct"]
    parent: Vec<Parent>,
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parent: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
    };

    let father_name = user.get_value("parent[0].name").unwrap();
    assert_eq!(father_name, AnyValue::String("Joseph"));
}
