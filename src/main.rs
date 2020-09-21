mod amount;
mod currency;
mod providers;

use std::str::FromStr;

use clap::{App,Arg};
use amount::Amount;
use providers::exchangeratesapi::{Provider,ProviderT};
use currency::Currency;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Currency Converter")
        .about("Converts an amount of a currency to another currency")
        .version("0.0.1")
        .author("Moritz H.")
        .arg(
            Arg::with_name("amount")
                .about("how much of the input currency")
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .about("input currency, e.g. USD, Euro")
                .index(2),
        )
        .arg(
            Arg::with_name("output")
                .about("output currency, e.g. USD, Euro")
                .index(3)
                .multiple(true)
        )
        .get_matches();

    let amount = matches.value_of("amount").map(Amount::from_str).unwrap().unwrap();
    let input = matches.value_of("input").map(Currency::from_str).unwrap().unwrap();
    let output = matches.values_of("output")
        .unwrap()
        .find_map(|s| Currency::from_str(s).ok())
        .expect("Failed to parse output currency");

    if input == output {
        println!("Input and output currency are the same. Can't convert.");
        return Ok(())
    }

    println!("Amount: {:?}, Input: {:?}, Output: {:?}", amount, input, output);

    let provider = Provider::new(
        String::from("exchangeratesapi"),
        String::from("https://api.exchangeratesapi.io/latest")
    );
    let conversion_rate = provider.get_rate(input.symbol, output.symbol).await?;
    println!("Fetched conversion rate: {}", conversion_rate);

    let quote_amount = amount * conversion_rate;

    println!("Result: {:?}", quote_amount);
    Ok(())

}
