use structpath::{StructInfo, StructPath};

#[derive(Debug, Clone, PartialEq)]
enum Pet {
    Dog,
}

#[derive(StructInfo, StructPath, Debug, Clone, PartialEq)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructInfo, StructPath, Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: i64,
    #[type_hint = "struct"]
    parent: Vec<Parent>,
    pets: Option<Vec<Pet>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parent: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
        pets: Some(vec![Pet::Dog]),
    };

    let age = user.get_value("age")?;
    assert_eq!(age.as_i64(), 32);

    let father_name = user.get_value("parent[0].name")?;
    assert_eq!(father_name.as_str(), "Joseph");

    let pet_name = user.get_value("pets[0]")?;
    assert_eq!(pet_name.as_unboxed::<Pet>(), &Pet::Dog);

    println!("{:?}", User::get_fields_info());

    Ok(())
}
