//! Stock endpoint models.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Real-time price snapshot for a symbol (`/quote`).
///
/// Upstream uses compact keys `c/d/dp/h/l/o/pc/t`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Current price (upstream key `c`).
    #[serde(rename = "c")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub current_price: Option<Decimal>,
    /// Change (upstream key `d`).
    #[serde(rename = "d")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    /// Percent change (upstream key `dp`).
    #[serde(rename = "dp")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub percent_change: Option<Decimal>,
    /// High price of the day (upstream key `h`).
    #[serde(rename = "h")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    /// Low price of the day (upstream key `l`).
    #[serde(rename = "l")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    /// Open price of the day (upstream key `o`).
    #[serde(rename = "o")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    /// Previous close price (upstream key `pc`).
    #[serde(rename = "pc")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_close: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub market_capitalization: Option<Decimal>,
    pub name: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "shareOutstanding")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub share_outstanding: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub actual: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub estimate: Option<Decimal>,
    /// Period, `YYYY-MM-DD`.
    pub period: Option<String>,
    pub quarter: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub surprise: Option<Decimal>,
    #[serde(rename = "surprisePercent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub surprise_percent: Option<Decimal>,
    pub symbol: Option<String>,
    pub year: Option<i64>,
}
