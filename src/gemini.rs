use super::*;
use crate::prices::Prices;
use serde::{Deserialize, Serialize};

const GEMINI_TICKER_V2_URL_BASE: &str = "https://api.sandbox.gemini.com/v2/ticker/";

// Structure of the expected JSON from the Gemini API
#[derive(Serialize, Deserialize)]
struct TickerResponse {
    pub symbol: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    // The last 24 hours of prices in descending order
    pub changes: Vec<String>,
    pub bid: String,
    pub ask: String,
}

/// Use REST APIs to retrieve the prices from the last 24 hours
pub async fn get_startup_prices(symbol: &str) -> Result<Prices> {
    // Make a GET request to https://api.gemini.com/v2/ticker:symbol for the last 24 hours
    let api_url = GEMINI_TICKER_V2_URL_BASE.to_string() + symbol;
    let response = reqwest::get(api_url).await?.text().await?;
    let parsed_json: TickerResponse = serde_json::from_str(&response)?;
    // Parse the changes strings as an f64
    let price_values = parsed_json
        .changes
        .iter()
        .map(|val| val.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    // Convert to array
    let price_values: [f64; NUM_HOURS] = price_values.try_into().unwrap();
    Prices::new(symbol, price_values)
}

#[cfg(test)]
mod tests {
    use crate::gemini::get_startup_prices;

    #[tokio::test]
    async fn gemini_ticker_v2() {
        // Test a REST API call and parse of the response
        let prices = get_startup_prices("BTCUSD").await.unwrap();
        println!("{:?}", prices);
    }
}
