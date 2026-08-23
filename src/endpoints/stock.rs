//! Stock endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::stock::{
    BasicFinancials, CompanyProfile, EpsSurprise, MarketHolidays, MarketStatus, Quote,
    RecommendationTrend, StockSymbol,
};

/// Handle for the `stock` endpoint group (`client.stock()`).
#[derive(Clone)]
pub struct StockEndpoints {
    inner: Arc<ClientInner>,
}

impl StockEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Real-time price snapshot for a symbol (`/quote`).
    pub async fn quote(&self, symbol: &str) -> Result<Quote> {
        self.inner.get("/quote", &[("symbol", symbol)]).await
    }

    /// General company information (`/stock/profile2`).
    pub async fn company_profile2(&self, symbol: &str) -> Result<CompanyProfile> {
        self.inner
            .get("/stock/profile2", &[("symbol", symbol)])
            .await
    }

    /// Supported symbols on an exchange (`/stock/symbol?exchange=US`).
    pub async fn symbols(&self, exchange: &str) -> Result<Vec<StockSymbol>> {
        self.inner
            .get("/stock/symbol", &[("exchange", exchange)])
            .await
    }

    /// Market open/close status (`/stock/market-status?exchange=US`).
    pub async fn market_status(&self, exchange: &str) -> Result<MarketStatus> {
        self.inner
            .get("/stock/market-status", &[("exchange", exchange)])
            .await
    }

    /// Market holidays (`/stock/market-holiday?exchange=US`).
    pub async fn market_holidays(&self, exchange: &str) -> Result<MarketHolidays> {
        self.inner
            .get("/stock/market-holiday", &[("exchange", exchange)])
            .await
    }

    /// Company peers (`/stock/peers`). Returns a list of peer symbols.
    pub async fn peers(&self, symbol: &str) -> Result<Vec<String>> {
        self.inner.get("/stock/peers", &[("symbol", symbol)]).await
    }

    /// Company basic financials, all metrics (`/stock/metric?metric=all`).
    pub async fn basic_financials(&self, symbol: &str) -> Result<BasicFinancials> {
        self.inner
            .get("/stock/metric", &[("symbol", symbol), ("metric", "all")])
            .await
    }

    /// Latest analyst recommendation trends (`/stock/recommendation`).
    pub async fn recommendation_trends(&self, symbol: &str) -> Result<Vec<RecommendationTrend>> {
        self.inner
            .get("/stock/recommendation", &[("symbol", symbol)])
            .await
    }

    /// Historical quarterly earnings surprises (`/stock/earnings`).
    pub async fn eps_surprises(&self, symbol: &str) -> Result<Vec<EpsSurprise>> {
        self.inner
            .get("/stock/earnings", &[("symbol", symbol)])
            .await
    }
}
