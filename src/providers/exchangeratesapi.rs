

pub struct Provider {
    name: String,
    url: String,
}

impl Provider {
    pub fn new(name: String, url: String) -> Self {
        Provider {
            name,
            url
        }
    }
    pub fn build_url(&self, base: &str, quote: &str) -> String {
        format!("https://api.exchangeratesapi.io/latest?base={}&symbols={}", base, quote)
    }
}
