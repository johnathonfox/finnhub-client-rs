//! Miscellaneous endpoint models.

use serde::{Deserialize, Serialize};

/// Symbol search results (`/search`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLookup {
    pub count: Option<u64>,
    #[serde(default)]
    pub result: Vec<SymbolLookupResult>,
}

/// A single symbol search hit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLookupResult {
    pub description: Option<String>,
    #[serde(rename = "displaySymbol")]
    pub display_symbol: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
}

/// A supported country (`/country`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// ISO 3166-1 alpha-2 code.
    pub code2: Option<String>,
    /// ISO 3166-1 alpha-3 code.
    pub code3: Option<String>,
    /// ISO 3166-1 numeric code.
    #[serde(rename = "codeNo")]
    pub code_no: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
}

/// An FDA advisory committee meeting (`/fda-committee-calendar`).
///
/// Upstream fields vary between events, so everything is optional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdaCalendarEntry {
    #[serde(rename = "applicationType")]
    pub application_type: Option<String>,
    #[serde(rename = "applicationTypeDescription")]
    pub application_type_description: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "companyExchange")]
    pub company_exchange: Option<String>,
    pub committee: Option<String>,
    #[serde(rename = "committeeDescription")]
    pub committee_description: Option<String>,
    pub drug: Option<String>,
    #[serde(rename = "fromDate")]
    pub from_date: Option<String>,
    #[serde(rename = "toDate")]
    pub to_date: Option<String>,
    #[serde(rename = "isEventVerified")]
    pub is_event_verified: Option<bool>,
    pub result: Option<String>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "urlDescription")]
    pub url_description: Option<String>,
}
