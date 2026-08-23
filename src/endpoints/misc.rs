//! Miscellaneous endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::misc::{Country, FdaCalendarEntry, SymbolLookup};

/// Handle for the `misc` endpoint group (`client.misc()`).
#[derive(Clone)]
pub struct MiscEndpoints {
    inner: Arc<ClientInner>,
}

impl MiscEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Search for symbols by name or ticker (`/search?q=apple`).
    pub async fn symbol_lookup(&self, query: &str) -> Result<SymbolLookup> {
        self.inner.get("/search", &[("q", query)]).await
    }

    /// Countries supported by Finnhub's fundamental data (`/country`).
    pub async fn country(&self) -> Result<Vec<Country>> {
        self.inner.get("/country", &[]).await
    }

    /// FDA advisory committee calendar (`/fda-committee-calendar`).
    pub async fn fda_calendar(&self) -> Result<Vec<FdaCalendarEntry>> {
        self.inner.get("/fda-committee-calendar", &[]).await
    }
}
