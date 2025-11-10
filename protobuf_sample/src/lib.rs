pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

#[cfg(feature = "extension-module")]
mod extension;

#[cfg(test)]
mod tests {
    use super::sample;
    use structpath::polars_core::prelude::{
        AnyValue, CategoricalMapping, DataType, Field, FrozenCategories, Series,
    };
    use structpath::{data_type_wrapper, EnumPath, HasDataTypeWrapper, StructPath};

    fn user_fields_expected() -> Vec<Field> {
        vec![
            Field::new("name".into(), DataType::String),
            Field::new("age".into(), DataType::Int64),
            Field::new("email".into(), DataType::String),
            Field::new("is_active".into(), DataType::Boolean),
            Field::new(
                "favourite_pet".into(),
                DataType::Struct(vec![
                    Field::new("name".into(), DataType::String),
                    Field::new("birth_year".into(), DataType::Int64),
                ]),
            ),
            Field::new("tags".into(), DataType::List(Box::new(DataType::String))),
            Field::new(
                "loyalty".into(),
                DataType::Enum(
                    FrozenCategories::new(["UNKNOWN", "SILVER", "GOLD", "PLATINUM"].into_iter())
                        .unwrap(),
                    std::sync::Arc::new(CategoricalMapping::new(3)),
                ),
            ),
            Field::new(
                "pets".into(),
                DataType::List(Box::new(DataType::Struct(vec![
                    Field::new("name".into(), DataType::String),
                    Field::new("birth_year".into(), DataType::Int64),
                ]))),
            ),
        ]
    }

    #[test]
    fn test_get_type_user() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            sample::User::data_type(),
            &DataType::Struct(user_fields_expected())
        );
        Ok(())
    }

    #[test]
    fn test_get_type_user_fields() -> Result<(), Box<dyn std::error::Error>> {
        let type_ = sample::User::get_type("name")?;
        assert_eq!(type_, data_type_wrapper!(String));
        let type_ = sample::User::get_type("age")?;
        assert_eq!(type_, data_type_wrapper!(Int64));
        let type_ = sample::User::get_type("email")?;
        assert_eq!(type_, data_type_wrapper!(Option(String)));
        let type_ = sample::User::get_type("is_active")?;
        assert_eq!(type_, data_type_wrapper!(Boolean));
        let type_ = sample::User::get_type("favourite_pet")?;
        assert_eq!(
            type_,
            data_type_wrapper!(Option(Struct([("name", String), ("birth_year", Int64)])))
        );
        let type_ = sample::User::get_type("favourite_pet.name")?;
        assert_eq!(type_, data_type_wrapper!(Option(String)));
        let type_ = sample::User::get_type("tags")?;
        assert_eq!(type_, data_type_wrapper!(List(String)));
        let type_ = sample::User::get_type("loyalty")?;
        assert_eq!(
            type_,
            data_type_wrapper!(Enum([
                ("UNKNOWN", 0),
                ("SILVER", 1),
                ("GOLD", 2),
                ("PLATINUM", 3)
            ]))
        );
        let type_ = sample::User::get_type("pets")?;
        assert_eq!(
            type_,
            data_type_wrapper!(List(Struct([("name", String), ("birth_year", Int64)])))
        );
        let type_ = sample::User::get_type("pets[0]")?;
        assert_eq!(
            type_,
            data_type_wrapper!(Struct([("name", String), ("birth_year", Int64)]))
        );
        let type_ = sample::User::get_type("pets[0].name")?;
        assert_eq!(type_, data_type_wrapper!(String));
        let type_ = sample::User::get_type("pets[0].birth_year")?;
        assert_eq!(type_, data_type_wrapper!(Int64));
        Ok(())
    }

    /// Create a new user with arbitrary values
    fn test_user() -> sample::User {
        // Note that `default()` is a prost method to create a message with protobuf default values
        // If we use `sample::User {...}` we would have to set all the fields manually
        let mut user = sample::User::default();

        user.name = "John Doe".to_string();
        user.age = 30;
        user.email = Some("john.doe@example.com".to_string());
        user.is_active = true;
        user.favourite_pet = Some(sample::user::Pet {
            name: "Buddy".to_string(),
            birth_year: 2020,
        });
        user.tags
            .extend(["premium".to_string(), "verified".to_string()]);
        user.loyalty = 1; // SILVER
        user.pets.extend([
            sample::user::Pet {
                name: "Buddy".to_string(),
                birth_year: 2020,
            },
            sample::user::Pet {
                name: "Max".to_string(),
                birth_year: 2022,
            },
        ]);
        user
    }

    fn test_user_values() -> Vec<AnyValue<'static>> {
        vec![
            AnyValue::StringOwned("John Doe".into()),
            AnyValue::Int64(30),
            AnyValue::StringOwned("john.doe@example.com".into()),
            AnyValue::Boolean(true),
            AnyValue::StructOwned(Box::new((
                vec![AnyValue::StringOwned("Buddy".into()), AnyValue::Int64(2020)],
                vec![
                    Field::new("name".into(), DataType::String),
                    Field::new("birth_year".into(), DataType::Int64),
                ],
            ))),
            AnyValue::List(Series::from_iter::<Vec<String>>(vec![
                "premium".to_string(),
                "verified".to_string(),
            ])),
            AnyValue::Enum(1, sample::user::Loyalty::mapping()),
            AnyValue::List(
                Series::from_any_values(
                    "".into(),
                    &[
                        AnyValue::StructOwned(Box::new((
                            vec![AnyValue::String("Buddy"), AnyValue::Int64(2020)],
                            vec![
                                Field::new("name".into(), DataType::String),
                                Field::new("birth_year".into(), DataType::Int64),
                            ],
                        ))),
                        AnyValue::StructOwned(Box::new((
                            vec![AnyValue::String("Max"), AnyValue::Int64(2022)],
                            vec![
                                Field::new("name".into(), DataType::String),
                                Field::new("birth_year".into(), DataType::Int64),
                            ],
                        ))),
                    ],
                    true,
                )
                .unwrap(),
            ),
        ]
    }

    #[test]
    fn test_get_value_user() -> Result<(), Box<dyn std::error::Error>> {
        let user = test_user();

        let name = user.get_value("name")?;
        assert_eq!(name, AnyValue::String("John Doe"));

        let age = user.get_value("age")?;
        assert_eq!(age, AnyValue::Int64(30));

        let email = user.get_value("email")?;
        assert_eq!(email, AnyValue::String("john.doe@example.com"));

        let is_active = user.get_value("is_active")?;
        assert_eq!(is_active, AnyValue::Boolean(true));

        let favourite_pet = user.get_value("favourite_pet")?;
        assert_eq!(
            favourite_pet,
            AnyValue::StructOwned(Box::new((
                vec![AnyValue::String("Buddy"), AnyValue::Int64(2020)],
                vec![
                    Field::new("name".into(), DataType::String),
                    Field::new("birth_year".into(), DataType::Int64),
                ]
            )))
        );

        let favourite_pet_name_type = sample::User::get_type("favourite_pet.name")?;
        assert_eq!(favourite_pet_name_type, data_type_wrapper!(Option(String)));
        let favourite_pet_name = user.get_value("favourite_pet.name")?;
        assert_eq!(favourite_pet_name, AnyValue::String("Buddy"));

        let tags = user.get_value("tags")?;
        assert_eq!(
            tags,
            AnyValue::List(Series::from_iter(vec![
                "premium".to_string(),
                "verified".to_string()
            ]))
        );
        let tag0 = user.get_value("tags[0]")?;
        assert_eq!(tag0, AnyValue::String("premium"));

        let loyalty = user.get_value("loyalty")?;
        assert_eq!(loyalty, AnyValue::Enum(1, sample::user::Loyalty::mapping()));

        let pets = user.get_value("pets")?;
        assert_eq!(
            pets,
            AnyValue::List(
                Series::from_any_values(
                    "".into(),
                    &[
                        AnyValue::StructOwned(Box::new((
                            vec![AnyValue::String("Buddy"), AnyValue::Int64(2020)],
                            vec![
                                Field::new("name".into(), DataType::String),
                                Field::new("birth_year".into(), DataType::Int64),
                            ]
                        ))),
                        AnyValue::StructOwned(Box::new((
                            vec![AnyValue::String("Max"), AnyValue::Int64(2022)],
                            vec![
                                Field::new("name".into(), DataType::String),
                                Field::new("birth_year".into(), DataType::Int64),
                            ]
                        ))),
                    ],
                    true
                )
                .unwrap()
            )
        );
        let pet0 = user.get_value("pets[0]")?;
        assert_eq!(
            pet0,
            AnyValue::StructOwned(Box::new((
                vec![AnyValue::String("Buddy"), AnyValue::Int64(2020)],
                vec![
                    Field::new("name".into(), DataType::String),
                    Field::new("birth_year".into(), DataType::Int64),
                ]
            )))
        );
        let pet0_name = user.get_value("pets[0].name")?;
        assert_eq!(pet0_name, AnyValue::String("Buddy"));
        let pet0_birth_year = user.get_value("pets[0].birth_year")?;
        assert_eq!(pet0_birth_year, AnyValue::Int64(2020));

        Ok(())
    }

    #[test]
    fn test_get_value_user_default() -> Result<(), Box<dyn std::error::Error>> {
        let user = sample::User::default();

        let name = user.get_value("name")?;
        assert_eq!(name, AnyValue::String(""));

        let age = user.get_value("age")?;
        assert_eq!(age, AnyValue::Int64(0));

        let email = user.get_value("email")?;
        assert_eq!(email, AnyValue::Null);

        let is_active = user.get_value("is_active")?;
        assert_eq!(is_active, AnyValue::Boolean(false));

        let favourite_pet = user.get_value("favourite_pet")?;
        assert_eq!(favourite_pet, AnyValue::Null);

        let favourite_pet_name = user.get_value("favourite_pet.name")?;
        assert_eq!(favourite_pet_name, AnyValue::Null);

        let tags = user.get_value("tags")?;
        assert_eq!(
            tags,
            AnyValue::List(Series::from_iter::<Vec<String>>(vec![]))
        );

        let pets = user.get_value("pets")?;
        assert_eq!(
            pets,
            AnyValue::List(Series::from_iter::<Vec<String>>(vec![]))
        );

        Ok(())
    }

    #[test]
    fn test_get_value_group() -> Result<(), Box<dyn std::error::Error>> {
        let group = sample::Group {
            name: "My Group".to_string(),
            admin: Some(test_user()),
            members: vec![test_user()],
        };

        let name = group.get_value("name")?;
        assert_eq!(name, AnyValue::String("My Group"));

        let admin = group.get_value("admin")?;
        assert_eq!(
            admin,
            AnyValue::StructOwned(Box::new((test_user_values(), user_fields_expected())))
        );
        let admin_name = group.get_value("admin.name")?;
        assert_eq!(admin_name, AnyValue::String("John Doe"));

        let members = group.get_value("members")?;
        let expected = "[{\"John Doe\",30,\"john.doe@example.com\",true,{\"Buddy\",2020},[\"premium\", \"verified\"],\"SILVER\",[{\"Buddy\",2020}, {\"Max\",2022}]}]";
        assert_eq!(members.to_string(), expected);

        let member0_pet0_birth_year = group.get_value("members[0].pets[0].birth_year")?;
        assert_eq!(member0_pet0_birth_year, AnyValue::Int64(2020));

        Ok(())
    }
}
