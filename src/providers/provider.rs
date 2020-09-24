use reqwest::Url;
use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::currency::Symbol;

pub struct BaseProvider {
    #[allow(dead_code)]
    pub name: String,
    pub base_url: String,
}

#[async_trait]
pub(crate) trait Provider {
    fn get_name() -> String;
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error>;
    fn parse_rate_from_response(&self, quote: &Symbol, response: &str) -> Result<Decimal, anyhow::Error>;
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<Decimal, anyhow::Error>;
}
