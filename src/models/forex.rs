//! Forex endpoint models.

use serde::{Deserialize, Serialize};

/// A forex symbol on an exchange (`/forex/symbol`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexSymbol {
    pub description: Option<String>,
    #[serde(rename = "displaySymbol")]
    pub display_symbol: Option<String>,
    pub symbol: Option<String>,
}
