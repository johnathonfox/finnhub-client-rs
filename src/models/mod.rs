//! Typed response models (ADR-0005).
//!
//! Pure serde data types mirroring Finnhub's JSON responses — no reqwest,
//! tokio, governor, or client/auth/rate_limit imports. Finnhub's compact keys
//! are renamed to meaningful Rust fields via `#[serde(rename)]`; fields are
//! `Option<T>` unless upstream guarantees presence; timestamps are `i64` Unix
//! seconds.

pub mod calendar;
pub mod crypto;
pub mod forex;
pub mod misc;
pub mod news;
pub mod stock;
