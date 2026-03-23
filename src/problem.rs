pub trait Problem: Send + Sync {
    fn num_variables(&self) -> usize;
    fn variable_ranges(&self) -> Vec<(f64, f64)>;
    fn calculate_objectives(&self, features: &[f64]) -> Vec<f64>;
    fn num_objectives(&self) -> usize;

    fn constraint_violations(&self, _features: &[f64]) -> Vec<f64> {
        vec![]
    }
    fn is_feasible(&self, features: &[f64]) -> bool {
        self.constraint_violations(features)
            .iter()
            .all(|&v| v <= 0.0)
    }
}

pub struct Schaffer;

impl Problem for Schaffer {
    fn num_variables(&self) -> usize {
        1
    }
    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(-55.0, 55.0)]
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0].powi(2), (x[0] - 2.0).powi(2)]
    }
}
