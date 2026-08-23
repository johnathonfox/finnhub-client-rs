//! News endpoint models.

use serde::{Deserialize, Serialize};

/// A market or company news article (`/news`, `/company-news`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    /// News category.
    pub category: Option<String>,
    /// Published time as Unix seconds.
    pub datetime: Option<i64>,
    pub headline: Option<String>,
    pub id: Option<i64>,
    /// Thumbnail image URL.
    pub image: Option<String>,
    /// Related symbols and tickers mentioned in the article.
    pub related: Option<String>,
    pub source: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
}
