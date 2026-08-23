//! Thin endpoint handles (ADR-0005).
//!
//! Each handle holds a clone of the client's shared state and contains no
//! logic beyond path building and parameter passing — all HTTP, auth, and
//! rate limiting live in `client.rs`'s single `send` path. Accessed via
//! `client.stock()`, `client.news()`, etc.

pub mod calendar;
pub mod crypto;
pub mod forex;
pub mod misc;
pub mod news;
pub mod stock;

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;

/// Raw escape hatch for endpoints not yet covered by typed models.
///
/// Goes through the same authenticated, rate-limited request path as every
/// typed endpoint.
#[derive(Clone)]
pub struct RawEndpoints {
    inner: Arc<ClientInner>,
}

impl RawEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// GET `path` (e.g. `/quote`) with query parameters, returning the raw
    /// JSON body.
    pub async fn get(&self, path: &str, params: &[(&str, &str)]) -> Result<serde_json::Value> {
        self.inner.get(path, params).await
    }
}
