use structpath::{FieldType, StructPath};

#[derive(Debug, Clone, PartialEq)]
enum Pet {
    Dog,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: i64,
    #[type_hint = "struct"]
    parent_favorite: Parent,
    #[type_hint = "struct"]
    parents: Vec<Parent>,
    pets: Option<Vec<Pet>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parent_favorite: Parent {
            name: "Mary".to_string(),
            age: 67,
        },
        parents: vec![
            Parent {
                name: "Joseph".to_string(),
                age: 65,
            },
            Parent {
                name: "Mary".to_string(),
                age: 67,
            },
        ],
        pets: Some(vec![Pet::Dog]),
    };

    let name_type = User::get_type("name")?;
    assert_eq!(name_type, FieldType::String);
    let age_type = User::get_type("age")?;
    assert_eq!(age_type, FieldType::Integer);
    let parent_favorite_type = User::get_type("parent_favorite")?;
    assert_eq!(
        parent_favorite_type,
        FieldType::StructPath("Parent".to_string())
    );
    let parent_favorite_name_type = User::get_type("parent_favorite.name")?;
    assert_eq!(parent_favorite_name_type, FieldType::String);
    let parents_type = User::get_type("parents")?;
    assert_eq!(
        parents_type,
        FieldType::Vec(Box::new(FieldType::StructPath("Parent".to_string())))
    );
    let parent_0_type = User::get_type("parents[0]")?;
    assert_eq!(parent_0_type, FieldType::StructPath("Parent".to_string()));
    let pets_type = User::get_type("pets")?;
    assert_eq!(
        pets_type,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Unknown))))
    );
    let pets_0_type = User::get_type("pets[0]")?;
    assert_eq!(pets_0_type, FieldType::Option(Box::new(FieldType::Unknown)));

    let name = user.get_value("name")?;
    assert_eq!(name, "John");
    assert_eq!(name, "John".to_string());
    let age = user.get_value("age")?;
    assert_eq!(age, 32);
    let parent_favorite = user.get_value("parent_favorite")?;
    assert_eq!(
        parent_favorite,
        &Parent {
            name: "Mary".to_string(),
            age: 67,
        }
    );
    let parent_favorite_name = user.get_value("parent_favorite.name")?;
    assert_eq!(parent_favorite_name, "Mary");
    let parent_0_name = user.get_value("parents[0].name")?;
    assert_eq!(parent_0_name, "Joseph");
    let pet_0 = user.get_value("pets[0]")?;
    assert_eq!(pet_0, &Pet::Dog);

    Ok(())
}
