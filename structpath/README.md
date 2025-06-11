# structpath

A Rust library for dynamically accessing nested structures using path notation.
For a use case, see the examples path.

## Example

```rust
use structpath::prelude::*;

use std::str::FromStr;

#[derive(StructPath, Debug, Clone)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone)]
struct User {
    name: String,
    age: i64,
    #[type_hint = "struct"]
    parent: Vec<Parent>,
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parent: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
    };

    let father_name = user.get_value("parent[0].name")?;
    assert_eq!(father_name.as_str(), "Joseph");
}
```

Find the full example, and some others, in the `examples` folder.

## Current status

Currently, `get_value()` can only access objects being:
- `String`
- `i64`
- `f64`
- `bool`
- arbitrary objects (boxing them)
- optionals of the above types
- vectors of the above types
- nested objects with implemented `StructPath` trait (`StructPath` macro)

Missing:
- add more scalar types
- handle nested vectors
