//! HTTP client wrapper for LinkedIn API requests.

/// Placeholder for the LinkedIn API client.
///
/// Will hold a `reqwest::Client` with cookie jar, auth headers,
/// and Chrome-like HTTP/2 settings.
pub struct LinkedInClient {
    _private: (),
}

impl LinkedInClient {
    /// Create a new client (stub).
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for LinkedInClient {
    fn default() -> Self {
        Self::new()
    }
}
