use reqwest::Url;
use async_trait::async_trait;

use crate::currency::Symbol;

pub struct BaseProvider {
    #[allow(dead_code)]
    pub name: String,
    pub base_url: String,
}

#[async_trait]
pub(crate) trait Provider {
    fn new() -> Self;
    fn build_url(&self, base: &Symbol, quote: &Symbol) -> Result<Url, anyhow::Error>;
    async fn get_rate(&self, base: Symbol, quote: Symbol) -> Result<f64, anyhow::Error>;
}