/// Represents a single component in a path
#[derive(Debug, Clone, PartialEq)]
pub enum PathComponent {
    /// A field name (e.g., "name", "father")
    Field(String),
    /// An array index (e.g., "pets[0]")
    ArrayIndex(String, usize),
}

/// Represents a parsed path
#[derive(Debug, Clone)]
pub struct Path {
    /// The components of the path
    pub components: Vec<PathComponent>,
}

#[allow(clippy::new_without_default)]
impl Path {
    /// Create a new empty path
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Get the components of the path
    pub fn components(&self) -> &[PathComponent] {
        &self.components
    }

    /// Check if the path is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

/// Error type for path parsing
#[derive(Debug, thiserror::Error)]
pub enum PathParseError {
    #[error("Empty path")]
    EmptyPath,

    #[error("Invalid array index: {0}")]
    InvalidArrayIndex(String),

    #[error("Unclosed array bracket")]
    UnclosedBracket,

    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
}

/// We don't implement the FromStr trait to avoid extra imports
/// This prevent us from e.g. using `"pets[0].name".parse::<Path>()`
impl Path {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, PathParseError> {
        if s.is_empty() {
            return Err(PathParseError::EmptyPath);
        }

        let mut components = Vec::new();
        let mut current = String::new();
        let mut in_bracket = false;

        for c in s.chars() {
            match c {
                '.' if !in_bracket => {
                    if !current.is_empty() {
                        components.push(PathComponent::Field(current));
                        current = String::new();
                    }
                }
                '[' if !in_bracket => {
                    if current.is_empty() {
                        return Err(PathParseError::UnexpectedChar('['));
                    }
                    components.push(PathComponent::Field(current));
                    current = String::new();
                    in_bracket = true;
                }
                ']' if in_bracket => {
                    let index = current
                        .parse::<usize>()
                        .map_err(|_| PathParseError::InvalidArrayIndex(current.clone()))?;

                    // Get the field name before the array index
                    if let Some(PathComponent::Field(field)) = components.pop() {
                        components.push(PathComponent::ArrayIndex(field, index));
                    } else {
                        return Err(PathParseError::UnexpectedChar(']'));
                    }

                    current = String::new();
                    in_bracket = false;
                }
                c if c.is_whitespace() => {
                    return Err(PathParseError::UnexpectedChar(c));
                }
                c => {
                    current.push(c);
                }
            }
        }

        if in_bracket {
            return Err(PathParseError::UnclosedBracket);
        }

        if !current.is_empty() {
            components.push(PathComponent::Field(current));
        }

        if components.is_empty() {
            return Err(PathParseError::EmptyPath);
        }

        Ok(Path { components })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let path = Path::from_str("name").unwrap();
        assert_eq!(path.components.len(), 1);
        assert_eq!(path.components[0], PathComponent::Field("name".to_string()));
    }

    #[test]
    fn test_nested_path() {
        let path = Path::from_str("father.name").unwrap();
        assert_eq!(path.components.len(), 2);
        assert_eq!(
            path.components[0],
            PathComponent::Field("father".to_string())
        );
        assert_eq!(path.components[1], PathComponent::Field("name".to_string()));
    }

    #[test]
    fn test_array_index() {
        let path = Path::from_str("pets[0].name").unwrap();
        assert_eq!(path.components.len(), 2);
        assert_eq!(
            path.components[0],
            PathComponent::ArrayIndex("pets".to_string(), 0)
        );
        assert_eq!(path.components[1], PathComponent::Field("name".to_string()));
    }

    #[test]
    fn test_invalid_paths() {
        assert!(Path::from_str("").is_err());
        assert!(Path::from_str(".").is_err());
        assert!(Path::from_str("pets[").is_err());
        assert!(Path::from_str("pets[a]").is_err());
        assert!(Path::from_str("pets[0").is_err());
        assert!(Path::from_str("name ").is_err());
    }
}
