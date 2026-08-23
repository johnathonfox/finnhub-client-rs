//! Typed errors with classification helpers (ADR-0003).
//!
//! The crate never retries on its own. Callers use [`Error::is_retryable`] and
//! [`Error::retry_after`] to implement their own backoff policy.

use thiserror::Error;

/// Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// All failure modes of the client.
#[derive(Debug, Error)]
pub enum Error {
    /// Finnhub returned HTTP 429. `retry_after` is populated from the
    /// `Retry-After` or `X-RateLimit-Reset` response header when present
    /// (seconds).
    #[error("rate limit exceeded{}", retry_after.map(|s| format!(" (retry after {s}s)")).unwrap_or_default())]
    RateLimitExceeded {
        /// Server-suggested wait in seconds, if it sent one.
        retry_after: Option<u64>,
    },

    /// Finnhub returned HTTP 401 or 403 — the API key is missing, invalid, or
    /// the endpoint requires a paid plan.
    #[error("unauthorized (missing/invalid API key, or premium endpoint on a free plan)")]
    Unauthorized,

    /// Finnhub returned HTTP 404.
    #[error("not found")]
    NotFound,

    /// Any other non-2xx HTTP status from Finnhub.
    #[error("api error (status {status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Response body, truncated.
        message: String,
    },

    /// Transport-level failure (connect, TLS, body read, …).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The request timed out.
    #[error("request timed out: {0}")]
    Timeout(reqwest::Error),

    /// The response body could not be deserialized into the expected model.
    #[error("failed to decode response: {0}")]
    Decode(serde_json::Error),
}

impl Error {
    /// Whether the call is worth retrying after a wait: 429, 5xx, timeouts,
    /// and connect failures. The crate never retries automatically.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::RateLimitExceeded { .. } | Error::Timeout(_) => true,
            Error::Api { status, .. } => (500..600).contains(status),
            Error::Http(e) => e.is_connect() || e.is_timeout(),
            Error::Unauthorized | Error::NotFound | Error::Decode(_) => false,
        }
    }

    /// Server-suggested wait before retrying, in seconds. Only
    /// [`Error::RateLimitExceeded`] carries this.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Error::RateLimitExceeded { retry_after } => *retry_after,
            _ => None,
        }
    }
}
