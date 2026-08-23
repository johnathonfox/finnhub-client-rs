# 0001: Async HTTP stack — reqwest on tokio

- Status: proposed
- Date: 2026-08-22

## Context / Problem

`finnhub-client-rs` is a client library for the [Finnhub.io](https://finnhub.io) financial data API (`https://finnhub.io/api/v1`). Callers will typically be async Rust services (data pipelines, trading tools, dashboards) that issue many concurrent requests against a 30 requests/second free-tier rate limit. We need an HTTP stack that:

- handles high concurrency efficiently (rate-limited batch fetching is the primary workload),
- has mature TLS, timeouts, and connection pooling,
- is the least surprising choice for Rust consumers of the crate.

Prior art exists ([jbradenbrown/finnhub](https://github.com/jbradenbrown/finnhub), [henryboisdequin/finnhub-rs](https://github.com/henryboisdequin/finnhub-rs)); both are async reqwest/tokio clients. We are building our own client, so we must make this foundational decision deliberately.

## Options Considered

### Option A: reqwest + tokio (async)

The de-facto standard: `reqwest` (with `rustls-tls`) on the `tokio` runtime.

- Pros: idiomatic async ecosystem; excellent connection pooling, timeout, and redirect handling; fits the rate-limited fan-out workload; what consumers of an async crate expect; matches prior art.
- Cons: pulls in the tokio runtime as a dependency consumers must run; heavier compile times than a sync client.

### Option B: ureq (blocking)

A minimal synchronous HTTP client.

- Pros: tiny dependency tree, fast compile, no runtime requirement.
- Cons: concurrency falls on the caller (threads); blocking I/O is a poor fit for hundreds of rate-limited requests; most downstream users of a market-data client are already async — a sync client forces `spawn_blocking` glue.

### Option C: hyper directly

Lower-level HTTP implementation that reqwest wraps.

- Pros: maximal control, fewer layers.
- Cons: significantly more boilerplate (TLS, pooling, body handling all manual); no practical benefit over reqwest for a JSON REST API.

## Decision

Use **reqwest (with `rustls-tls`, `json`) on tokio**. The crate is async-first; no blocking API is provided initially.

## Consequences

Positive:

- Connection reuse and pooling come free via a shared `reqwest::Client` inside our `FinnhubClient`.
- Timeouts, retries-at-the-call-site, and structured JSON are one-liners.
- Matches the ecosystem; contributors need no bespoke knowledge.

Negative:

- Consumers must have a tokio runtime (documented requirement).
- Sync users need `tokio::runtime` or `#[tokio::main]` glue; acceptable for the target audience.
