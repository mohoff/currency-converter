use anyhow::*;
use clap::{App, Arg};

use std::convert::TryInto;
use std::str::FromStr;

use crate::currency::{Currency, Symbol, SymbolPair};

const SEPERATORS: &[&str] = &["in", "as", "into", "to", ">", "->", "-->"];

pub fn build_cli() -> App<'static> {
    App::new("Currency Converter")
        .about("Converts an amount of a currency to another currency")
        .version("0.0.1")
        .author("Moritz H.")
        .arg(
            Arg::with_name("amount")
                .about("how much of the input currency")
                .index(1),
        )
        .arg(
            Arg::with_name("currencies")
                .about("currency conversion, e.g. 'turkish lira in usd'")
                .index(2)
                .multiple(true),
        )
        .arg(
            Arg::with_name("precise")
                .about("Show sub-cent decimals")
                .short('p')
                .long("--precise"),
        )
        .arg(
            Arg::with_name("raw")
                .about("Only print output currency value")
                .short('r')
                .long("--raw"),
        )
        .arg(
            Arg::with_name("stats")
                .about("Show conversion statistics")
                .short('s')
                .long("--stats"),
        )
        .arg(
            Arg::with_name("access-key-fixer")
                .about("Enables the Fixer.io API")
                .takes_value(true)
                .long("--access-key-fixer"),
        )
        .arg(
            Arg::with_name("access-key-coinmarketcap")
                .about("Enables the CoinMarketCap API")
                .takes_value(true)
                .long("--access-key-coinmarketcap"),
        )
}

type Words<'a> = Vec<&'a str>;
fn partition_words_by(seperators: &'static [&'static str]) -> Box<dyn Fn(Words) -> (Words, Words)> {
    Box::new(move |input: Words| {
        let (_, pre, post) = input.iter().fold(
            (false, vec![], vec![]),
            |(mut found_seperator, mut pre, mut post), x| {
                if found_seperator {
                    post.push(*x);
                } else if seperators.contains(&x) {
                    found_seperator = true;
                } else {
                    pre.push(*x);
                }

                (found_seperator, pre, post)
            },
        );

        (pre, post)
    })
}

pub fn parse_currencies(words: Vec<&str>) -> Result<SymbolPair, anyhow::Error> {
    let (pre, post) = partition_words_by(SEPERATORS)(words);

    let [base, quote]: [Symbol; 2] = [pre, post]
        .iter()
        .map(|l| l.join(" "))
        .map(|s| Currency::from_str(&s))
        .map(|r| r.map(|c| c.symbol))
        .collect::<Result<Vec<Symbol>, _>>()?
        .as_slice()
        .try_into()
        .unwrap();

    Ok(SymbolPair { base, quote })
}

#[cfg(test)]
mod tests {
    use super::parse_currencies;
    use super::partition_words_by;
    use crate::currency::{Symbol, SymbolPair};

    #[test]
    fn basic_partition() {
        let (pre, post) = partition_words_by(&["in"])(vec!["eur", "in", "usd"]);

        assert_eq!(pre, vec!["eur"]);
        assert_eq!(post, vec!["usd"]);
    }

    #[test]
    fn compound_currency_partition() {
        let (pre, post) = partition_words_by(&["in"])(vec!["turkish", "lira", "in", "usd"]);

        assert_eq!(pre, vec!["turkish", "lira"]);
        assert_eq!(post, vec!["usd"]);
    }

    #[test]
    fn compound_currency_with_junk() {
        let (pre, post) =
            partition_words_by(&["in", "to"])(vec!["turkish", "lira", "to", "foo", "usd", "bar"]);

        assert_eq!(pre, vec!["turkish", "lira"]);
        assert_eq!(post, vec!["foo", "usd", "bar"]);
    }

    #[test]
    fn no_seperator_partition() {
        let (pre, post) = partition_words_by(&["in"])(vec!["eur", "usd"]);

        assert_eq!(pre, vec!["eur", "usd"]);
        assert_eq!(post, Vec::new() as Vec<String>);
    }

    #[test]
    fn no_pre_partition() {
        let (pre, post) = partition_words_by(&["in"])(vec!["in", "usd"]);

        assert_eq!(pre, Vec::new() as Vec<String>);
        assert_eq!(post, vec!["usd"]);
    }

    #[test]
    fn no_post_partition() {
        let (pre, post) = partition_words_by(&["in"])(vec!["usd", "in"]);

        assert_eq!(pre, vec!["usd"]);
        assert_eq!(post, Vec::new() as Vec<String>);
    }

    // parse_currencies tests
    #[test]
    fn basic_currency_parsing() {
        let expected_pair = SymbolPair {
            base: Symbol::USD,
            quote: Symbol::EUR,
        };
        let option = parse_currencies(vec!["usd", "in", "eur"]).ok();

        assert_eq!(option, Some(expected_pair));
    }

    #[test]
    fn compound_currency_parsing() {
        let expected_pair = SymbolPair {
            base: Symbol::TL,
            quote: Symbol::EUR,
        };
        let option = parse_currencies(vec!["turkish", "lira", "in", "eur"]).ok();

        assert_eq!(option, Some(expected_pair));
    }

    #[test]
    fn compound_currency_parsing2() {
        let expected_pair = SymbolPair {
            base: Symbol::TL,
            quote: Symbol::TL,
        };
        let option = parse_currencies(vec!["turkish", "lira", "in", "turkish", "lira"]).ok();

        assert_eq!(option, Some(expected_pair));
    }

    #[test]
    fn no_base_currency_parsing() {
        let result = parse_currencies(vec!["invalid", "in", "turkish", "lira"]);

        assert!(result.is_err());
    }

    #[test]
    fn no_quote_currency_parsing() {
        let result = parse_currencies(vec!["usd", "in"]);

        assert!(result.is_err());
    }

    #[test]
    fn no_seperator_currency_parsing() {
        let result = parse_currencies(vec!["usd", "foo", "eur"]);

        assert!(result.is_err());
    }
}
