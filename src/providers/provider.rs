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
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error>;
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<Decimal, anyhow::Error>;
}
