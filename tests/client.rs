//! Wiremock-based integration tests. No live network access.

use rust_decimal_macros::dec;
use std::num::NonZeroU32;

use finnhub_client_rs::{AuthMethod, ClientConfig, Error, FinnhubClient};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TEST_KEY: &str = "test-key";

fn test_client(server: &MockServer) -> FinnhubClient {
    FinnhubClient::with_config(
        TEST_KEY,
        ClientConfig::default()
            .with_base_url(server.uri())
            .with_requests_per_second(NonZeroU32::new(10_000).unwrap()),
    )
}

fn quote_fixture() -> serde_json::Value {
    serde_json::json!({
        "c": 178.1,
        "d": 1.2,
        "dp": 0.68,
        "h": 179.0,
        "l": 177.0,
        "o": 177.5,
        "pc": 176.9,
        "t": 1690000000
    })
}

#[tokio::test]
async fn sends_api_key_as_header_by_default() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/quote"))
        .and(query_param("symbol", "AAPL"))
        .and(header("X-Finnhub-Token", TEST_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(quote_fixture()))
        .expect(1)
        .mount(&server)
        .await;

    let client = test_client(&server);
    client.stock().quote("AAPL").await.unwrap();

    // verify() runs on drop; assert explicitly for a clear failure.
    server.verify().await;
}

#[tokio::test]
async fn sends_api_key_as_query_param_when_url_parameter_auth() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/quote"))
        .and(query_param("symbol", "AAPL"))
        .and(query_param("token", TEST_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(quote_fixture()))
        .expect(1)
        .mount(&server)
        .await;

    let client = FinnhubClient::with_config(
        TEST_KEY,
        ClientConfig::default()
            .with_base_url(server.uri())
            .with_auth_method(AuthMethod::UrlParameter)
            .with_requests_per_second(NonZeroU32::new(10_000).unwrap()),
    );
    client.stock().quote("AAPL").await.unwrap();

    server.verify().await;
}

#[tokio::test]
async fn maps_429_to_rate_limit_exceeded_with_retry_after() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "17")
                .set_body_string("rate limit exceeded"),
        )
        .mount(&server)
        .await;

    let client = test_client(&server);
    let err = client.stock().quote("AAPL").await.unwrap_err();
    match err {
        Error::RateLimitExceeded { retry_after } => assert_eq!(retry_after, Some(17)),
        other => panic!("expected RateLimitExceeded, got {other:?}"),
    }
    assert!(err.is_retryable());
    assert_eq!(err.retry_after(), Some(17));
}

#[tokio::test]
async fn maps_401_to_unauthorized() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401).set_body_string("invalid api key"))
        .mount(&server)
        .await;

    let client = test_client(&server);
    let err = client.stock().quote("AAPL").await.unwrap_err();
    assert!(matches!(err, Error::Unauthorized));
    assert!(!err.is_retryable());
}

#[tokio::test]
async fn quote_deserializes_compact_keys_into_renamed_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/quote"))
        .respond_with(ResponseTemplate::new(200).set_body_json(quote_fixture()))
        .mount(&server)
        .await;

    let client = test_client(&server);
    let quote = client.stock().quote("AAPL").await.unwrap();

    assert_eq!(quote.current_price, Some(dec!(178.1)));
    assert_eq!(quote.change, Some(dec!(1.2)));
    assert_eq!(quote.percent_change, Some(dec!(0.68)));
    assert_eq!(quote.high, Some(dec!(179.0)));
    assert_eq!(quote.low, Some(dec!(177.0)));
    assert_eq!(quote.open, Some(dec!(177.5)));
    assert_eq!(quote.previous_close, Some(dec!(176.9)));
    assert_eq!(quote.timestamp, Some(1690000000));
}

#[tokio::test]
async fn company_news_sends_symbol_and_date_range_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/company-news"))
        .and(query_param("symbol", "AAPL"))
        .and(query_param("from", "2024-01-01"))
        .and(query_param("to", "2024-01-31"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "category": "company",
                "datetime": 1706600000,
                "headline": "Apple news",
                "id": 123456,
                "image": "https://example.com/img.png",
                "related": "AAPL",
                "source": "Example",
                "summary": "Summary",
                "url": "https://example.com/article"
            }
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let client = test_client(&server);
    let news = client
        .news()
        .company_news("AAPL", "2024-01-01", "2024-01-31")
        .await
        .unwrap();

    assert_eq!(news.len(), 1);
    assert_eq!(news[0].headline.as_deref(), Some("Apple news"));
    assert_eq!(news[0].datetime, Some(1706600000));

    server.verify().await;
}

#[tokio::test]
async fn api_key_does_not_appear_in_debug_output() {
    let client = test_client(&MockServer::start().await);
    let debug = format!("{client:?}");
    assert!(!debug.contains(TEST_KEY));
}
