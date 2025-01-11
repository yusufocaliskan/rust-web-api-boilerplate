use thiserror::Error;
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Connot be created: {0}")]
    ConnotCreate(String),
}

impl From<mongodb::error::Error> for ServiceError {
    fn from(err: mongodb::error::Error) -> Self {
        ServiceError::Database(err.to_string())
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(err: validator::ValidationErrors) -> Self {
        ServiceError::Validation(err.to_string())
    }
}
