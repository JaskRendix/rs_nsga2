use super::Problem;

pub struct Schaffer {
    ranges: [(f64, f64); 1],
}

impl Default for Schaffer {
    fn default() -> Self {
        Self {
            ranges: [(-55.0, 55.0)],
        }
    }
}

impl Problem for Schaffer {
    fn num_variables(&self) -> usize {
        1
    }
    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let f1 = x[0] * x[0];
        let f2 = (x[0] - 2.0) * (x[0] - 2.0);
        vec![f1, f2]
    }
}
