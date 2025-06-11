use thiserror::Error;

#[derive(Debug, Error)]
pub enum StructPathError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Vector index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    #[error("Cannot access field of null value")]
    NullValue,

    #[error("Functionality not yet implemented")]
    NotImplemented,
}
