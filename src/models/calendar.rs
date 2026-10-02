//! Calendar endpoint models.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Earnings calendar response (`/calendar/earnings`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsCalendar {
    #[serde(rename = "earningsCalendar", default)]
    pub earnings_calendar: Vec<EarningsEvent>,
}

/// A single earnings announcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsEvent {
    /// Earnings date, `YYYY-MM-DD`.
    pub date: Option<String>,
    #[serde(rename = "epsActual")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub eps_actual: Option<Decimal>,
    #[serde(rename = "epsEstimate")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub eps_estimate: Option<Decimal>,
    /// Announcement timing (`bmo` = before market open, `amc` = after close,
    /// `dmh` = during market hours).
    pub hour: Option<String>,
    pub quarter: Option<i64>,
    #[serde(rename = "revenueActual")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub revenue_actual: Option<Decimal>,
    #[serde(rename = "revenueEstimate")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub revenue_estimate: Option<Decimal>,
    pub symbol: Option<String>,
    pub year: Option<i64>,
}

/// IPO calendar response (`/calendar/ipo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpoCalendar {
    #[serde(rename = "ipoCalendar", default)]
    pub ipo_calendar: Vec<IpoEvent>,
}

/// A single IPO event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpoEvent {
    /// IPO date, `YYYY-MM-DD`.
    pub date: Option<String>,
    pub exchange: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "numberOfShares")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub number_of_shares: Option<Decimal>,
    /// Projected price or price range (e.g. `20.00-23.00`).
    pub price: Option<String>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "totalSharesValue")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_shares_value: Option<Decimal>,
}
