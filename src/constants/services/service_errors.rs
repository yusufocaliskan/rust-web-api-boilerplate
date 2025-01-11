use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Database(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    ConnotCreate(String),

    #[error("{0}")]
    Error(String),

    #[error("{0}")]
    Success(String),
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
