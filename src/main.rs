mod cli;
mod currency;
mod providers;
mod utils;

use std::str::FromStr;

use futures::stream::{self, StreamExt};

use providers::exchangeratesapi::ExchangeRatesApiProvider;
use providers::fixer::FixerProvider;
use providers::provider::{Provider};
use currency::Currency;
use cli::build_cli;
use utils::{Mean};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = build_cli().get_matches();

    let amount = matches.value_of("amount").map(str::parse::<f64>).unwrap().unwrap();
    let input = matches.value_of("input").map(Currency::from_str).unwrap().unwrap();
    let output = matches.values_of("output")
        .unwrap()
        .find_map(|s| Currency::from_str(s).ok())
        .expect("Failed to parse output currency");

    if input == output {
        println!("Input and output currency are the same. Can't convert.");
        return Ok(())
    }

    println!("Amount: {:.2?}, Input: {:?}, Output: {:?}", amount, input, output);

    let mut providers : Vec<Box<dyn Provider>> = vec!(Box::new(ExchangeRatesApiProvider::new()));

    if let Some(access_key) = matches.value_of("access-key-fixer") {
        providers.push(Box::new(FixerProvider::new(access_key.to_string())));
    }

    let rates = stream::iter(&providers)
        .map(|p| p.get_rate(input.symbol.clone(), output.symbol.clone()))
        .buffered(providers.len())
        .map(|r| r.unwrap())
        .collect::<Vec<_>>().await;

    // // Blocking version
    // let mut rates = vec![];
    // for p in providers {
    //     let r = p.get_rate(input.symbol.clone(), output.symbol.clone()).await?;
    //     rates.push(r);
    // }

    println!("Fetched conversion rate: {:?}", rates);
    let avg_rate = (&rates[..]).mean();
    println!("Average rate: {:?}", avg_rate);

    let quote_amount = amount * avg_rate;

    if matches.is_present("precise") {
        println!("Result: {:?}", quote_amount);
    } else {
        println!("Result: {:.2?}", quote_amount);
    }

    Ok(())
}
