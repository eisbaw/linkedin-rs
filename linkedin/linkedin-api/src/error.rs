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

    /// LinkedIn API returned an error status code.
    #[error("API error (HTTP {status}): {body}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Response body (may be JSON error or empty).
        body: String,
    },

    /// Invalid input provided by the caller.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
