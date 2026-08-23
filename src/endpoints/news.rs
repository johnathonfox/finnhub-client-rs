//! News endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::news::NewsArticle;

/// Handle for the `news` endpoint group (`client.news()`).
#[derive(Clone)]
pub struct NewsEndpoints {
    inner: Arc<ClientInner>,
}

impl NewsEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Latest market news (`/news?category=general`; other categories:
    /// `forex`, `crypto`, `merger`).
    pub async fn market_news(&self, category: &str) -> Result<Vec<NewsArticle>> {
        self.inner.get("/news", &[("category", category)]).await
    }

    /// Company-specific news between two dates, `YYYY-MM-DD`
    /// (`/company-news`). Free tier: up to one year of history.
    pub async fn company_news(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> Result<Vec<NewsArticle>> {
        self.inner
            .get(
                "/company-news",
                &[("symbol", symbol), ("from", from), ("to", to)],
            )
            .await
    }
}
