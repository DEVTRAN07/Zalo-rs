//! Error hierarchy for the Zalo client.
//!
//! All errors inherit conceptually from [`ZaloError`]. Use [`ZaloApiError`]
//! for API-level failures that carry an optional numeric code.

use thiserror::Error;

/// Base error type for all Zalo client failures.
#[derive(Debug, Error)]
pub enum ZaloError {
    #[error("{0}")]
    Api(#[from] ZaloApiError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

/// API-level error with an optional Zalo error code.
#[derive(Debug, Error, Clone)]
#[error("{message}{}", .code.map(|c| format!(" (code {})", c)).unwrap_or_default())]
pub struct ZaloApiError {
    pub message: String,
    pub code: Option<i64>,
}

impl ZaloApiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    pub fn with_code(message: impl Into<String>, code: i64) -> Self {
        Self {
            message: message.into(),
            code: Some(code),
        }
    }
}

pub type Result<T> = std::result::Result<T, ZaloError>;
