pub fn identifier_snake_case_to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;
    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else {
            result.push(if capitalize {
                c.to_uppercase().next().unwrap_or(c)
            } else {
                c.to_lowercase().next().unwrap_or(c)
            });
            capitalize = false;
        }
    }
    result
}

pub fn identifier_camel_case_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lowercase = false;
    for c in s.chars() {
        if c.is_uppercase() && prev_was_lowercase {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
        prev_was_lowercase = c.is_lowercase();
    }
    result
}

pub fn identifier_camel_case_to_upper_snake_case(s: &str) -> String {
    identifier_camel_case_to_snake_case(s).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::{
        identifier_camel_case_to_snake_case, identifier_camel_case_to_upper_snake_case,
        identifier_snake_case_to_camel_case,
    };

    #[test]
    fn test_identifier_snake_case_to_camel_case() {
        assert_eq!(identifier_snake_case_to_camel_case("hello"), "Hello");
        assert_eq!(
            identifier_snake_case_to_camel_case("hello_world"),
            "HelloWorld"
        );
        assert_eq!(identifier_snake_case_to_camel_case("HELLO"), "Hello");
        assert_eq!(
            identifier_snake_case_to_camel_case("HELLO_WORLD"),
            "HelloWorld"
        );
    }

    #[test]
    fn test_identifier_camel_case_to_snake_case() {
        assert_eq!(identifier_camel_case_to_snake_case("Hello"), "hello");
        assert_eq!(
            identifier_camel_case_to_snake_case("HelloWorld"),
            "hello_world"
        );
        assert_eq!(identifier_camel_case_to_snake_case("HELLO"), "hello");
        assert_eq!(
            identifier_camel_case_to_snake_case("HELLO_WORLD"),
            "hello_world"
        );
    }

    #[test]
    fn test_identifier_camel_case_to_upper_snake_case() {
        assert_eq!(identifier_camel_case_to_upper_snake_case("Hello"), "HELLO");
        assert_eq!(
            identifier_camel_case_to_upper_snake_case("HelloWorld"),
            "HELLO_WORLD"
        );
        assert_eq!(identifier_camel_case_to_upper_snake_case("HELLO"), "HELLO");
        assert_eq!(
            identifier_camel_case_to_upper_snake_case("HELLO_WORLD"),
            "HELLO_WORLD"
        );
    }
}
