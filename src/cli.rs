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
                .about("input currency, e.g. USD, Euro")
                .index(2),
        )
        .arg(
            Arg::with_name("output")
                .about("output currency, e.g. USD, Euro")
                .index(3)
                .multiple(true)
        )
        .arg(
            Arg::new("access_key_fixer")
                .about("Enables the Fixer.io API")
                .takes_value(true)
                .long("--access-key-fixer"),
        )
}
