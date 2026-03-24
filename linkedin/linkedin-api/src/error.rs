//! Error types for the LinkedIn API client.

/// Top-level error type for the linkedin-api crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Authentication failed or session expired.
    #[error("Auth error: {0}")]
    Auth(String),
}
