use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct SphereAndSquare {
    ranges: [(f64, f64); 2],
}

impl SphereAndSquare {
    fn new() -> Self {
        Self {
            ranges: [(-5.0, 5.0), (-5.0, 5.0)],
        }
    }
}

impl Problem for SphereAndSquare {
    fn num_variables(&self) -> usize {
        2
    }
    fn num_objectives(&self) -> usize {
        2
    }
    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        let f1 = x[0].powi(2) + x[1].powi(2);
        let f2 = (x[0] - 2.0).powi(2) + (x[1] - 2.0).powi(2);
        vec![f1, f2]
    }
}

fn main() {
    println!("Running NSGA-II on a custom bi-objective problem...");

    let result = Evolution::new(SphereAndSquare::new(), 100, 150)
        .with_seed(99)
        .evolve();

    println!("Custom evolution complete!");
    println!("Final Pareto front size: {}", result.pareto_front.len());

    for (i, ind) in result.pareto_front.iter().enumerate().take(3) {
        println!("  Point {}: objectives = {:?}", i, ind.objectives);
    }
}
