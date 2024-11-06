use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    #[serde(flatten)]
    prices: std::collections::HashMap<String, CurrencyPrices>,
}

#[derive(Deserialize, Debug)]
struct CurrencyPrices {
    usd: f64,
}

pub async fn fetch_price(symbol: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        symbol.to_lowercase()
    );

    log::info!("Fetching price for {}", symbol);
    
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;

    match resp.status() {
        status if status.is_success() => {
            let price_response: PriceResponse = resp.json().await?;
            price_response
                .prices
                .get(&symbol.to_lowercase())
                .map(|prices| prices.usd)
                .ok_or_else(|| format!("Symbol {} not found in response", symbol).into())
        }
        status if status.as_u16() == 429 => {
            Err("Rate limit exceeded. Please try again later.".into())
        }
        status if status.as_u16() == 404 => {
            Err(format!("Symbol {} not found", symbol).into())
        }
        status => {
            Err(format!("Failed to fetch data: HTTP {}", status).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::{method, path}, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_price_success() {
        let mock_server = MockServer::start().await;

        let response_body = r#"
        {
            "prices": {
                "btc": {
                    "usd": 20000.0
                }
            }
        }"#;

        Mock::given(method("GET"))
            .and(path("/api/v3/simple/price"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(response_body, "application/json"))
            .mount(&mock_server)
            .await;

        let url = format!("{}/api/v3/simple/price", &mock_server.uri());
        let price = fetch_price_with_url("BTC", &url).await.unwrap();
        assert_eq!(price, 20000.0);
    }

    async fn fetch_price_with_url(symbol: &str, url: &str) -> Result<f64, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;

        if resp.status().is_success() {
            let price_response: PriceResponse = resp.json().await?;
            price_response
                .prices
                .get(&symbol.to_lowercase())
                .map(|prices| prices.usd)
                .ok_or_else(|| format!("Symbol {} not found", symbol).into())
        } else {
            Err(format!("Failed to fetch data: HTTP {}", resp.status()).into())
        }
    }
} 
