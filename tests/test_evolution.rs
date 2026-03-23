use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::{Problem, Schaffer};

struct DummyProblem;
impl Problem for DummyProblem {
    fn num_variables(&self) -> usize {
        2
    }
    fn num_objectives(&self) -> usize {
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
    let result = evo.evolve();
    assert!(result.pareto_front.len() <= 40);
}

#[test]
fn test_evolve_respects_bounds() {
    let evo = Evolution::new(DummyProblem, 40, 5);
    let result = evo.evolve();
    for ind in result.pareto_front {
        assert!(ind.features.len() == 2);
        assert!((0.0..=1.0).contains(&ind.features[0]));
        assert!((0.0..=1.0).contains(&ind.features[1]));
    }
}

#[test]
fn test_schaffer_front_is_pareto_optimal() {
    let evo = Evolution::new(Schaffer, 50, 20)
        .with_crossover_param(15.0)
        .with_mutation_param(10.0);
    let result = evo.evolve();
    let front = &result.pareto_front;

    assert!(!front.is_empty());

    for a in front {
        for b in front {
            let dominates = (b.objectives[0] <= a.objectives[0]
                && b.objectives[1] <= a.objectives[1])
                && (b.objectives != a.objectives);
            assert!(!dominates, "Front contains dominated solutions");
        }
    }

    for ind in front {
        assert!(
            ind.objectives[0] >= 0.0 && ind.objectives[0] <= 4.0 + 0.5,
            "f1={} outside Pareto front range [0, 4]",
            ind.objectives[0]
        );
        assert!(
            ind.objectives[1] >= 0.0 && ind.objectives[1] <= 4.0 + 0.5,
            "f2={} outside Pareto front range [0, 4]",
            ind.objectives[1]
        );
        let x = ind.objectives[0].sqrt();
        let expected_f2 = (x - 2.0).powi(2);
        assert!(
            (ind.objectives[1] - expected_f2).abs() < 0.5,
            "Solution deviates from true Pareto front: f1={:.4}, f2={:.4}, expected f2≈{:.4}",
            ind.objectives[0],
            ind.objectives[1],
            expected_f2
        );
    }
}

#[test]
fn test_evolve_odd_population_size() {
    let evo = Evolution::new(DummyProblem, 41, 5);
    let result = evo.evolve();
    assert!(result.pareto_front.len() <= 42);
}

#[test]
fn test_offspring_features_differ_from_parents_sometimes() {
    let evo = Evolution::new(Schaffer, 50, 20)
        .with_crossover_param(15.0)
        .with_mutation_param(10.0);
    let result = evo.evolve();
    let unique: std::collections::HashSet<_> = result
        .pareto_front
        .iter()
        .map(|i| i.features[0].to_bits())
        .collect();
    assert!(unique.len() > 1);
}

#[test]
fn test_history_length_matches_generations() {
    let evo = Evolution::new(DummyProblem, 40, 10);
    let result = evo.evolve();
    assert_eq!(result.history.len(), 10);
}

#[test]
fn test_history_fronts_are_non_empty() {
    let evo = Evolution::new(DummyProblem, 40, 5);
    let result = evo.evolve();
    for snapshot in &result.history {
        assert!(!snapshot.is_empty());
    }
}

#[test]
fn test_history_last_matches_pareto_front() {
    let evo = Evolution::new(DummyProblem, 40, 5);
    let result = evo.evolve();
    let last = result.history.last().unwrap();
    assert_eq!(last.len(), result.pareto_front.len());
}

#[test]
fn test_sbx_children_within_bounds() {
    let evo = Evolution::new(DummyProblem, 100, 20);
    let result = evo.evolve();
    for ind in &result.pareto_front {
        for &f in &ind.features {
            assert!(
                (0.0..=1.0).contains(&f),
                "SBX produced out-of-bounds feature: {}",
                f
            );
        }
    }
}

#[test]
fn test_custom_crossover_param_produces_valid_front() {
    let evo = Evolution::new(DummyProblem, 40, 5).with_crossover_param(15.0);
    let result = evo.evolve();
    assert!(!result.pareto_front.is_empty());
}

#[test]
fn test_custom_mutation_param_produces_valid_front() {
    let evo = Evolution::new(DummyProblem, 40, 5).with_mutation_param(10.0);
    let result = evo.evolve();
    assert!(!result.pareto_front.is_empty());
}

#[test]
#[should_panic(expected = "crossover_param must be positive")]
fn test_invalid_crossover_param_panics() {
    Evolution::new(DummyProblem, 40, 5).with_crossover_param(-1.0);
}

#[test]
#[should_panic(expected = "mutation_param must be positive")]
fn test_invalid_mutation_param_panics() {
    Evolution::new(DummyProblem, 40, 5).with_mutation_param(0.0);
}
#[test]
fn test_generations_completed_without_early_stopping() {
    let evo = Evolution::new(DummyProblem, 40, 10).with_reference_point(vec![3.0, 3.0]);
    let result = evo.evolve();
    assert_eq!(result.generations_completed, 10);
}

#[test]
fn test_early_stopping_fires_before_max_generations() {
    // min_delta=1000.0 is impossible to satisfy so it will stop at window
    let evo = Evolution::new(Schaffer, 50, 100)
        .with_reference_point(vec![10.0, 10.0])
        .with_convergence_threshold(5, 1000.0);
    let result = evo.evolve();
    assert!(
        result.generations_completed < 100,
        "Expected early stopping but ran all 100 generations"
    );
}

#[test]
fn test_generations_completed_matches_history_length() {
    let evo = Evolution::new(Schaffer, 50, 20)
        .with_reference_point(vec![10.0, 10.0])
        .with_convergence_threshold(5, 1000.0);
    let result = evo.evolve();
    assert_eq!(result.generations_completed, result.history.len());
    assert_eq!(
        result.generations_completed,
        result.hypervolume_history.len()
    );
}

#[test]
#[should_panic(expected = "convergence_threshold requires a reference_point")]
fn test_convergence_threshold_without_reference_point_panics() {
    Evolution::new(DummyProblem, 40, 10).with_convergence_threshold(5, 0.01);
}

#[test]
#[should_panic(expected = "convergence window must be >= 2")]
fn test_convergence_window_too_small_panics() {
    Evolution::new(DummyProblem, 40, 10)
        .with_reference_point(vec![3.0, 3.0])
        .with_convergence_threshold(1, 0.01);
}

#[test]
#[should_panic(expected = "min_delta must be >= 0.0")]
fn test_convergence_negative_delta_panics() {
    Evolution::new(DummyProblem, 40, 10)
        .with_reference_point(vec![3.0, 3.0])
        .with_convergence_threshold(5, -0.01);
}
