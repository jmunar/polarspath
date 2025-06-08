pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

#[cfg(test)]
mod tests {
    use super::sample;
    use structpath::StructPath;

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

        let name = user.get_value("name")?;
        assert_eq!(name.as_str(), "John Doe");

        let age = user.get_value("age")?;
        assert_eq!(age.as_i64(), 30);

        let email = user.get_value("email")?;
        assert_eq!(email.unwrap().as_str(), "john.doe@example.com");

        let is_active = user.get_value("is_active")?;
        assert_eq!(is_active.as_bool(), true);

        // Note that protobuf sub-messages are always optional
        let favourite_pet = user.get_value("favourite_pet")?;
        assert_eq!(
            favourite_pet
                .unwrap()
                .as_struct::<sample::user::Pet>()
                .to_owned(),
            sample::user::Pet {
                name: "Buddy".to_string(),
                birth_year: 2020,
            }
        );

        let tags = user.get_value("tags")?;
        assert_eq!(
            tags.as_array::<Vec<String>>().to_owned(),
            vec!["premium".to_string(), "verified".to_string()]
        );
        let tag0 = user.get_value("tags[0]")?;
        assert_eq!(tag0.as_str(), "premium");

        let pets = user.get_value("pets")?;
        assert_eq!(
            pets.as_array::<Vec<sample::user::Pet>>().to_owned(),
            vec![
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
            pet0.as_struct::<sample::user::Pet>().to_owned(),
            sample::user::Pet {
                name: "Buddy".to_string(),
                birth_year: 2020,
            }
        );
        let pet0_name = user.get_value("pets[0].name")?;
        assert_eq!(pet0_name.as_str(), "Buddy");
        let pet0_birth_year = user.get_value("pets[0].birth_year")?;
        assert_eq!(pet0_birth_year.as_i64(), 2020);

        Ok(())
    }

    #[test]
    fn test_get_value_user_default() -> Result<(), Box<dyn std::error::Error>> {
        let user = sample::User::default();

        let name = user.get_value("name")?;
        assert_eq!(name.as_str(), "");

        let age = user.get_value("age")?;
        assert_eq!(age.as_i64(), 0);

        let email = user.get_value("email")?;
        assert_eq!(email.as_option(), None);

        let is_active = user.get_value("is_active")?;
        assert_eq!(is_active.as_bool(), false);

        let favourite_pet = user.get_value("favourite_pet")?;
        assert_eq!(favourite_pet.as_option(), None);

        let tags = user.get_value("tags")?;
        assert_eq!(
            tags.as_array::<Vec<String>>().to_owned(),
            vec![] as Vec<String>
        );

        let pets = user.get_value("pets")?;
        assert_eq!(
            pets.as_array::<Vec<sample::user::Pet>>().to_owned(),
            vec![] as Vec<sample::user::Pet>
        );

        Ok(())
    }
}
