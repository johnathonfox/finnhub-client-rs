//! Stock endpoint models.

use serde::{Deserialize, Serialize};

/// Real-time price snapshot for a symbol (`/quote`).
///
/// Upstream uses compact keys `c/d/dp/h/l/o/pc/t`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Current price (upstream key `c`).
    #[serde(rename = "c")]
    pub current_price: Option<f64>,
    /// Change (upstream key `d`).
    #[serde(rename = "d")]
    pub change: Option<f64>,
    /// Percent change (upstream key `dp`).
    #[serde(rename = "dp")]
    pub percent_change: Option<f64>,
    /// High price of the day (upstream key `h`).
    #[serde(rename = "h")]
    pub high: Option<f64>,
    /// Low price of the day (upstream key `l`).
    #[serde(rename = "l")]
    pub low: Option<f64>,
    /// Open price of the day (upstream key `o`).
    #[serde(rename = "o")]
    pub open: Option<f64>,
    /// Previous close price (upstream key `pc`).
    #[serde(rename = "pc")]
    pub previous_close: Option<f64>,
    /// Unix timestamp of the quote (upstream key `t`).
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Company profile (`/stock/profile2`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyProfile {
    pub country: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "estimateCurrency")]
    pub estimate_currency: Option<String>,
    pub exchange: Option<String>,
    #[serde(rename = "finnhubIndustry")]
    pub finnhub_industry: Option<String>,
    /// IPO date, `YYYY-MM-DD`.
    pub ipo: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "marketCapitalization")]
    pub market_capitalization: Option<f64>,
    pub name: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "shareOutstanding")]
    pub share_outstanding: Option<f64>,
    pub ticker: Option<String>,
    pub weburl: Option<String>,
}

/// A listed stock symbol (`/stock/symbol`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSymbol {
    pub currency: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "displaySymbol")]
    pub display_symbol: Option<String>,
    pub figi: Option<String>,
    pub mic: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
}

/// Market open/close status (`/stock/market-status`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatus {
    pub exchange: Option<String>,
    /// Holiday name if the market is closed for a holiday.
    pub holiday: Option<String>,
    #[serde(rename = "isOpen")]
    pub is_open: Option<bool>,
    /// Current session (`pre-market`, `regular`, `post-market`), if any.
    pub session: Option<String>,
    /// Unix timestamp of the status (upstream key `t`).
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    pub timezone: Option<String>,
}

/// Market holiday list (`/stock/market-holiday`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHolidays {
    #[serde(default)]
    pub data: Vec<MarketHoliday>,
}

/// A single market holiday entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHoliday {
    /// Holiday date, `YYYY-MM-DD`.
    #[serde(rename = "atDate")]
    pub at_date: Option<String>,
    pub exchange: Option<String>,
    #[serde(rename = "holidayName")]
    pub holiday_name: Option<String>,
    pub status: Option<String>,
    /// Abbreviated trading hours on the holiday, if any.
    #[serde(rename = "tradingHour")]
    pub trading_hour: Option<String>,
}

/// Company basic financials / metrics (`/stock/metric?metric=all`).
///
/// Finnhub returns a large, evolving map of metric values; `metric` and
/// `series` are kept as raw JSON to avoid modeling hundreds of keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFinancials {
    /// Map of metric name to value (varies by `metric` query parameter).
    pub metric: Option<serde_json::Value>,
    #[serde(rename = "metricType")]
    pub metric_type: Option<String>,
    /// Historical series data, when requested.
    pub series: Option<serde_json::Value>,
    pub symbol: Option<String>,
}

/// Analyst recommendation trend for a period (`/stock/recommendation`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationTrend {
    pub buy: Option<i64>,
    pub hold: Option<i64>,
    /// Period, `YYYY-MM-DD`.
    pub period: Option<String>,
    pub sell: Option<i64>,
    #[serde(rename = "strongBuy")]
    pub strong_buy: Option<i64>,
    #[serde(rename = "strongSell")]
    pub strong_sell: Option<i64>,
    pub symbol: Option<String>,
}

/// Quarterly earnings surprise (`/stock/earnings`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsSurprise {
    pub actual: Option<f64>,
    pub estimate: Option<f64>,
    /// Period, `YYYY-MM-DD`.
    pub period: Option<String>,
    pub quarter: Option<i64>,
    pub surprise: Option<f64>,
    #[serde(rename = "surprisePercent")]
    pub surprise_percent: Option<f64>,
    pub symbol: Option<String>,
    pub year: Option<i64>,
}
