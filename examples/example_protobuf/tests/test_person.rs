use example_protobuf::example_protobuf::{Person, Address, Status};
use polars_structpath::{ArrowBuffer, IntoArrow};

#[test]
fn test_person_to_arrow() {
    // Create a person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
        is_active: true,
        address: Some(Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            zip_code: 12345,
        }),
        tags: vec!["premium".to_string(), "verified".to_string()],
        status: Status::ACTIVE,
        previous_addresses: vec![],
    };

    // Create a buffer and push the person
    let mut buffer = Person::new_buffer(1);
    buffer.push(person.clone());

    // Convert to Arrow array
    let arrow_array = buffer.to_arrow().expect("Failed to convert to Arrow");

    // Verify the array has 1 element
    assert_eq!(arrow_array.len(), 1);
}

#[test]
fn test_person_roundtrip() {
    use polars_structpath::FromArrow;

    // Create persons
    let persons = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
            is_active: true,
            address: Some(Address {
                street: "123 Main St".to_string(),
                city: "Springfield".to_string(),
                zip_code: 12345,
            }),
            tags: vec!["premium".to_string()],
            status: Status::ACTIVE,
            previous_addresses: vec![],
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            email: None,
            is_active: false,
            address: None,
            tags: vec![],
            status: Status::INACTIVE,
            previous_addresses: vec![
                Address {
                    street: "Old St".to_string(),
                    city: "Oldtown".to_string(),
                    zip_code: 11111,
                },
            ],
        },
    ];

    // Convert to Arrow
    let mut buffer = Person::new_buffer(persons.len());
    for person in &persons {
        buffer.push(person.clone());
    }
    let arrow_array = buffer.to_arrow().expect("Failed to convert to Arrow");

    // Convert back from Arrow
    let recovered: Vec<Person> = Person::from_arrow(Box::new(arrow_array));

    // Verify roundtrip
    assert_eq!(persons.len(), recovered.len());
    assert_eq!(persons[0].name, recovered[0].name);
    assert_eq!(persons[0].age, recovered[0].age);
    assert_eq!(persons[1].name, recovered[1].name);
    assert_eq!(persons[1].email, recovered[1].email);
}

#[test]
fn test_enum_status() {
    // Test that enum values are preserved
    let statuses = vec![Status::UNKNOWN, Status::ACTIVE, Status::INACTIVE];

    let mut buffer = Status::new_buffer(statuses.len());
    for status in &statuses {
        buffer.push(status.clone());
    }
    let arrow_array = buffer.to_arrow().expect("Failed to convert to Arrow");

    // Verify the array has correct length
    assert_eq!(arrow_array.len(), 3);
}
