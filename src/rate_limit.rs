//! Client-side rate limiting with `governor` (ADR-0004).
//!
//! Finnhub enforces a hard ceiling of 30 API calls/second on every plan, so a
//! GCRA token bucket is awaited inside the client's single request path. All
//! governor wiring lives in this module so a governor upgrade touches one file.

use std::num::NonZeroU32;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};

/// The concrete limiter type shared by the client and every endpoint handle.
pub(crate) type SharedRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Build a limiter allowing `requests_per_second` sustained requests.
pub(crate) fn rate_limiter(requests_per_second: NonZeroU32) -> SharedRateLimiter {
    RateLimiter::direct(Quota::per_second(requests_per_second))
}
