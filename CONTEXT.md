# CONTEXT

Domain model and shared vocabulary for `finnhub-client-rs`, an async Rust client for the
[Finnhub.io](https://finnhub.io) financial data API.

## Glossary

Terms used across code, issues, and ADRs. Use these exact names; avoid the listed synonyms.

- **Client** — `FinnhubClient`, the single entry point owning config, HTTP connection pool,
  auth, and rate limiting. (Not: "connector", "session", "api object".)
- **API key** — the Finnhub secret token. Sent as the `X-Finnhub-Token` header by default
  (ADR-0002). Read from `FINNHUB_API_KEY` in examples/tests. Never logged.
- **Endpoint group** — Finnhub's own documentation grouping, mirrored in our public API:
  `stock`, `news`, `calendar`, `forex`, `crypto`, `etf`, `index`, `economic`,
  `technical` (a.k.a. scanner upstream), `alternative`, `misc`. Accessed as
  `client.stock()`, `client.news()`, etc. (Not: "service", "resource", "namespace".)
- **Model** — a plain serde data type in `models/` mirroring one Finnhub response
  (e.g. `Quote`, `Candle`, `CompanyProfile`). Contains no behavior and no infrastructure
  imports (ADR-0005).
- **Quote** — Finnhub's real-time price snapshot for a symbol (upstream keys `c/h/l/o/pc/t`,
  renamed to meaningful Rust fields). (Not: "price", "tick" — tick data is a separate,
  premium endpoint.)
- **Candle** — an OHLCV bar (open/high/low/close/volume) with a resolution and time range.
  **Resolution** — the bar size string (`1`, `5`, `15`, `30`, `60`, `D`, `W`, `M`).
- **Symbol** — a Finnhub ticker string as used in requests (`AAPL`, `OANDA:EUR_USD`,
  `BINANCE:BTCUSDT`). Kept as an opaque `String`; we do not validate or parse it.
- **Rate limit** — Finnhub's server-side quota. Per the official docs, **30 API
  calls/second is the ceiling on every plan** ("On top of all plan's limit, there is a
  30 API calls/ second limit"); paid tiers raise broader quota allowances, not the
  per-second cap. Exceeding a limit returns HTTP 429. Enforced client-side with a
  token bucket (ADR-0004).
- **Retryable error** — an `Error` where `is_retryable()` is true (429, 5xx, timeouts,
  connect failures). The crate never retries on its own (ADR-0003).
- **Raw escape hatch** — `client.raw().get(path, params)` returning `serde_json::Value`
  for endpoints not yet covered by typed models (ADR-0005).
- **Premium endpoint** — an endpoint requiring a paid Finnhub plan (marked upstream and in
  our doc comments). Calling one on a free key yields `Error::Unauthorized`/`Error::Api`.

## Prior art

- [jbradenbrown/finnhub](https://github.com/jbradenbrown/finnhub) — comprehensive async
  client (96% endpoint coverage, header auth, governor-style limiting). Our design
  decisions converge with it on reqwest/tokio, header auth, and typed errors.
- [henryboisdequin/finnhub-rs](https://github.com/henryboisdequin/finnhub-rs) — older,
  minimal, effectively unmaintained.
- **No official Rust SDK exists.** Finnhub's official libraries cover Python, Go,
  JavaScript, Ruby, Kotlin, and PHP only — a Rust client fills a real gap.
- Finnhub publishes a **Swagger schema** (linked from the docs Introduction) — useful
  for cross-checking our hand-written models against the source of truth.
- Full endpoint inventory with tier labels: `docs/api-coverage.md`.

## Invariants

- `models/` never imports `reqwest`, `tokio`, `governor`, or anything from `client` /
  `auth` / `rate_limit` (ADR-0005; enforced by dependency linting).
- All HTTP goes through the client's single internal `send` path so auth injection and
  rate limiting cannot be bypassed.
- The API key never appears in `Debug` output, URLs (default config), or error messages.
- Timestamps are `i64` Unix seconds as returned upstream; no implicit timezone handling.

## Out of scope (first milestone)

- WebSocket streaming (`wss://ws.finnhub.io?token=`) — feature-gated later. Design
  constraints to respect when we get there: **1 API key = 1 concurrent connection**,
  trade timestamps are UNIX **milliseconds** (unlike the REST API's seconds), and
  FXCM/Forex.com/FHFX have no streaming — use Forex Candles / All Rates instead.
- Automatic retry, response caching, persistence — application-layer concerns.
