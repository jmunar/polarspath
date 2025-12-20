use polars_core::prelude::{AnyValue, DataType, Field};
use polars_structpath::{data_type_wrapper, EnumPath, StructPath};

#[derive(EnumPath, Debug, Clone, PartialEq)]
enum Pet {
    Dog = 1,
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
    #[type_hint("struct")]
    parent_favorite: Parent,
    #[type_hint("struct")]
    parents: Vec<Parent>,
    #[type_hint("enum")]
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
    assert_eq!(name_type, data_type_wrapper!(String));
    let age_type = User::get_type("age")?;
    assert_eq!(age_type, data_type_wrapper!(Int64));
    let parent_favorite_type = User::get_type("parent_favorite")?;
    assert_eq!(
        parent_favorite_type,
        data_type_wrapper!(Struct([("name", String), ("age", Int64)]))
    );
    let parent_favorite_name_type = User::get_type("parent_favorite.name")?;
    assert_eq!(parent_favorite_name_type, data_type_wrapper!(String));
    let parents_type = User::get_type("parents")?;
    assert_eq!(
        parents_type,
        data_type_wrapper!(List(Struct([("name", String), ("age", Int64)])))
    );
    let parent_0_type = User::get_type("parents[0]")?;
    assert_eq!(
        parent_0_type,
        data_type_wrapper!(Struct([("name", String), ("age", Int64)]))
    );
    let pets_type = User::get_type("pets")?;
    assert_eq!(
        pets_type,
        data_type_wrapper!(Option(List(Enum([("Dog", 1)]))))
    );
    let pets_0 = User::get_type("pets[0]")?;
    assert_eq!(pets_0, data_type_wrapper!(Option(Enum([("Dog", 1)]))));

    let name = user.get_value("name")?;
    assert_eq!(name, AnyValue::String("John"));
    let age = user.get_value("age")?;
    assert_eq!(age, AnyValue::Int64(32));
    let parent_favorite = user.get_value("parent_favorite")?;
    assert_eq!(
        parent_favorite,
        // (Box<(Vec<AnyValue<'a>>, Vec<Field>)>)
        AnyValue::StructOwned(Box::new((
            vec![AnyValue::String("Mary"), AnyValue::Int64(67)],
            vec![
                Field::new("name".into(), DataType::String),
                Field::new("age".into(), DataType::Int64)
            ]
        )))
    );
    let parent_favorite_name = user.get_value("parent_favorite.name")?;
    assert_eq!(parent_favorite_name, AnyValue::String("Mary"));
    let parent_0_name = user.get_value("parents[0].name")?;
    assert_eq!(parent_0_name, AnyValue::String("Joseph"));
    let pet_0 = user.get_value("pets[0]")?;
    assert_eq!(pet_0, AnyValue::Enum(0, Pet::mapping()));

    Ok(())
}
