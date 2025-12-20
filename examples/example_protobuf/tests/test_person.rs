use polars_structpath::polars_core::prelude::{AnyValue, DataType};
use polars_structpath::{data_type_wrapper, HasDataTypeWrapper, StructPath};
use example_protobuf::example_protobuf;

#[test]
fn test_get_type_person() -> Result<(), Box<dyn std::error::Error>> {
    let person_type = example_protobuf::Person::data_type();
    assert!(matches!(person_type, DataType::Struct(_)));
    Ok(())
}

#[test]
fn test_get_type_fields() -> Result<(), Box<dyn std::error::Error>> {
    let name_type = example_protobuf::Person::get_type("name")?;
    assert_eq!(name_type, data_type_wrapper!(String));

    let age_type = example_protobuf::Person::get_type("age")?;
    assert_eq!(age_type, data_type_wrapper!(Int64));

    let email_type = example_protobuf::Person::get_type("email")?;
    assert_eq!(email_type, data_type_wrapper!(Option(String)));

    let street_type = example_protobuf::Person::get_type("address.street")?;
    assert_eq!(street_type, data_type_wrapper!(Option(String)));

    let tag_type = example_protobuf::Person::get_type("tags")?;
    assert_eq!(tag_type, data_type_wrapper!(List(String)));

    let tag0_type = example_protobuf::Person::get_type("tags[0]")?;
    assert_eq!(tag0_type, data_type_wrapper!(String));

    Ok(())
}

#[test]
fn test_get_value_person() -> Result<(), Box<dyn std::error::Error>> {
    let mut person = example_protobuf::Person::default();
    person.name = "Alice".to_string();
    person.age = 30;
    person.email = Some("alice@example.com".to_string());
    person.is_active = true;

    person.address = Some(example_protobuf::person::Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip_code: 12345,
    });

    person.tags.push("premium".to_string());
    person.tags.push("verified".to_string());

    person.status = 1; // ACTIVE

    let name = person.get_value("name")?;
    assert_eq!(name, AnyValue::String("Alice"));

    let age = person.get_value("age")?;
    assert_eq!(age, AnyValue::Int64(30));

    let email = person.get_value("email")?;
    assert_eq!(email, AnyValue::String("alice@example.com"));

    let street = person.get_value("address.street")?;
    assert_eq!(street, AnyValue::String("123 Main St"));

    let tag0 = person.get_value("tags[0]")?;
    assert_eq!(tag0, AnyValue::String("premium"));

    Ok(())
}

#[test]
fn test_get_value_nested_array() -> Result<(), Box<dyn std::error::Error>> {
    let mut person = example_protobuf::Person::default();

    person
        .previous_addresses
        .push(example_protobuf::person::Address {
            street: "456 Old St".to_string(),
            city: "Oldtown".to_string(),
            zip_code: 54321,
        });

    person
        .previous_addresses
        .push(example_protobuf::person::Address {
            street: "789 New St".to_string(),
            city: "Newtown".to_string(),
            zip_code: 98765,
        });

    let first_old_street = person.get_value("previous_addresses[0].street")?;
    assert_eq!(first_old_street, AnyValue::String("456 Old St"));

    let second_old_city = person.get_value("previous_addresses[1].city")?;
    assert_eq!(second_old_city, AnyValue::String("Newtown"));

    Ok(())
}
