use std::str::FromStr;

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
