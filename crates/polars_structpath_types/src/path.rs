//! Module for parsing and representing paths to nested struct fields.
//!
//! This module provides the `Path` and `PathComponent` types for representing
//! and parsing string-based paths to nested fields, similar to JSON path notation.
//! Paths support field access (`.field`) and array indexing (`[0]`).

/// Represents a single component in a path.
///
/// A path is composed of one or more `PathComponent` values. Each component
/// represents either a field access or an array index access.
///
/// # Examples
///
/// ```rust
/// use polars_structpath_types::{Path, PathComponent};
///
/// // Simple field access
/// let path = Path::from_str("name").unwrap();
/// assert_eq!(path.components()[0], PathComponent::Field("name".to_string()));
///
/// // Nested field access
/// let path = Path::from_str("parent.name").unwrap();
/// assert_eq!(path.components()[0], PathComponent::Field("parent".to_string()));
/// assert_eq!(path.components()[1], PathComponent::Field("name".to_string()));
///
/// // Array index access
/// let path = Path::from_str("items[0]").unwrap();
/// assert_eq!(
///     path.components()[0],
///     PathComponent::ArrayIndex("items".to_string(), 0)
/// );
///
/// // Combined: array index then field
/// let path = Path::from_str("items[0].name").unwrap();
/// assert_eq!(
///     path.components()[0],
///     PathComponent::ArrayIndex("items".to_string(), 0)
/// );
/// assert_eq!(path.components()[1], PathComponent::Field("name".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PathComponent {
    /// A field name access (e.g., `"name"`, `"father"`).
    ///
    /// This represents accessing a field directly on a struct.
    Field(String),
    /// An array index access (e.g., `"pets[0]"`).
    ///
    /// The first `String` is the field name containing the array,
    /// and the `usize` is the index into that array.
    ArrayIndex(String, usize),
}

/// Represents a parsed path for accessing nested struct fields.
///
/// A `Path` is a sequence of [`PathComponent`] values that describes how to
/// navigate through nested structures. Paths can be created from strings using
/// [`from_str()`](Path::from_str).
///
/// # Path Syntax
///
/// Paths support the following syntax:
///
/// - **Simple fields**: `"name"` - accesses a top-level field
/// - **Nested fields**: `"parent.name"` - accesses a nested field using dot notation
/// - **Array indices**: `"items[0]"` - accesses an element in an array
/// - **Combined**: `"items[0].name"` - accesses a field within an array element
///
/// # Examples
///
/// ```rust
/// use polars_structpath_types::{Path, PathComponent};
///
/// // Simple field
/// let path = Path::from_str("name").unwrap();
/// assert_eq!(path.components().len(), 1);
///
/// // Nested field
/// let path = Path::from_str("parent.name").unwrap();
/// assert_eq!(path.components().len(), 2);
///
/// // Array access
/// let path = Path::from_str("items[0]").unwrap();
/// assert_eq!(
///     path.components()[0],
///     PathComponent::ArrayIndex("items".to_string(), 0)
/// );
///
/// // Complex nested path
/// let path = Path::from_str("users[0].address.street").unwrap();
/// assert_eq!(path.components().len(), 3);
/// ```
///
/// # Error Cases
///
/// The following will result in parse errors:
///
/// - Empty strings: `""`
/// - Invalid array indices: `"items[a]"`
/// - Unclosed brackets: `"items[0"`
/// - Whitespace: `"name "`
#[derive(Debug, Clone)]
pub struct Path {
    /// The components of the path
    pub components: Vec<PathComponent>,
}

#[allow(clippy::new_without_default)]
impl Path {
    /// Creates a new empty path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use polars_structpath_types::Path;
    ///
    /// let path = Path::new();
    /// assert!(path.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Returns a slice of the path components.
    ///
    /// # Example
    ///
    /// ```rust
    /// use polars_structpath_types::{Path, PathComponent};
    ///
    /// let path = Path::from_str("parent.name").unwrap();
    /// let components = path.components();
    /// assert_eq!(components.len(), 2);
    /// assert_eq!(components[0], PathComponent::Field("parent".to_string()));
    /// ```
    pub fn components(&self) -> &[PathComponent] {
        &self.components
    }

    /// Checks if the path is empty (has no components).
    ///
    /// # Example
    ///
    /// ```rust
    /// use polars_structpath_types::Path;
    ///
    /// let empty = Path::new();
    /// assert!(empty.is_empty());
    ///
    /// let path = Path::from_str("name").unwrap();
    /// assert!(!path.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

/// Error type for path parsing failures.
///
/// This error is returned when attempting to parse an invalid path string.
///
/// # Examples
///
/// ```rust
/// use polars_structpath_types::{Path, PathParseError};
///
/// // Empty path
/// assert!(matches!(
///     Path::from_str(""),
///     Err(PathParseError::EmptyPath)
/// ));
///
/// // Invalid array index
/// assert!(matches!(
///     Path::from_str("items[a]"),
///     Err(PathParseError::InvalidArrayIndex(_))
/// ));
///
/// // Unclosed bracket
/// assert!(matches!(
///     Path::from_str("items[0"),
///     Err(PathParseError::UnclosedBracket)
/// ));
///
/// // Unexpected character (whitespace)
/// assert!(matches!(
///     Path::from_str("name "),
///     Err(PathParseError::UnexpectedChar(_))
/// ));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum PathParseError {
    /// The path string was empty.
    #[error("Empty path")]
    EmptyPath,

    /// The array index could not be parsed as a `usize`.
    ///
    /// The contained string is the invalid index value.
    #[error("Invalid array index: {0}")]
    InvalidArrayIndex(String),

    /// An opening bracket `[` was found but no closing bracket `]`.
    #[error("Unclosed array bracket")]
    UnclosedBracket,

    /// An unexpected character was encountered (e.g., whitespace).
    ///
    /// The contained `char` is the unexpected character.
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
}

/// We don't implement the FromStr trait to avoid extra imports.
/// This prevents us from e.g. using `"pets[0].name".parse::<Path>()`.
impl Path {
    /// Parses a path string into a `Path` object.
    ///
    /// This method parses a string representation of a path into a structured
    /// `Path` object that can be used for type queries and value extraction.
    ///
    /// # Arguments
    ///
    /// * `s` - A string representing the path (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Path)` if the path is valid, or a [`PathParseError`] if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use polars_structpath_types::{Path, PathComponent};
    ///
    /// // Simple field
    /// let path = Path::from_str("name").unwrap();
    /// assert_eq!(path.components()[0], PathComponent::Field("name".to_string()));
    ///
    /// // Nested field
    /// let path = Path::from_str("parent.name").unwrap();
    /// assert_eq!(path.components().len(), 2);
    ///
    /// // Array index
    /// let path = Path::from_str("items[0]").unwrap();
    /// assert_eq!(
    ///     path.components()[0],
    ///     PathComponent::ArrayIndex("items".to_string(), 0)
    /// );
    ///
    /// // Complex path
    /// let path = Path::from_str("users[0].address.street").unwrap();
    /// assert_eq!(path.components().len(), 3);
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error for:
    ///
    /// - Empty strings
    /// - Invalid array indices (non-numeric)
    /// - Unclosed brackets
    /// - Paths containing whitespace
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
