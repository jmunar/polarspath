use structpath::{Path, PathComponent, StructPath, StructPathError, StructPathTrait, Value};

use std::str::FromStr;

// #[derive(StructPath, Debug, Clone)]
// enum Pet {
//     Dog,
//     Cat,
// }

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
    // #[type_hint = "enum"]
    // pets: Option<Vec<Pet>>,
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parent: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
        // pets: Some(vec![Pet::Dog]),
    };
    let age = user.get_value("age").unwrap();
    assert_eq!(age, Value::Integer(32));

    let father_name = user.get_value("parent[0].name").unwrap();
    assert_eq!(father_name, Value::String("Joseph".to_string()));

    // let pet_name = user.get_value("pets[0]").unwrap();
    // assert_eq!(pet_name, Value::Boxable(Pet::Dog.clone_box()));
}
