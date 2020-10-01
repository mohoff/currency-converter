mod cli;
mod currency;
mod providers;
mod utils;
mod join_all_progress;

use std::str::FromStr;

use anyhow::*;
use colored::*;
use rust_decimal::Decimal;

use providers::coinmarketcap::CoinMarketCapProvider;
use providers::exchangeratesapi::ExchangeRatesApiProvider;
use providers::fixer::FixerProvider;
use providers::provider::{Provider};
use cli::{build_cli, parse_currencies};
use utils::Stats;
use join_all_progress::join_all_progress;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = build_cli().get_matches();

    let amount = matches.value_of("amount").map(Decimal::from_str).unwrap().unwrap();
    let symbols = parse_currencies(
        matches.values_of("currencies")
            .context("No currency conversion found in input")?
            .collect::<Vec<&str>>()
        ).context("Failed to parse currency string")?;

    if symbols.base == symbols.quote {
        println!("Input and output currency are the same. Can't convert.");
        return Ok(())
    }

    let mut providers : Vec<Box<dyn Provider>> = vec!(Box::new(ExchangeRatesApiProvider::new()));

    if let Some(access_key) = matches.value_of("access-key-fixer") {
        providers.push(Box::new(FixerProvider::new(access_key.to_string())));
    }
    if let Some(access_key) = matches.value_of("access-key-coinmarketcap") {
        providers.push(Box::new(CoinMarketCapProvider::new(access_key.to_string())));
    }

    let futures = providers.iter()
        .map(|p| p.get_rate(symbols.base, symbols.quote))
        .collect::<Vec<_>>();

    // NOTE: must preserve order so we can associate future output with provider name
    let rate_results : Vec<Option<Decimal>> = join_all_progress(futures)
        .await
        .into_iter()
        .map(|r| r.ok())
        .collect::<Vec<_>>();
    let rates : Vec<Decimal> = rate_results.iter()
        .cloned()
        .filter_map(|o| o)
        .collect::<Vec<_>>();

    let avg_rate = (&rates[..]).mean()
        .context("No data to compute mean")?;
    let quote_amount = amount * avg_rate;

    let result = match matches.is_present("precise") {
        true => quote_amount.normalize(),
        _ => quote_amount.round_dp(2).normalize(),
    };

    match matches.is_present("raw") {
        true => println!("{}", result),
        _ => println!("{} {} ⟶  {} {}", amount, symbols.base.to_string().dimmed(), result, symbols.quote.to_string().dimmed()),
    }

    if matches.is_present("stats") {
        let std_deviation = (&rates[..]).std_deviation()
            .map(|e| e.to_string().normal())
            .unwrap_or_else(|| "<cannot compute>".italic());
        let provider_statuses = providers.iter()
            .zip(rate_results)
            .map(|(p,r)| if r.is_some() { p.get_name().green() } else { p.get_name().dimmed() })
            .fold(String::from(""), |mut acc, x| { // Joins Vec<ColoredString>
                acc.push_str(&x.to_string());
                acc.push(' ');
                acc
            });

        vec![
            format!("Successfully fetched {}/{} sources: {}", rates.len(), providers.len(), provider_statuses),
            format!("Fetched rates: {:?}, σ: {}", rates, std_deviation)
        ]
        .iter()
        .for_each(|l| println!("{}", l));
    };

    Ok(())
}
