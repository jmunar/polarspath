use polars_core::prelude::AnyValue;
use polars_structpath::{data_type_wrapper, StructPath};

#[derive(StructPath, Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        name: "John".to_string(),
        age: 32,
    };

    let name_type = Person::get_type("name")?;
    assert_eq!(name_type, data_type_wrapper!(String));
    let age_type = Person::get_type("age")?;
    assert_eq!(age_type, data_type_wrapper!(Int64));

    let name = person.get_value("name")?;
    assert_eq!(name, AnyValue::String("John"));
    let age = person.get_value("age")?;
    assert_eq!(age, AnyValue::Int64(32));

    Ok(())
}
