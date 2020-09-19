use clap::Clap;


#[derive(Clap)]
#[clap(version = "0.0.1", author = "Moritz H.")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    currency_from: String,
    currency_to: String,
    amount: usize,
    to: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}
