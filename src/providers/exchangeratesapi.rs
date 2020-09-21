use anyhow::{Context};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json;
use async_trait::async_trait;

use std::collections::HashMap;

use crate::currency::Symbol;

pub struct Provider {
    name: String,
    base_url: String,
}

#[derive(Serialize,Deserialize)]
struct Response {
    rates: HashMap<Symbol, f64>,
    base: Symbol,
    date: String
}

#[async_trait]
pub trait ProviderT {
    fn new(name: String, base_url: String) -> Provider;
    fn build_url(&self, base: Symbol, quote: Symbol) -> Result<Url, anyhow::Error>;
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<f64, anyhow::Error>;
}

#[async_trait]
impl ProviderT for Provider {
    fn new(name: String, base_url: String) -> Self {
        Provider {
            name,
            base_url
        }
    }
    fn build_url(&self, base: Symbol, quote: Symbol) -> Result<Url, anyhow::Error> {
        //"https://api.exchangeratesapi.io/latest?base={}&symbols={}
        Url::parse_with_params(
            &self.base_url,
            &[("base", base.to_string()), ("symbols", quote.to_string())]
        ).context("Failed to build URL")
    }
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<f64, anyhow::Error> {
        let url = self.build_url(base, quote)?;
        let client = reqwest::Client::new();
        let resp = client.get(url)
            .send()
            .await?
            .text()
            .await?;

        let parsed_rate = serde_json::from_str::<Response>(&resp)
            .context("Failed to parse API response")?
            .rates
            .get(&Symbol::USD)
            .cloned()
            .context("Failed to find quote symbol in parsed API response")?;

        Ok(parsed_rate)
    }
}
