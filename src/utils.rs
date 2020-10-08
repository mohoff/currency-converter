use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub trait Stats {
    fn mean(self) -> Option<Decimal>;

    fn variance(self) -> Option<Decimal>;

    // FIXME: Decimal does currently not support sqrt() but hopefully in the future
    // (https://github.com/paupino/rust-decimal/issues/169). Working with f64 for now.
    fn std_deviation(self) -> Option<f64>;
}

impl<'a, I: IntoIterator<Item=&'a Decimal>> Stats for I {
    fn mean(self) -> Option<Decimal> {
        let mut count = 0;
        let mut total = Decimal::new(0, 0);

        for x in self.into_iter() {
            count += 1;
            total += x;
        }

        if count == 0 {
            None
        } else {
            Some(total / Decimal::new(count, 0))
        }
    }

    fn variance(self) -> Option<Decimal> {
        let mut count = 0;
        let mut total = Decimal::new(0, 0);
        let mut totalsq = Decimal::new(0, 0);

        for x in self.into_iter() {
            count += 1;
            total += x;
            totalsq += x * x;
        }

        if count == 0 { return None }

        let count = Decimal::new(count, 0);

        let mean = total / count;
        let meansq = totalsq / count;

        let variance = meansq - (mean * mean);
        Some(variance.abs())
    }

    fn std_deviation(self) -> Option<f64> {
        Some(self.variance()?.to_f64()?.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use rust_decimal::Decimal;

    #[test]
    fn computes_mean_of_decimals() {
        let decimals = vec![Decimal::new(1, 0), Decimal::new(2, 0)];

        let average = decimals.as_slice().mean();

        assert_eq!(
            average,
            Some(Decimal::new(15, 1)),
            "The mean of [1,2] should be 1.5"
        );
    }

    #[test]
    fn mean_fails_on_empty_data() {
        let decimals = vec![];

        let average = decimals.as_slice().mean();

        assert!(average.is_none());
    }

    #[test]
    fn computes_variance_of_decimals() {
        let decimals = vec![Decimal::new(1, 0), Decimal::new(2, 0)];

        let variance = decimals.variance();

        assert_eq!(
            variance,
            Some(Decimal::new(25, 2)),
            "The variance of [1,2] should be 0.25"
        );
    }

    #[test]
    fn computes_variance_of_arrays() {
        let decimals: [Decimal; 2] = [Decimal::new(1, 0), Decimal::new(2, 0)];

        let variance = decimals.variance();

        assert_eq!(
            variance,
            Some(Decimal::new(25, 2)),
            "The variance of [1,2] should be 0.25"
        );
    }

    #[test]
    fn computes_std_deviation_of_decimals() {
        let decimals = vec![Decimal::new(1, 0), Decimal::new(2, 0)];

        let std_deviation = decimals.as_slice().std_deviation();

        assert_eq!(
            std_deviation,
            Some(0.5),
            "The standard deviation of [1,2] should be 0.5"
        );
    }

    #[test]
    fn std_deviation_fails_on_empty_data() {
        let decimals = vec![];

        let std_deviation = decimals.as_slice().std_deviation();

        assert!(std_deviation.is_none());
    }
}
