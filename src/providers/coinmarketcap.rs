// /v1/tools/price-conversion

use anyhow::{Context};
use serde::{Deserialize,Serialize};
use reqwest::Url;
use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::currency::Symbol;
use crate::providers::provider::{BaseProvider,Provider};
use std::collections::HashMap;

pub struct CoinMarketCapProvider {
    provider: BaseProvider,
    access_key: String,
}

#[derive(Serialize,Deserialize)]
struct Response {
    data: ResponseData,
}
#[derive(Serialize,Deserialize)]
struct ResponseData {
    symbol: Symbol,
    id: usize,
    name: String,
    amount: Decimal,
    last_updated: String,
    quote: HashMap<Symbol,Quote>,
}
#[derive(Serialize,Deserialize)]
struct Quote {
    price: Decimal,
    last_updated: String
}

#[async_trait]
impl Provider for CoinMarketCapProvider {
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error> {
        Url::parse_with_params(
            &self.provider.base_url,
            &[("symbol", base.to_string()), ("amount", 1.to_string()), ("convert", quote.to_string())]
        ).context("Failed to build URL")
    }
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<Decimal, anyhow::Error> {
        let url = self.build_url(&base, &quote)?;
        let client = reqwest::Client::new();
        let resp = client.get(url)
            .header("X-CMC_PRO_API_KEY", self.access_key.clone())
            .send()
            .await?
            .text()
            .await?;

        let parsed_rate = CoinMarketCapProvider::parse_rate_from_response(&self, &quote, &resp)?;

        Ok(parsed_rate)
    }
    fn parse_rate_from_response(&self, quote: &Symbol, response: &str) -> Result<Decimal, anyhow::Error> {
        Ok(serde_json::from_str::<Response>(response)
            .context("Failed to parse API response")?
            .data.quote
            .get(quote)
            .context("Failed to find quote symbol in parsed API response")?
            .price
        )
    }
}

impl CoinMarketCapProvider {
    pub fn new(access_key: String) -> Self {
        Self {
            provider: BaseProvider {
                name: String::from("coinmarketcap"),
                base_url: String::from("https://pro-api.coinmarketcap.com/v1/tools/price-conversion"),
            },
            access_key,
        }
    }
}
