//! Authentication method selection (ADR-0002).
//!
//! The API key itself is stored as a `secrecy::SecretString` inside the client
//! so it can never appear in `Debug` output. By default the key travels in the
//! `X-Finnhub-Token` request header, keeping it out of URLs and the logs that
//! contain them.

/// How the API key is attached to outgoing requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthMethod {
    /// Send the key as the `X-Finnhub-Token` request header (default).
    #[default]
    Header,
    /// Append the key as the `token` query parameter. Opt-in for environments
    /// (e.g. some corporate proxies) that require credentials in the URL.
    UrlParameter,
}
