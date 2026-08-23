# 0005: Module layout — typed domain models separate from transport

- Status: proposed
- Date: 2026-08-22

## Context / Problem

Finnhub exposes 100+ endpoints across stocks, forex, crypto, news, calendars, technical analysis, ETFs, and alternative data. Responses use compact JSON keys (`{"c": 178.32, "h": 179.0, ...}` for a quote) and liberal `null`s. Two structural risks:

1. Without grouping, a 100-endpoint client becomes an unnavigable flat file.
2. If serde/reqwest types leak into the data structures callers use, transport concerns contaminate the domain surface (and vice versa).

Per this project's architecture rules: domain logic must never import from infrastructure, and adapters stay thin.

## Options Considered

### Option A: `models/` + `endpoints/` split, grouped by Finnhub domain

```
src/
├── client.rs        # FinnhubClient: config, auth injection, rate limiting, single send()
├── auth.rs          # AuthMethod (header vs query param)
├── error.rs         # Error enum (ADR-0003)
├── rate_limit.rs    # governor wiring (ADR-0004)
├── models/          # pure data types, serde only — no reqwest/tokio imports
│   ├── stock.rs     # Quote, Candle, CompanyProfile, ...
│   ├── news.rs
│   └── ...
└── endpoints/       # thin async fns: build path + params, call client.send, deserialize
    ├── stock.rs
    ├── news.rs
    └── ...
```

`FinnhubClient` exposes accessor methods (`client.stock().quote("AAPL")`) returning lightweight endpoint handles, so the public API mirrors Finnhub's own documentation grouping.

- Pros: one obvious home per endpoint; `models/` is independently testable against recorded JSON fixtures; the dependency rule is mechanically enforceable (`models/` never imports `client`/`reqwest`); matches both Finnhub's doc structure and prior art.
- Cons: some tiny modules; endpoint handles add one layer of indirection.

### Option B: Flat single `api.rs` with `#[derive(Deserialize)]` inline

- Pros: fewest files.
- Cons: collapses at 100+ endpoints; inline anonymous response types leak into caller code and can't be reused or fixture-tested; no seam between domain and transport.

### Option C: Fully generic `get<T>(path, params)` public API only

- Pros: zero per-endpoint code; every Finnhub endpoint reachable day one.
- Cons: callers deserialize `serde_json::Value` or supply their own types — no type safety, no discoverability; a client crate that makes users read upstream JSON docs has abdicated its job. (Useful as an *additional* escape hatch, not as the interface.)

## Decision

**Option A**, with these conventions:

- Models are plain data: `#[derive(Debug, Clone, Serialize, Deserialize)]`, no behavior beyond trivial helpers.
- Finnhub's cryptic keys are renamed to meaningful Rust fields via `#[serde(rename = "c")]` etc., with doc comments quoting the upstream key.
- Fields are `Option<T>` unless Finnhub's docs guarantee presence; timestamps are `i64` Unix seconds (a `chrono`/`time` newtype may come later behind a feature).
- `endpoints/` functions contain no logic beyond parameter validation and path building — all HTTP, auth, and rate limiting live in `client.rs`.
- A public `client.raw().get(path, params) -> Result<serde_json::Value>` escape hatch covers endpoints we haven't typed yet.

WebSocket support is explicitly **out of scope** for the first milestone; it will be feature-gated (`websocket`) when added.

## Consequences

Positive:

- The public API is discoverable (`client.stock().…`, `client.news().…`) and mirrors upstream docs.
- Fixtures + `wiremock` tests exercise models and endpoint mapping without network access.
- The domain/infrastructure dependency rule has a physical home: `models/` vs everything else.

Negative:

- Each new endpoint means touching three places (model, endpoint, tests) — accepted as the price of type safety.
- 100% coverage is a long tail; the raw escape hatch carries users over gaps.
