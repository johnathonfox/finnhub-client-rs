//! Crypto endpoint models.

use serde::{Deserialize, Serialize};

/// A crypto symbol on an exchange (`/crypto/symbol`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSymbol {
    pub description: Option<String>,
    #[serde(rename = "displaySymbol")]
    pub display_symbol: Option<String>,
    pub symbol: Option<String>,
}
