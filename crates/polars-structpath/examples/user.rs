use polars_structpath::{ArrowBuffer, EnumPath, FromArrow, IntoArrow, StructPath};

#[derive(EnumPath, Debug, Clone, PartialEq)]
pub enum Pet {
    Dog = 1,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct User {
    name: String,
    age: i64,
    parent_favorite: Parent,
    parents: Vec<Parent>,
    pets: Option<Vec<Pet>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_in = vec![User {
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
    }];

    let mut buffer = User::new_buffer(1);
    for user in &users_in {
        buffer.push(user.clone());
    }
    let array_ref = buffer.to_arrow()?;
    let users_out = User::from_arrow(Box::new(array_ref));
    assert_eq!(users_in, users_out);

    Ok(())
}
