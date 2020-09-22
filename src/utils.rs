
pub trait Mean {
    fn mean(&self) -> f64;
}

impl Mean for &[f64] {
    fn mean(&self) -> f64 {
        self.iter().sum::<f64>() / (self.len() as f64)
    }
}
