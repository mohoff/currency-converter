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
    fn get_name(&self) -> String {
        self.provider.name.clone()
    }
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

        let parsed_rate = FixerProvider::parse_rate_from_response(&self, &quote, &resp)?;

        Ok(parsed_rate)
    }
    fn parse_rate_from_response(&self, quote: &Symbol, response: &str) -> Result<Decimal, anyhow::Error> {
        serde_json::from_str::<Response>(response)
            .context("Failed to parse API response")?
            .rates
            .get(quote)
            .cloned()
            .context("Failed to find quote symbol in parsed API response")
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

#[cfg(test)]
mod tests {
    use super::FixerProvider;
    use crate::providers::provider::Provider;
    use crate::currency::Symbol;

    use rust_decimal::Decimal;

    #[test]
    fn parses_response_correctly() {
        let quote = Symbol::USD;
        let expected_rate = Decimal::new(111, 4);
        let response = format!(r#"
            {{
                "rates": {{
                    "USD": "{}"
                }},
                "base": "EUR",
                "date": "2020-09-23",
                "timestamp": 1600865231,
                "success": true
            }}
        "#, expected_rate);
        let provider = FixerProvider::new(String::from("some-access-key"));

        // Note: Converting to Option to get rid of E in Result<T,E>. Otherwise,
        // the assertion fails as anyhow::Error does not implement Eq.
        let rate = provider.parse_rate_from_response(&quote, &response).ok();

        assert_eq!(rate, Some(expected_rate), "Parsed rate should match");
    }

    #[test]
    fn fails_parsing_invalid_response() {
        let quote = Symbol::USD;
        let response = r#"
            {{
                "rates": {{
                    "EUR": "0.111"
                }},
                "base": "EUR",
                "date": "2020-09-23",
                "timestamp": 1600865231,
                "success": true
            }}
        "#;
        let provider = FixerProvider::new(String::from("some-access-key"));

        // Note: Converting to Option to get rid of E in Result<T,E>. Otherwise,
        // the assertion fails as anyhow::Error does not implement Eq.
        let rate = provider.parse_rate_from_response(&quote, &response);

        assert!(rate.is_err(), "Parsing invalid response should fail");
    }
}
