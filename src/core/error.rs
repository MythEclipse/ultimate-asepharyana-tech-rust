use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Scraper error: {0}")]
    ScraperError(String),
    #[error("Fantoccini error: {0}")]
    FantocciniError(String),
    #[error("Chromiumoxide error: {0}")]
    ChromiumoxideError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not Found: {0}")]
    NotFound(String),
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Other(s.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Other(s)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(err: deadpool_redis::PoolError) -> Self {
        AppError::Other(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::Other(err.to_string())
    }
}


impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::NotFound(_) => (http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::DatabaseError(_) => {
                (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            _ => (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        // Note: crate::core::types needs to be available. 
        // If not, we might need to adjust this line or ensure types are there.
        // Since we are validating structure, let's assume types is in core/types.rs
        let body = axum::Json(crate::core::types::ApiResponse::<()>::error(error_message));
        (status, body).into_response()
    }
}
