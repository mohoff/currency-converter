use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::ops::Div;
use std::convert::TryInto;


pub trait Stats {
    fn mean(&self) -> Option<Decimal>;
    // FIXME: Decimal does currently not support sqrt() but hopefully in the future
    // (https://github.com/paupino/rust-decimal/issues/169). Working with f64 for now.
    fn std_deviation(&self) -> Option<f64>;
}

impl Stats for &[Decimal] {
    fn mean(&self) -> Option<Decimal> {
        if self.is_empty() {
            return None
        }

        let mean = self.iter().cloned().sum::<Decimal>() / Decimal::new(
            self.len()
                .try_into()
                .expect("Failed to build mean of Decimals") // Should never happen
        , 0);

        Some(mean)
    }

    fn std_deviation(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }

        let mean = self.mean().expect("Non-empty data should have mean"); // Should never happen

        let variance = self.iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<Decimal>()
            .div(Decimal::new(self.len() as i64, 0));

        variance.to_f64().map(|f| f.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use super::Stats;

    #[test]
    fn computes_mean_of_decimals() {
        let decimals = vec![Decimal::new(1, 0), Decimal::new(2, 0)];

        let average = decimals.as_slice().mean();

        assert_eq!(average, Some(Decimal::new(15, 1)), "The mean of [1,2] should be 1.5");
    }

    #[test]
    fn fail_on_empty_data() {
        let decimals = vec![];

        let average = decimals.as_slice().mean();

        assert!(average.is_none());
    }

    // TODO: tests for standard deviation
}
