use thiserror::Error;

pub type ValidationResult<T> = Result<T, Vec<ValidationError>>;

/// Validation error with path context.
#[derive(Debug, Clone, Error, PartialEq)]
#[error("{path}: {message}")]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}
