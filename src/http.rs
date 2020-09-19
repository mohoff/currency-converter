

use std::collections::HashMap;

use crate::providers::exchangeratesapi::Provider;

pub async fn get_quote(base: &str, quote: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = Provider::new(
        String::from("exchangeratesapi"),
        String::from("https://api.exchangeratesapi.io/latest?base={}&symbols={}"))
        .build_url(base, quote);
    let resp = reqwest::get(&url).await?.text()
    .await?;

    println!("{:#?}", resp);

    Ok(())
}
