# `cuco` - convert currencies on the command line


## Development

Example usage

```
cargo run -- 10234 eur in usd [--access-key-fixer=ACCESS_KEY] [--access-key-coinmarketcap=ACCESS_KEY] [--stats] [--raw] [--precise]
```

### TODO

- when getting rates through providers, keep track of provider name per fetched rate.
   * show in `--stats` output which APIs were fetched successfully and which not
