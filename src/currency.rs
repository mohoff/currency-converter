use std::collections::{HashMap,HashSet};
use std::fmt;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::sync::Arc;
use serde::{Deserialize,Serialize};

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Currency {
    pub symbol: Symbol,
    sign: String,
    name: String,
    currency_type: CurrencyType,
}

#[derive(Serialize,Deserialize,Clone,Eq,PartialEq,Hash,Debug)]
pub enum Symbol {
    EUR, USD, GBP, ETH, BTC
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub enum CurrencyType {
    Fiat,
    Crypto
}

pub struct Currencies {
    pub currencies: HashSet<Arc<Currency>>,
    guesses: HashMap<&'static str, Arc<Currency>>,
}

impl Currencies {
    pub fn guess(&self, guess: &str) -> Option<&Currency> {
        (self.guesses.get(guess)).map(|o| &**o)
    }
}

lazy_static! {
    static ref CURRENCIES: Currencies = {
        let mut currencies = HashSet::<Arc<Currency>>::new();
        let mut guesses = HashMap::<&'static str, Arc<Currency>>::new();

        let usd = Arc::new(Currency {
            symbol: Symbol::USD,
            sign: String::from("$"),
            name: String::from("US Dollar"),
            currency_type: CurrencyType::Fiat,
        });
        let eur = Arc::new(Currency {
            symbol: Symbol::EUR,
            sign: String::from("€"),
            name: String::from("Euro"),
            currency_type: CurrencyType::Fiat,
        });
        let gbp = Arc::new(Currency {
            symbol: Symbol::GBP,
            sign: String::from("£"),
            name: String::from("British Pounds"),
            currency_type: CurrencyType::Fiat,
        });
        let eth = Arc::new(Currency {
            symbol: Symbol::ETH,
            sign: String::from("Ξ"),
            name: String::from("Ether"),
            currency_type: CurrencyType::Crypto,
        });

        guesses.insert("usd", usd.clone());
        guesses.insert("usdollar", usd.clone());
        guesses.insert("$", usd.clone());
        guesses.insert("eur", eur.clone());
        guesses.insert("euro", eur.clone());
        guesses.insert("€", eur.clone());
        guesses.insert("gbp", eur.clone());
        guesses.insert("pound", gbp.clone());
        guesses.insert("pounds", gbp.clone());
        guesses.insert("£", gbp.clone());
        guesses.insert("eth", eth.clone());
        guesses.insert("eths", eth.clone());
        guesses.insert("ether", eth.clone());
        guesses.insert("ethers", eth.clone());
        guesses.insert("Ξ", eth.clone());
        guesses.insert("ethereum", eth.clone());

        currencies.insert(usd);
        currencies.insert(eur);
        currencies.insert(gbp);
        currencies.insert(eth);

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
        CURRENCIES
            .guess(&s.to_lowercase())
            .cloned()
            .ok_or_else(||
                ParseCurrencyError(format!("Could not parse {} into a currency", s))
            )
    }
}


#[cfg(test)]
mod tests {
    use super::Currency;
    use std::str::FromStr;

    #[test]
    fn parses_currency_from_str() {
        let input = "usd";

        let currency = Currency::from_str(input);

        assert!(currency.is_ok(), "Currency {} should be parsed correctly", input);
    }

    #[test]
    fn fails_parsing_invalid_currency_from_str() {
        let input = "usdd";

        let invalid_currency = Currency::from_str(input);

        assert!(invalid_currency.is_err(), "Invalid Currency {} should fail to parse", input);
    }
}
