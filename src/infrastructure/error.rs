/// Central error type for the application.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("LLM service error: {0}")]
    LlmError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Vector store error: {0}")]
    VectorStoreError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[cfg(feature = "server")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        tracing::error!("Application error: {self}");

        let (status, message) = match &self {
            Error::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Error::DatabaseError(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            Error::AuthError(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Error::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Error::LlmError(_) => (StatusCode::BAD_GATEWAY, "LLM service unavailable".into()),
            Error::EmbeddingError(_) => {
                (StatusCode::BAD_GATEWAY, "Embedding service unavailable".into())
            }
            Error::VectorStoreError(_) => {
                (StatusCode::BAD_GATEWAY, "Vector store unavailable".into())
            }
            Error::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
            Error::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        };

        (status, message).into_response()
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::InternalError(e.to_string())
    }
}
