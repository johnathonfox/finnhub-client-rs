//! Print the current quote for AAPL.
//!
//! Reads the API key from the `FINNHUB_API_KEY` environment variable
//! (a `.env` file in the working directory also works):
//!
//! ```sh
//! cargo run --example quote
//! ```

use finnhub_client_rs::FinnhubClient;

#[tokio::main]
async fn main() -> finnhub_client_rs::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");

    let client = FinnhubClient::new(api_key);
    let quote = client.stock().quote("AAPL").await?;

    println!("AAPL quote: {quote:#?}");
    Ok(())
}
