use std::str::FromStr;
use std::ops::Mul;

#[derive(Debug)]
pub struct Amount { // TODO: use big int
    cents: usize
}

#[derive(Clone,Debug)]
pub struct ParseAmountError(String);

impl FromStr for Amount {
    type Err = ParseAmountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dot_index = None;
        let mut digits_after_dot = 0;
        let mut digits = String::from("");
        for (i,c) in s.chars().enumerate() {
            if c == '.' {
                if dot_index.is_some() {
                    return Err(
                        ParseAmountError(format!("Too many decimal dots in amount {}", s))
                    );
                }
                dot_index = Some(i);
                continue;
            }

            if !c.is_numeric() {
                return Err(
                        ParseAmountError(format!("Invalid amount {}", s))
                    );
            }

            if dot_index.is_some() {
                if digits_after_dot == 2 {
                    return Err(
                        ParseAmountError(format!("Too many decimals in amount {}", s))
                    );
                }
                digits_after_dot += 1;
            }
            digits.push(c);
        };

        let parsed = digits.parse::<usize>().unwrap();
        let parsed = match digits_after_dot {
            0 => parsed * 100,
            1 => parsed * 10,
            _ => parsed, // FIXME: enum instead of digits-after-dot?
        };

        Ok(Amount { cents: parsed })
    }
}

impl Mul for Amount {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Amount {
            cents: self.cents * other.cents
        }
    }
}

// FIXME: Not accurate
impl Mul<f64> for Amount {
    type Output = Amount;
    fn mul(self, float: f64) -> Amount {
        Amount {
            cents: self.cents * ((float * 100.0) as usize) / 100
        }
    }
}

impl Mul<Amount> for f64 {
    type Output = Amount;
    fn mul(self, a: Amount) -> Amount {
        Amount {
            cents: a.cents * ((self * 100.0) as usize) / 100
        }
    }
}
