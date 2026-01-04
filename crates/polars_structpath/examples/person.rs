use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow, StructPath};

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct Person {
    name: String,
    age: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let people_in = vec![Person {
        name: "John".to_string(),
        age: 32,
    }];

    let mut buffer = Person::new_buffer(1);
    for person in &people_in {
        buffer.push(person.clone());
    }
    let array_ref = buffer.to_arrow()?;
    let people_out = Person::from_arrow(Box::new(array_ref));
    assert_eq!(people_in, people_out);

    Ok(())
}
