//! `FinnhubClient`: config, HTTP pool, auth injection, rate limiting, and the
//! single internal `send` path every request goes through.

use std::fmt;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

use crate::auth::AuthMethod;
use crate::endpoints::{
    calendar::CalendarEndpoints, crypto::CryptoEndpoints, forex::ForexEndpoints,
    misc::MiscEndpoints, news::NewsEndpoints, stock::StockEndpoints, RawEndpoints,
};
use crate::error::{Error, Result};
use crate::rate_limit::{rate_limiter, SharedRateLimiter};

/// Default Finnhub REST API base URL.
pub const DEFAULT_BASE_URL: &str = "https://finnhub.io/api/v1";

/// Default client-side rate limit: Finnhub's documented per-second ceiling on
/// every plan (ADR-0004).
pub const DEFAULT_REQUESTS_PER_SECOND: u32 = 30;

/// Error response bodies are truncated to this many characters.
const MAX_ERROR_BODY: usize = 200;

/// Client configuration (everything except the API key, which is supplied at
/// construction and stored separately).
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL of the API. Override for testing or proxies.
    pub base_url: String,
    /// How the API key is attached to requests (ADR-0002).
    pub auth_method: AuthMethod,
    /// Client-side rate limit in requests per second (ADR-0004).
    pub requests_per_second: NonZeroU32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            auth_method: AuthMethod::default(),
            requests_per_second: NonZeroU32::new(DEFAULT_REQUESTS_PER_SECOND)
                .expect("default rate limit is non-zero"),
        }
    }
}

impl ClientConfig {
    /// Set a custom base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the authentication method.
    pub fn with_auth_method(mut self, auth_method: AuthMethod) -> Self {
        self.auth_method = auth_method;
        self
    }

    /// Set the client-side rate limit (requests per second).
    pub fn with_requests_per_second(mut self, requests_per_second: NonZeroU32) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
}

/// Shared client state. Cloned (behind `Arc`) into every endpoint handle so
/// auth injection and rate limiting cannot be bypassed.
pub(crate) struct ClientInner {
    http: reqwest::Client,
    api_key: SecretString,
    config: ClientConfig,
    limiter: SharedRateLimiter,
}

impl ClientInner {
    /// The single request path: rate-limit, inject auth, send, map errors.
    async fn send(&self, path: &str, params: &[(&str, &str)]) -> Result<reqwest::Response> {
        self.limiter.until_ready().await;

        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let mut request = self.http.get(&url).query(params);
        request = match self.config.auth_method {
            AuthMethod::Header => request.header("X-Finnhub-Token", self.api_key.expose_secret()),
            AuthMethod::UrlParameter => request.query(&[("token", self.api_key.expose_secret())]),
        };

        let response = request.send().await.map_err(map_reqwest_error)?;
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let retry_after = parse_retry_after(response.headers());
        Err(map_status_error(status, retry_after, response).await)
    }

    /// GET `path` with query `params` and deserialize the JSON body.
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let response = self.send(path, params).await?;
        let body = response.text().await.map_err(map_reqwest_error)?;
        serde_json::from_str(&body).map_err(Error::Decode)
    }
}

/// Map a transport error, stripping the URL so an API key supplied via
/// `AuthMethod::UrlParameter` cannot leak into error messages or Debug output.
fn map_reqwest_error(e: reqwest::Error) -> Error {
    let e = e.without_url();
    if e.is_timeout() {
        Error::Timeout(e)
    } else {
        Error::Http(e)
    }
}

/// Map a non-2xx status per ADR-0003.
async fn map_status_error(
    status: StatusCode,
    retry_after: Option<u64>,
    response: reqwest::Response,
) -> Error {
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Unauthorized,
        StatusCode::NOT_FOUND => Error::NotFound,
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimitExceeded { retry_after },
        _ => {
            let body = response.text().await.unwrap_or_default();
            Error::Api {
                status: status.as_u16(),
                message: body.chars().take(MAX_ERROR_BODY).collect(),
            }
        }
    }
}

/// Parse `Retry-After` (seconds) or `X-RateLimit-Reset` (unix epoch) into a
/// wait duration in seconds.
fn parse_retry_after(headers: &HeaderMap) -> Option<u64> {
    if let Some(value) = headers.get("retry-after") {
        if let Ok(seconds) = value.to_str().ok()?.trim().parse::<u64>() {
            return Some(seconds);
        }
    }
    if let Some(value) = headers.get("x-ratelimit-reset") {
        let reset_at = value.to_str().ok()?.trim().parse::<u64>().ok()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        return Some(reset_at.saturating_sub(now));
    }
    None
}

/// The single entry point: owns the config, HTTP connection pool, API key,
/// and rate limiter. Cheap to clone (everything is behind an `Arc`).
#[derive(Clone)]
pub struct FinnhubClient {
    inner: Arc<ClientInner>,
}

impl FinnhubClient {
    /// Create a client with default configuration (header auth, 30 req/s,
    /// `https://finnhub.io/api/v1`).
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(api_key, ClientConfig::default())
    }

    /// Create a client with custom configuration.
    pub fn with_config(api_key: impl Into<String>, config: ClientConfig) -> Self {
        let inner = ClientInner {
            http: reqwest::Client::new(),
            api_key: SecretString::new(api_key.into().into_boxed_str()),
            limiter: rate_limiter(config.requests_per_second),
            config,
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Stock endpoints: quotes, profiles, symbols, market status, peers,
    /// financials, recommendations, earnings surprises.
    pub fn stock(&self) -> StockEndpoints {
        StockEndpoints::new(Arc::clone(&self.inner))
    }

    /// News endpoints: market and company news.
    pub fn news(&self) -> NewsEndpoints {
        NewsEndpoints::new(Arc::clone(&self.inner))
    }

    /// Calendar endpoints: earnings and IPO calendars.
    pub fn calendar(&self) -> CalendarEndpoints {
        CalendarEndpoints::new(Arc::clone(&self.inner))
    }

    /// Forex endpoints: exchanges and symbols.
    pub fn forex(&self) -> ForexEndpoints {
        ForexEndpoints::new(Arc::clone(&self.inner))
    }

    /// Crypto endpoints: exchanges and symbols.
    pub fn crypto(&self) -> CryptoEndpoints {
        CryptoEndpoints::new(Arc::clone(&self.inner))
    }

    /// Miscellaneous endpoints: symbol lookup, countries, FDA calendar.
    pub fn misc(&self) -> MiscEndpoints {
        MiscEndpoints::new(Arc::clone(&self.inner))
    }

    /// Raw escape hatch (ADR-0005): reach endpoints we haven't typed yet.
    pub fn raw(&self) -> RawEndpoints {
        RawEndpoints::new(Arc::clone(&self.inner))
    }
}

impl fmt::Debug for FinnhubClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FinnhubClient")
            .field("base_url", &self.inner.config.base_url)
            .field("auth_method", &self.inner.config.auth_method)
            .field(
                "requests_per_second",
                &self.inner.config.requests_per_second,
            )
            .finish_non_exhaustive()
    }
}
