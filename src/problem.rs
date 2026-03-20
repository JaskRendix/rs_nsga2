pub trait Problem: Send + Sync {
    fn num_variables(&self) -> usize;
    fn variable_ranges(&self) -> Vec<(f64, f64)>;
    fn calculate_objectives(&self, features: &[f64]) -> Vec<f64>;
}

pub struct Schaffer;

impl Problem for Schaffer {
    fn num_variables(&self) -> usize {
        1
    }

    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(-55.0, 55.0)]
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0].powi(2), (x[0] - 2.0).powi(2)]
    }
}
