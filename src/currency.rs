use std::collections::{HashMap,HashSet};
use lazy_static::lazy_static;
use std::str::FromStr;

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Currency {
    pub code: String,
    symbol: String,
    name: String,
    currency_type: CurrencyType,
}

#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub enum CurrencyType {
    Fiat,
    Crypto
}

pub struct Currencies {
    pub currencies: HashSet<Currency>,
    guesses: HashMap<&'static str, Currency>,
}

impl Currencies {
    pub fn guess(&self, guess: &str) -> Option<&Currency> {
        self.guesses.get(guess) //.map(|c| *c)
    }
}

lazy_static! {
    static ref CURRENCIES: Currencies = {
        let mut currencies = HashSet::<Currency>::new();
        let mut guesses = HashMap::<&'static str, Currency>::new();

        let usd = Currency {
            code: String::from("USD"),
            symbol: String::from("$"),
            name: String::from("US Dollar"),
            currency_type: CurrencyType::Fiat,
        };
        let eur = Currency {
            code: String::from("EUR"),
            symbol: String::from("eur"),
            name: String::from("Euro"),
            currency_type: CurrencyType::Fiat,
        };

        // FIXME: too many copies
        guesses.insert("usd", usd.clone());
        guesses.insert("usdollar", usd.clone());
        guesses.insert("$", usd.clone());
        guesses.insert("eur", eur.clone());
        guesses.insert("euro", eur.clone());
        guesses.insert("€", eur.clone());

        currencies.insert(usd);
        currencies.insert(eur);

        Currencies {
            currencies,
            guesses,
        }
    };
}

#[derive(Clone,Debug)]
pub struct ParseCurrencyError(String);

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CURRENCIES.guess(&s.to_lowercase()).ok_or(ParseCurrencyError(
            format!("Could not parse {} into a currency", s)
        )).map(|r| r.clone())
    }
}
