//! Crypto endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::crypto::CryptoSymbol;

/// Handle for the `crypto` endpoint group (`client.crypto()`).
#[derive(Clone)]
pub struct CryptoEndpoints {
    inner: Arc<ClientInner>,
}

impl CryptoEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Supported crypto exchanges (`/crypto/exchange`).
    pub async fn exchanges(&self) -> Result<Vec<String>> {
        self.inner.get("/crypto/exchange", &[]).await
    }

    /// Supported symbols on a crypto exchange
    /// (`/crypto/symbol?exchange=binance`).
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<CryptoSymbol>> {
        self.inner
            .get("/crypto/symbol", &[("exchange", exchange)])
            .await
    }
}
