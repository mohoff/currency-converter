use rust_decimal::Decimal;
use std::convert::TryInto;

pub trait Mean {
    fn mean(&self) -> Decimal;
}

impl Mean for &[Decimal] {
    fn mean(&self) -> Decimal {
        self.iter().cloned().sum::<Decimal>() / Decimal::new(
            self.len()
                .try_into()
                .expect("Failed to build mean of Decimals") // Should never happen
        , 0)
    }
}
