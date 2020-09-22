use anyhow::{Context};
use serde::{Deserialize,Serialize};
use reqwest::Url;
use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::currency::Symbol;
use crate::providers::provider::{BaseProvider,Provider};
use std::collections::HashMap;

pub struct ExchangeRatesApiProvider(BaseProvider);

#[derive(Serialize,Deserialize)]
struct Response {
    rates: HashMap<Symbol, Decimal>,
    base: Symbol,
    date: String
}

#[async_trait]
impl Provider for ExchangeRatesApiProvider {
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error> {
        Url::parse_with_params(
            &self.0.base_url,
            &[("base", base.to_string()), ("symbols", quote.to_string())]
        ).context("Failed to build URL")
    }
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<Decimal, anyhow::Error> {
        let url = self.build_url(&base, &quote)?;
        let client = reqwest::Client::new();
        let resp = client.get(url)
            .send()
            .await?
            .text()
            .await?;

        let parsed_rate = serde_json::from_str::<Response>(&resp)
            .context("Failed to parse API response")?
            .rates
            .get(&quote)
            .cloned()
            .context("Failed to find quote symbol in parsed API response")?;

        Ok(parsed_rate)
    }
}

impl ExchangeRatesApiProvider {
    pub fn new() -> Self {
        Self(BaseProvider {
            name: String::from("exchangeratesapi"),
            base_url: String::from("https://api.exchangeratesapi.io/latest"),
        })
    }
}
