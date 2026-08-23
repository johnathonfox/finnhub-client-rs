# 0002: Authentication — `X-Finnhub-Token` header by default

- Status: proposed
- Date: 2026-08-22

## Context / Problem

Every Finnhub REST request must carry an API key. Finnhub accepts the key two ways:

- `X-Finnhub-Token: <key>` request header
- `?token=<key>` query parameter

The key is a secret. Where it travels affects how easily it leaks (logs, proxies, browser history, error messages) and how the client code stays testable. The key must also come from configuration, never be hardcoded; this repo already keeps a `.env` with `FINNHUB_API_KEY` for local development.

## Options Considered

### Option A: Header auth by default, query param opt-in

Send `X-Finnhub-Token` on every request; expose `AuthMethod::UrlParameter` in `ClientConfig` for environments that need it.

- Pros: headers don't appear in URLs, so keys stay out of access logs, `tracing` spans, and reqwest error messages that include the URL; matches what the prior-art `finnhub` crate settled on.
- Cons: marginally more code (an auth enum and an injection point per request).

### Option B: Query param only

Append `?token=...` to every request.

- Pros: trivially simple; no header handling.
- Cons: the secret lands in URLs everywhere — logs, debug output, error chains. Unacceptable default.

### Option C: Header only, no opt-in

- Pros: simplest secure option.
- Cons: some corporate proxies / tooling expect credentials in the URL; cheap to support both, so there is no reason to hard-commit.

## Decision

Default to **`X-Finnhub-Token` header**; support `AuthMethod::UrlParameter` via `ClientConfig`. The key is supplied at construction (`FinnhubClient::new(api_key)`); examples and tests read it from the `FINNHUB_API_KEY` environment variable. The key is stored in the client as a `secrecy`-style wrapper (or at minimum excluded from `Debug` output) so it cannot be printed accidentally.

## Consequences

Positive:

- Secrets stay out of URLs and the logs that contain them.
- Both Finnhub-supported auth mechanisms remain available.

Negative:

- A small `auth` module and per-request injection logic to maintain.
- If we hand-rolled the key wrapper instead of using `secrecy`, we'd own the guarantee; prefer the crate.
