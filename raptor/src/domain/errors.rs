use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("Key cannot be empty")]
    EmptyKey,
    #[error("Operation is not applicable to the value's type")]
    WrongType,
    #[error("Value is not a valid integer")]
    NotAnInteger,
    #[error("Failed to decode base64 data")]
    DecodeError,
    #[error("Counter value cannot be negative")]
    NegativeCounterValue,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("An unexpected error occurred")]
    Unexpected,
}
