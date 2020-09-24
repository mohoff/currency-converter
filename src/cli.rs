use clap::{App,Arg};

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
            Arg::with_name("input")
                .about("Input currency (base), e.g. USD, Euro")
                .index(2),
        )
        .arg(
            Arg::with_name("output")
                .about("Output currency (quote), e.g. USD, Euro")
                .index(3)
                .multiple(true)
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
