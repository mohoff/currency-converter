use anyhow::{Context};
use serde::{Deserialize,Serialize};
use reqwest::Url;
use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::currency::Symbol;
use crate::providers::provider::{BaseProvider,Provider};
use std::collections::HashMap;

pub struct FixerProvider {
    provider: BaseProvider,
    access_key: String,
}

#[derive(Serialize,Deserialize)]
struct Response {
    rates: HashMap<Symbol, Decimal>,
    base: Symbol,
    date: String,
    timestamp: usize,
    success: bool,
}

#[async_trait]
impl Provider for FixerProvider {
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error> {
        Url::parse_with_params(
            &self.provider.base_url,
            &[("access_key", self.access_key.clone()), ("base", base.to_string()), ("symbols", quote.to_string())]
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

impl FixerProvider {
    pub fn new(access_key: String) -> Self {
        Self {
            provider: BaseProvider {
                name: String::from("fixer.io"),
                base_url: String::from("http://data.fixer.io/api/latest"), // FIXME: favor provider that supports https in free plan
            },
            access_key,
        }
    }
}
