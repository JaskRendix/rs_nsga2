use super::Problem;

//
// Shared g(x) function for DTLZ1 and DTLZ3
//
fn dtlz_g(x: &[f64], k: usize) -> f64 {
    let tail = &x[x.len() - k..];
    tail.iter()
        .map(|xi| (xi - 0.5).powi(2) - (20.0 * std::f64::consts::PI * (xi - 0.5)).cos())
        .sum::<f64>()
        + 100.0 * tail.len() as f64
}

//
// DTLZ1
//
pub struct DTLZ1 {
    ranges: Vec<(f64, f64)>,
    m: usize,
    k: usize,
}

impl DTLZ1 {
    pub fn new(num_objectives: usize, num_variables: usize) -> Self {
        let k = num_variables - num_objectives + 1;
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
            m: num_objectives,
            k,
        }
    }
}

impl Problem for DTLZ1 {
    fn name(&self) -> &'static str {
        "DTLZ1"
    }

    fn description(&self) -> &'static str {
        "Deb–Thiele–Laumanns–Zitzler benchmark problem 1"
    }

    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        self.m
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let g = dtlz_g(x, self.k);
        let mut f = vec![0.5 * (1.0 + g); self.m];

        for i in 0..self.m {
            for &xj in x.iter().take(self.m - i - 1) {
                f[i] *= xj;
            }
            if i > 0 {
                f[i] *= 1.0 - x[self.m - i - 1];
            }
        }

        f
    }

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for x in features {
            *x = x.clamp(0.0, 1.0);
        }
    }
}

//
// DTLZ2
//
pub struct DTLZ2 {
    ranges: Vec<(f64, f64)>,
    m: usize,
    k: usize,
}

impl DTLZ2 {
    pub fn new(num_objectives: usize, num_variables: usize) -> Self {
        let k = num_variables - num_objectives + 1;
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
            m: num_objectives,
            k,
        }
    }
}

impl Problem for DTLZ2 {
    fn name(&self) -> &'static str {
        "DTLZ2"
    }

    fn description(&self) -> &'static str {
        "Deb–Thiele–Laumanns–Zitzler benchmark problem 2"
    }

    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        self.m
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let g = x[x.len() - self.k..]
            .iter()
            .map(|xi| (xi - 0.5).powi(2))
            .sum::<f64>();

        let mut f = vec![1.0 + g; self.m];

        for i in 0..self.m {
            for &xj in x.iter().take(self.m - i - 1) {
                f[i] *= (xj * std::f64::consts::FRAC_PI_2).cos();
            }
            if i > 0 {
                f[i] *= (x[self.m - i - 1] * std::f64::consts::FRAC_PI_2).sin();
            }
        }

        f
    }

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for x in features {
            *x = x.clamp(0.0, 1.0);
        }
    }
}

//
// DTLZ3
//
pub struct DTLZ3 {
    ranges: Vec<(f64, f64)>,
    m: usize,
    k: usize,
}

impl DTLZ3 {
    pub fn new(num_objectives: usize, num_variables: usize) -> Self {
        let k = num_variables - num_objectives + 1;
        Self {
            ranges: vec![(0.0, 1.0); num_variables],
            m: num_objectives,
            k,
        }
    }
}

impl Problem for DTLZ3 {
    fn name(&self) -> &'static str {
        "DTLZ3"
    }

    fn description(&self) -> &'static str {
        "Deb–Thiele–Laumanns–Zitzler benchmark problem 3"
    }

    fn num_variables(&self) -> usize {
        self.ranges.len()
    }

    fn num_objectives(&self) -> usize {
        self.m
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let g = dtlz_g(x, self.k);
        let mut f = vec![1.0 + g; self.m];

        for i in 0..self.m {
            for &xj in x.iter().take(self.m - i - 1) {
                f[i] *= (xj * std::f64::consts::FRAC_PI_2).cos();
            }
            if i > 0 {
                f[i] *= (x[self.m - i - 1] * std::f64::consts::FRAC_PI_2).sin();
            }
        }

        f
    }

    fn calculate_objectives_in_place(&self, x: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(x);
        out.copy_from_slice(&vals);
    }

    fn repair_solution(&self, features: &mut [f64]) {
        for x in features {
            *x = x.clamp(0.0, 1.0);
        }
    }
}
