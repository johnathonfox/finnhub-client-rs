# 0004: Client-side rate limiting with `governor`

- Status: proposed
- Date: 2026-08-22

## Context / Problem

Finnhub returns HTTP 429 when a limit is exceeded. Per the [official Limits doc](https://finnhub.io/docs/api/rate-limit): *"On top of all plan's limit, there is a 30 API calls/second limit."* — the 30 req/s ceiling applies to **every** plan; paid tiers raise the broader per-minute/quota allowances, not the per-second cap. Any realistic use of this crate — backfilling candles, fanning out quotes across a symbol list — will hit that ceiling immediately if requests go out unthrottled. Waiting for 429s and backing off is wasteful: the limit is known, fixed, and cheap to respect client-side.

## Options Considered

### Option A: `governor` token bucket, checked before every request

Use the [`governor`](https://docs.rs/governor) crate (GCRA rate limiter) with a default quota of 30 req/s and small burst capacity; every outgoing request awaits the limiter inside the client's single request path. Configurable via `ClientConfig` for paid tiers.

- Pros: battle-tested, lock-free-ish, async-aware (`until_ready`); one choke point guarantees no code path bypasses the limit; callers never think about 429s in normal operation.
- Cons: one more dependency; a shared limiter serializes at the permit check (negligible at 30/s).

### Option B: Hand-rolled `tokio::sync::Semaphore` + interval tick

- Pros: no new dependency.
- Cons: we re-implement and re-test GCRA poorly; burst semantics and edge cases (clock changes, refill) are easy to get wrong.

### Option C: No client-side limiting; react to 429

- Pros: simplest code; respects whatever the server's actual limit is.
- Cons: every 429 is a wasted round trip and wasted quota; free-tier users would constantly bounce off the ceiling during batch work. Bad default.

## Decision

**Option A.** A `governor` limiter lives in the client's infrastructure layer and is awaited in the single internal `send` method, so all endpoints are throttled uniformly. Default quota: 30 requests/second — the documented ceiling for all plans — overridable through `ClientConfig` (e.g. to throttle *lower* for shared keys or conservative scripts). The limiter is compositional, not hidden: advanced callers can construct the client with their own limiter to share budget across multiple clients.

## Consequences

Positive:

- Batch fan-out "just works" at maximum legal speed.
- 429 handling (ADR-0003) becomes the rare escape hatch, not the control loop.

Negative:

- A single shared limiter couples all calls from one client; per-endpoint limits (Finnhub has none documented) would need rework.
- `governor` API churn has bitten dependents before; we isolate it behind the internal request path so an upgrade touches one file.
