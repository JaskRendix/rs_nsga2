use super::Problem;

//
// ZDT1
//
pub struct ZDT1 {
    ranges: Vec<(f64, f64)>,
}

impl ZDT1 {
    pub fn new(num_variables: usize) -> Self {
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
        }
    }
}

impl Problem for ZDT1 {
    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let f1 = x[0];
        let n = x.len() as f64;
        let g = 1.0 + 9.0 * x[1..].iter().sum::<f64>() / (n - 1.0);
        let h = 1.0 - (f1 / g).sqrt();
        vec![f1, g * h]
    }
}

//
// ZDT2
//
pub struct ZDT2 {
    ranges: Vec<(f64, f64)>,
}

impl ZDT2 {
    pub fn new(num_variables: usize) -> Self {
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
        }
    }
}

impl Problem for ZDT2 {
    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let f1 = x[0];
        let n = x.len() as f64;
        let g = 1.0 + 9.0 * x[1..].iter().sum::<f64>() / (n - 1.0);
        let h = 1.0 - (f1 / g).powi(2);
        vec![f1, g * h]
    }
}

//
// ZDT3
//
pub struct ZDT3 {
    ranges: Vec<(f64, f64)>,
}

impl ZDT3 {
    pub fn new(num_variables: usize) -> Self {
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
        }
    }
}

impl Problem for ZDT3 {
    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let f1 = x[0];
        let n = x.len() as f64;
        let g = 1.0 + 9.0 * x[1..].iter().sum::<f64>() / (n - 1.0);
        let h = 1.0 - (f1 / g).sqrt() - (f1 / g) * (10.0 * std::f64::consts::PI * f1).sin();
        vec![f1, g * h]
    }
}
