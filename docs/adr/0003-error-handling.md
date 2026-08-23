# 0003: Error handling — typed errors, no built-in retry

- Status: proposed
- Date: 2026-08-22

## Context / Problem

A network client fails in distinguishable ways, and callers need to react differently to each: back off on `429`, fix credentials on `401`/`403`, give up on `4xx`, maybe retry on `5xx`/timeouts. Finnhub signals rate limiting with HTTP 429 and quota exhaustion distinctly from transient server errors. The crate's error design decides how much of that callers can do without string-matching.

## Options Considered

### Option A: `thiserror` enum with classification helpers

One public `Error` enum (`RateLimitExceeded { retry_after }`, `Unauthorized`, `NotFound`, `Api { status, message }`, `Http`, `Decode`, `Timeout`, …), plus `is_retryable()` and `retry_after()` helpers. The crate itself never retries.

- Pros: exhaustive `match` for callers; library stays policy-free — retry strategy belongs to the application, which alone knows its latency/cost trade-offs (same conclusion as the prior-art crate); `thiserror` keeps boilerplate near zero.
- Cons: callers who want retries must write ~15 lines of backoff (we ship an example).

### Option B: Opaque `anyhow::Error`

- Pros: zero design effort.
- Cons: callers cannot distinguish 429 from 401 without downcasting; fine for binaries, wrong for a library.

### Option C: Built-in automatic retry with exponential backoff

- Pros: convenient for naive usage.
- Cons: hides policy (attempt counts, jitter, budget) inside the library; interacts badly with a 30 req/s shared rate limiter — retries amplify pressure; surprising behavior in a building-block crate.

## Decision

**Option A.** Public `finnhub_client_rs::Error` as a `thiserror` enum with `is_retryable()` / `retry_after()` helpers; `pub type Result<T> = std::result::Result<T, Error>`. No automatic retry, no automatic caching — both are application-layer concerns. Map HTTP statuses eagerly: 401/403 → `Unauthorized`, 404 → `NotFound`, 429 → `RateLimitExceeded` (parsing `Retry-After` / `X-RateLimit-Reset` when present).

## Consequences

Positive:

- Callers get compile-time-checked, fine-grained control.
- The crate has no hidden timing behavior; tests stay deterministic.

Negative:

- Every caller needing resilience writes a retry loop (mitigated by a documented `with_retry` example).
- Error variants tied to HTTP semantics; if Finnhub adds non-HTTP failure modes (WebSocket), the enum grows.
