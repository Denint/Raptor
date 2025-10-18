use crate::domain::errors::DomainError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct ApiError(DomainError);

impl ApiError {
    pub fn new(error: DomainError) -> Self {
        Self(error)
    }

    pub fn domain_error(&self) -> &DomainError {
        &self.0
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        Self(error)
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(_error: base64::DecodeError) -> Self {
        Self(DomainError::DecodeError)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match self.0 {
            DomainError::EmptyKey
            | DomainError::NotAnInteger
            | DomainError::DecodeError
            | DomainError::WrongType
            | DomainError::NegativeCounterValue => StatusCode::BAD_REQUEST,
            DomainError::StorageError(_) | DomainError::Unexpected => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status_code, self.0.to_string()).into_response()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
