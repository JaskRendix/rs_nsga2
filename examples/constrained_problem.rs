use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct RingConstraintProblem {
    ranges: [(f64, f64); 2],
}

impl RingConstraintProblem {
    fn new() -> Self {
        Self {
            ranges: [(0.0, 5.0), (0.0, 5.0)],
        }
    }
}

impl Problem for RingConstraintProblem {
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
        vec![x[0], x[1]]
    }

    fn constraint_violations(&self, x: &[f64]) -> Vec<f64> {
        // Constraint: x[0] + x[1] >= 3.0 -> violation is 3.0 - (x[0] + x[1]) if negative
        let violation = 3.0 - (x[0] + x[1]);
        vec![if violation > 0.0 { violation } else { 0.0 }]
    }
}

fn main() {
    println!("Running constrained optimization...");

    let result = Evolution::new(RingConstraintProblem::new(), 100, 200)
        .with_seed(1337)
        .evolve();

    println!("Constrained evolution complete!");
    println!(
        "Final valid Pareto front size: {}",
        result.pareto_front.len()
    );

    // Verify constraints on the final front
    let all_feasible = result.pareto_front.iter().all(|ind| ind.feasible);
    println!(
        "All returned solutions are strictly feasible: {}",
        all_feasible
    );

    if let Some(ind) = result.pareto_front.first() {
        println!("Sample optimal point features: {:?}", ind.features);
        println!("Sample optimal point objectives: {:?}", ind.objectives);
    }
}
