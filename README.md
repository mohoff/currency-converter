# `currency-converter`

A command line tool written in Rust to convert between currencies.

## Usage

```
$ currency-converter 1 eur in usd
1 EUR ⟶ 1.17 USD
```

## Configuration

- By default, one API is used to fetch the conversion rate. Additional providers can be enabled by passing their API access keys. The tool averages the results.
- By default, 2 decimal places are printed. By using `--precise`, more decimals might be shown, depending on the conversion rates fetched from the rate providers.
- Stats can be shown with `--stats`.

All configuration options are shown in the `--help` output:

```
Converts an amount of a currency to another currency

USAGE:
    currency-converter [FLAGS] [OPTIONS] [ARGS]

ARGS:
    <amount>           how much of the input currency
    <currencies>...    currency conversion, e.g. 'turkish lira in usd'

FLAGS:
    -h, --help       Prints help information
    -p, --precise    Show sub-cent decimals
    -r, --raw        Only print output currency value
    -s, --stats      Show conversion statistics
    -V, --version    Prints version information

OPTIONS:
        --access-key-coinmarketcap <access-key-coinmarketcap>    Enables the CoinMarketCap API
        --access-key-fixer <access-key-fixer>                    Enables the Fixer.io API
```
