use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::{Problem, Schaffer};

struct DummyProblem;

impl Problem for DummyProblem {
    fn num_variables(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 1.0), (0.0, 1.0)]
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0] + x[1], x[0] * x[1]]
    }
}

#[test]
fn test_evolve_population_size() {
    let evo = Evolution::new(DummyProblem, 40, 5);
    let front = evo.evolve();
    assert!(front.len() <= 40);
}

#[test]
fn test_evolve_respects_bounds() {
    let evo = Evolution::new(DummyProblem, 40, 5);
    let front = evo.evolve();

    for ind in front {
        assert!(ind.features.len() == 2);
        assert!(ind.features[0] >= 0.0 && ind.features[0] <= 1.0);
        assert!(ind.features[1] >= 0.0 && ind.features[1] <= 1.0);
    }
}

#[test]
fn test_schaffer_front_is_pareto_optimal() {
    let evo = Evolution::new(Schaffer, 50, 20);
    let front = evo.evolve();

    for a in &front {
        for b in &front {
            let dominates = (b.objectives[0] <= a.objectives[0]
                && b.objectives[1] <= a.objectives[1])
                && (b.objectives != a.objectives);

            assert!(!dominates, "Front contains dominated solutions");
        }
    }
}
