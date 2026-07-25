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
    fn name(&self) -> &'static str {
        "ZDT1"
    }

    fn description(&self) -> &'static str {
        "Zitzler–Deb–Thiele benchmark problem 1"
    }

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

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for xi in features {
            *xi = xi.clamp(0.0, 1.0);
        }
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
    fn name(&self) -> &'static str {
        "ZDT2"
    }

    fn description(&self) -> &'static str {
        "Zitzler–Deb–Thiele benchmark problem 2"
    }

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

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for xi in features {
            *xi = xi.clamp(0.0, 1.0);
        }
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
    fn name(&self) -> &'static str {
        "ZDT3"
    }

    fn description(&self) -> &'static str {
        "Zitzler–Deb–Thiele benchmark problem 3 (disconnected Pareto front)"
    }

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

        let ratio = f1 / g;
        let h = 1.0 - ratio.sqrt() - ratio * (10.0 * std::f64::consts::PI * f1).sin();

        vec![f1, g * h]
    }

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for xi in features {
            *xi = xi.clamp(0.0, 1.0);
        }
    }
}
