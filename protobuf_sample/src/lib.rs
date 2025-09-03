pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

#[cfg(test)]
mod tests {
    use super::sample;
    use structpath::{FieldInfo, FieldType, StructPath, StructPathError};

    #[test]
    fn test_get_type_user() -> Result<(), Box<dyn std::error::Error>> {
        let type_ = sample::User::get_type("name")?;
        assert_eq!(type_, FieldType::String);
        let type_ = sample::User::get_type("age")?;
        assert_eq!(type_, FieldType::Integer);
        let type_ = sample::User::get_type("email")?;
        assert_eq!(type_, FieldType::Option(Box::new(FieldType::String)));
        let type_ = sample::User::get_type("is_active")?;
        assert_eq!(type_, FieldType::Boolean);
        let type_ = sample::User::get_type("favourite_pet")?;
        assert_eq!(
            type_,
            FieldType::Option(Box::new(FieldType::StructPath(
                "user :: Pet".to_string(),
                vec![
                    FieldInfo::new("name", FieldType::String),
                    FieldInfo::new("birth_year", FieldType::Integer),
                ]
            )))
        );
        let type_ = sample::User::get_type("favourite_pet.name")?;
        assert_eq!(type_, FieldType::String);
        let type_ = sample::User::get_type("tags")?;
        assert_eq!(type_, FieldType::Vec(Box::new(FieldType::String)));
        let type_ = sample::User::get_type("loyalty")?;
        assert_eq!(type_, FieldType::Unknown);
        let type_ = sample::User::get_type("pets")?;
        assert_eq!(
            type_,
            FieldType::Vec(Box::new(FieldType::StructPath(
                "user :: Pet".to_string(),
                vec![
                    FieldInfo::new("name", FieldType::String),
                    FieldInfo::new("birth_year", FieldType::Integer),
                ]
            )))
        );
        let type_ = sample::User::get_type("pets[0]")?;
        assert_eq!(
            type_,
            FieldType::StructPath(
                "user :: Pet".to_string(),
                vec![
                    FieldInfo::new("name", FieldType::String),
                    FieldInfo::new("birth_year", FieldType::Integer),
                ]
            )
        );
        let type_ = sample::User::get_type("pets[0].name")?;
        assert_eq!(type_, FieldType::String);
        let type_ = sample::User::get_type("pets[0].birth_year")?;
        assert_eq!(type_, FieldType::Integer);
        Ok(())
    }

    /// Create a new user with arbitrary values
    fn create_test_user() -> sample::User {
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

    #[test]
    fn test_get_value_user() -> Result<(), Box<dyn std::error::Error>> {
        let user = create_test_user();

        let name = user.get_value_safe("name")?;
        assert_eq!(name, "John Doe".to_string());

        let age = user.get_value_safe("age")?;
        assert_eq!(age, 30);

        let email = user.get_value_safe("email")?;
        assert_eq!(email, Some("john.doe@example.com".to_string()));

        let is_active = user.get_value_safe("is_active")?;
        assert_eq!(is_active, true);

        // Note that protobuf sub-messages are always optional
        let favourite_pet = user.get_value_safe("favourite_pet")?;
        assert_eq!(
            favourite_pet,
            Some(&sample::user::Pet {
                name: "Buddy".to_string(),
                birth_year: 2020,
            })
        );

        let favourite_pet_name_type = sample::User::get_type_safe("favourite_pet.name")?;
        assert_eq!(
            favourite_pet_name_type,
            FieldType::Option(Box::new(FieldType::String))
        );
        let favourite_pet_name = user.get_value_safe("favourite_pet.name")?;
        assert_eq!(favourite_pet_name, Some("Buddy".to_string()));

        let tags = user.get_value("tags")?;
        assert_eq!(tags, &vec!["premium".to_string(), "verified".to_string()]);
        let tag0 = user.get_value("tags[0]")?;
        assert_eq!(tag0, "premium".to_string());

        let pets = user.get_value("pets")?;
        assert_eq!(
            pets,
            &vec![
                sample::user::Pet {
                    name: "Buddy".to_string(),
                    birth_year: 2020,
                },
                sample::user::Pet {
                    name: "Max".to_string(),
                    birth_year: 2022,
                },
            ]
        );
        let pet0 = user.get_value("pets[0]")?;
        assert_eq!(
            pet0,
            &sample::user::Pet {
                name: "Buddy".to_string(),
                birth_year: 2020,
            }
        );
        let pet0_name = user.get_value("pets[0].name")?;
        assert_eq!(pet0_name, "Buddy".to_string());
        let pet0_birth_year = user.get_value("pets[0].birth_year")?;
        assert_eq!(pet0_birth_year, 2020);

        Ok(())
    }

    #[test]
    fn test_get_value_user_default() -> Result<(), Box<dyn std::error::Error>> {
        let user = sample::User::default();

        let name = user.get_value("name")?;
        assert_eq!(name, "".to_string());

        let age = user.get_value("age")?;
        assert_eq!(age, 0);

        let email = user.get_value("email")?;
        assert_eq!(email, None::<String>);

        let is_active = user.get_value("is_active")?;
        assert_eq!(is_active, false);

        let favourite_pet = user.get_value("favourite_pet")?;
        assert_eq!(favourite_pet, None::<&sample::user::Pet>);

        let favourite_pet_name = user.get_value("favourite_pet.name");
        assert_eq!(favourite_pet_name.unwrap_err(), StructPathError::NullValue);

        let tags = user.get_value("tags")?;
        assert_eq!(tags, &Vec::<String>::new());

        let pets = user.get_value("pets")?;
        assert_eq!(pets, &Vec::<sample::user::Pet>::new());

        Ok(())
    }

    #[test]
    fn test_get_value_group() -> Result<(), Box<dyn std::error::Error>> {
        let group = sample::Group {
            name: "My Group".to_string(),
            admin: Some(create_test_user()),
            members: vec![create_test_user()],
        };

        let name = group.get_value("name")?;
        assert_eq!(name, "My Group".to_string());

        let admin = group.get_value("admin")?;
        assert_eq!(admin, &create_test_user());
        let admin_name = group.get_value("admin.name")?;
        assert_eq!(admin_name, "John Doe".to_string());

        let members = group.get_value("members")?;
        assert_eq!(members, &vec![create_test_user()]);

        let member0_pet0_birth_year = group.get_value("members[0].pets[0].birth_year")?;
        assert_eq!(member0_pet0_birth_year, 2020);

        Ok(())
    }
}
