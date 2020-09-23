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

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use super::Mean;

    #[test]
    fn computes_mean_of_decimals() {
        let decimals = vec![Decimal::new(1, 0), Decimal::new(2, 0)];

        let average = decimals.as_slice().mean();

        assert_eq!(average, Decimal::new(15, 1), "The mean of [1,2] should be 1.5");
    }
}
