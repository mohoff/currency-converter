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
    fn get_name(&self) -> String {
        self.0.name.clone()
    }
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

        let parsed_rate = ExchangeRatesApiProvider::parse_rate_from_response(&self, &quote, &resp)?;

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

impl ExchangeRatesApiProvider {
    pub fn new() -> Self {
        Self(BaseProvider {
            name: String::from("exchangeratesapi.io"),
            base_url: String::from("https://api.exchangeratesapi.io/latest"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ExchangeRatesApiProvider;
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
                "date": "2020-09-23"
            }}
        "#, expected_rate);
        let provider = ExchangeRatesApiProvider::new();

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
                    "GBP": "0.111"
                }},
                "base": "EUR",
                "date": "2020-09-23"
            }}
        "#;
        let provider = ExchangeRatesApiProvider::new();

        // Note: Converting to Option to get rid of E in Result<T,E>. Otherwise,
        // the assertion fails as anyhow::Error does not implement Eq.
        let rate = provider.parse_rate_from_response(&quote, &response);

        assert!(rate.is_err(), "Parsing invalid response should fail");
    }
}
