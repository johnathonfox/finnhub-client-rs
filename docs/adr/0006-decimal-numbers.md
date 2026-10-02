# 0006: Numeric model fields are `Decimal`, decoded from the raw JSON token

- Status: accepted
- Date: 2026-10-02

## Context / Problem

Finnhub sends prices, EPS, revenue and share counts as plain JSON numbers.
Decoding them into `f64` rounds every value that has no exact binary form
(`178.1` becomes the nearest double), and the error compounds in any
arithmetic a caller does with it. A consumer that forbids money passing through
a float (capybara-platform's trade-idea data adapters) cannot use `f64` models
at all.

## Options Considered

### Option A: `rust_decimal::Decimal` read from `serde_json::value::RawValue`

Each numeric field is `Option<Decimal>` with
`#[serde(default, deserialize_with = "crate::de::decimal_opt")]`. The helper
parses the token text (`178.1`, `"178.1"`, `1.5e-7`) exactly.

- Pros: exact; no workspace-wide feature flags; the same approach
  `alpaca-rs-client` and `massive-rs` 0.3 use.
- Cons: needs the `serde_json` deserializer itself (this client decodes with
  `serde_json::from_str`, so it does); `Decimal` serializes as a string.

### Option B: `serde_json`'s `arbitrary_precision` feature

- Pros: no per-field attribute.
- Cons: Cargo feature unification turns it on for every crate in a dependent's
  workspace and changes how `serde_json::Value` behaves for all of them.

### Option C: keep `f64`

- Pros: no breaking change.
- Cons: the problem above.

## Decision

Option A. Integer fields stay `i64`. Released as 0.2.0 (breaking).
