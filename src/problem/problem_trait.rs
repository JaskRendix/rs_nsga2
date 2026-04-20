pub trait Problem: Send + Sync {
    fn num_variables(&self) -> usize;
    fn num_objectives(&self) -> usize;

    /// Variable bounds for each decision variable.
    fn variable_ranges(&self) -> &[(f64, f64)];

    /// Compute objective values for a given feature vector.
    fn calculate_objectives(&self, features: &[f64]) -> Vec<f64>;

    /// Constraint violations (<= 0 means satisfied).
    fn constraint_violations(&self, _features: &[f64]) -> Vec<f64> {
        vec![]
    }

    /// Feasibility check based on constraint violations.
    fn is_feasible(&self, features: &[f64]) -> bool {
        self.constraint_violations(features)
            .iter()
            .all(|&v| v <= 0.0)
    }
}
