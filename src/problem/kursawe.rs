use super::Problem;

pub struct Kursawe {
    ranges: [(f64, f64); 3],
}

impl Default for Kursawe {
    fn default() -> Self {
        Self {
            ranges: [(-5.0, 5.0); 3],
        }
    }
}

impl Problem for Kursawe {
    fn num_variables(&self) -> usize {
        3
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), 3);

        // f1 = sum of two exponential terms
        let f1 = (-10.0 * (-0.2 * (x[0].powi(2) + x[1].powi(2)).sqrt()).exp())
            + (-10.0 * (-0.2 * (x[1].powi(2) + x[2].powi(2)).sqrt()).exp());

        // f2 = sum over all variables
        let f2 = x
            .iter()
            .map(|xi| xi.abs().powf(0.8) + 5.0 * (xi.powi(3)).sin())
            .sum::<f64>();

        vec![f1, f2]
    }
}
