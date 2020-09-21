use std::collections::{HashMap,HashSet};
use std::fmt;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::rc::Rc;
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
    pub currencies: HashSet<Rc<Currency>>,
    guesses: HashMap<&'static str, Rc<Currency>>,
}

impl Currencies {
    pub fn guess(&self, guess: &str) -> Option<&Currency> {
        (self.guesses.get(guess)).map(|o| &**o)
    }
}

lazy_static! {
    static ref CURRENCIES: Currencies = {
        let mut currencies = HashSet::<Rc<Currency>>::new();
        let mut guesses = HashMap::<&'static str, Rc<Currency>>::new();

        let usd = Rc::new(Currency {
            symbol: Symbol::USD,
            sign: String::from("$"),
            name: String::from("US Dollar"),
            currency_type: CurrencyType::Fiat,
        });
        let eur = Rc::new(Currency {
            symbol: Symbol::EUR,
            sign: String::from("€"),
            name: String::from("Euro"),
            currency_type: CurrencyType::Fiat,
        });
        let gbp = Rc::new(Currency {
            symbol: Symbol::GBP,
            sign: String::from("£"),
            name: String::from("British Pounds"),
            currency_type: CurrencyType::Fiat,
        });
        let eth = Rc::new(Currency {
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
