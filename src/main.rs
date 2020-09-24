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
use currency::Currency;
use cli::build_cli;
use utils::Stats;
use join_all_progress::join_all_progress;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = build_cli().get_matches();

    let amount = matches.value_of("amount").map(Decimal::from_str).unwrap().unwrap();
    let input = matches.value_of("input").map(Currency::from_str).unwrap().unwrap();
    let output = matches.values_of("output")
        .unwrap()
        .find_map(|s| Currency::from_str(s).ok())
        .expect("Failed to parse output currency");

    if input == output {
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
        .map(|p| p.get_rate(input.symbol.clone(), output.symbol.clone()))
        .collect::<Vec<_>>();

    let rates = join_all_progress(futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // // Blocking version
    // let mut rates = vec![];
    // for p in providers {
    //     let r = p.get_rate(input.symbol.clone(), output.symbol.clone()).await?;
    //     rates.push(r);
    // }

    let avg_rate = (&rates[..]).mean()
        .context("No data to compute mean")?;
    let quote_amount = amount * avg_rate;

    let result = match matches.is_present("precise") {
        true => quote_amount.normalize(),
        _ => quote_amount.round_dp(2).normalize(),
    };

    match matches.is_present("raw") {
        true => println!("{}", result),
        _ => println!("{} {} ⟶  {} {}", amount, input.symbol.to_string().dimmed(), result, output.symbol.to_string().dimmed()),
    }

    if matches.is_present("stats") {
        let std_deviation = (&rates[..]).std_deviation()
            .map(|e| e.to_string().normal())
            .unwrap_or_else(|| "<cannot compute>".italic());

        vec![
            format!("Successfully fetched {}/{} sources", rates.len(), providers.len()),
            format!("Fetched rates: {:?}, σ: {}", rates, std_deviation)
        ]
        .iter()
        .for_each(|l| println!("{}", l.dimmed()));
    };

    Ok(())
}
