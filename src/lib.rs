//! Async Rust client for the [Finnhub.io](https://finnhub.io) financial data API.
//!
//! Create a [`FinnhubClient`] with your API key, then call endpoints through the
//! accessor methods that mirror Finnhub's own documentation grouping:
//!
//! ```no_run
//! use finnhub_client_rs::FinnhubClient;
//!
//! # async fn example() -> finnhub_client_rs::Result<()> {
//! let client = FinnhubClient::new("your-api-key");
//! let quote = client.stock().quote("AAPL").await?;
//! println!("{:?}", quote.current_price);
//! # Ok(())
//! # }
//! ```
//!
//! All HTTP goes through a single internal request path that injects
//! authentication (ADR-0002) and enforces a client-side rate limit
//! (ADR-0004). The crate never retries on its own; use [`Error::is_retryable`]
//! and [`Error::retry_after`] to build your own backoff policy (ADR-0003).

mod auth;
mod client;
mod error;
mod rate_limit;

pub mod endpoints;
pub mod models;

pub use auth::AuthMethod;
pub use client::{ClientConfig, FinnhubClient};
pub use error::{Error, Result};
