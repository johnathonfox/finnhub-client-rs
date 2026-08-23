//! Forex endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::forex::ForexSymbol;

/// Handle for the `forex` endpoint group (`client.forex()`).
#[derive(Clone)]
pub struct ForexEndpoints {
    inner: Arc<ClientInner>,
}

impl ForexEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Supported forex exchanges (`/forex/exchange`).
    pub async fn exchanges(&self) -> Result<Vec<String>> {
        self.inner.get("/forex/exchange", &[]).await
    }

    /// Supported symbols on a forex exchange (`/forex/symbol?exchange=oanda`).
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<ForexSymbol>> {
        self.inner
            .get("/forex/symbol", &[("exchange", exchange)])
            .await
    }
}
