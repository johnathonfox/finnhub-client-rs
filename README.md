# finnhub-client-rs

Async Rust client for the [Finnhub.io](https://finnhub.io) financial data API
(stocks, forex, crypto, news, calendars, and more).

- **Async-first**: built on `reqwest` (rustls) + `tokio`
- **Type-safe**: strongly typed models with meaningful field names (Finnhub's
  compact keys like `c`/`h`/`l` become `current_price`/`high`/`low`)
- **Secure by default**: API key sent via `X-Finnhub-Token` header, never in
  URLs or `Debug` output
- **Rate-limited**: built-in token bucket respecting Finnhub's 30
  requests/second ceiling (all plans)
- **Typed errors**: distinguish 401/403/404/429/5xx; `is_retryable()` and
  `retry_after()` helpers — retry policy stays in your hands

## Quick start

```toml
[dependencies]
finnhub-client-rs = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use finnhub_client_rs::{FinnhubClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = FinnhubClient::new(std::env::var("FINNHUB_API_KEY").unwrap());

    let quote = client.stock().quote("AAPL").await?;
    println!("AAPL: {:?}", quote.current_price);

    let news = client.news().company_news("AAPL", "2026-08-01", "2026-08-22").await?;
    println!("{} articles", news.len());

    // Endpoints without typed models yet are reachable via the raw escape hatch:
    let v = client.raw().get("/stock/candle", &[("symbol", "AAPL"), ("resolution", "D")]).await?;
    println!("{v}");

    Ok(())
}
```

Get a free API key at <https://finnhub.io> (free tier: 30 API calls/second).

## Coverage

v0.1.0 implements all free-tier REST endpoints (quotes, symbol lookup, stock
symbols, market status/holidays, market & company news, company profile, peers,
basic financials, recommendation trends, EPS surprises, earnings & IPO
calendars, forex/crypto symbols, country list, FDA calendar). See
[`docs/api-coverage.md`](docs/api-coverage.md) for the full checklist.
Premium endpoints and WebSocket streaming are planned; anything not yet typed
works through `client.raw()`.

## Design docs

Architecture decisions live in [`docs/adr/`](docs/adr/), domain vocabulary in
[`CONTEXT.md`](CONTEXT.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
