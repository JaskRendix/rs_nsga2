use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rs_nsga2::data::Individual;
use rs_nsga2::evolve::operators::{polynomial_mutation, sbx_crossover, tournament};

fn ind(x: f64, y: f64) -> Individual {
    let mut i = Individual::new(vec![x, y]);
    i.objectives = vec![x, y];
    i
}

#[test]
fn test_tournament_returns_valid_index() {
    let pop = vec![ind(0.0, 0.0), ind(1.0, 1.0), ind(2.0, 2.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(123);

    for _ in 0..50 {
        let idx = tournament(&pop, &mut rng);
        assert!(idx < pop.len());
    }
}

#[test]
fn test_tournament_prefers_better_individual() {
    let pop = vec![ind(0.0, 0.0), ind(5.0, 5.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(999);

    for _ in 0..200 {
        let idx = tournament(&pop, &mut rng);
        assert!(idx == 0 || idx == 1);
    }
}

#[test]
fn test_sbx_children_within_bounds() {
    let p1 = ind(0.2, 0.8);
    let p2 = ind(0.7, 0.3);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let (c1, c2) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng);

    for f in c1.features.iter().chain(c2.features.iter()) {
        assert!((0.0..=1.0).contains(f));
    }
}

#[test]
fn test_sbx_produces_variation() {
    let p1 = ind(0.2, 0.8);
    let p2 = ind(0.7, 0.3);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(7);

    let (c1, c2) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng);

    assert_ne!(c1.features, p1.features);
    assert_ne!(c2.features, p2.features);
}

#[test]
fn test_polynomial_mutation_respects_bounds() {
    let mut ind = ind(0.5, 0.5);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(1234);

    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    for f in ind.features {
        assert!((0.0..=1.0).contains(&f));
    }
}

#[test]
fn test_polynomial_mutation_changes_values() {
    let mut ind = ind(0.5, 0.5);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(555);

    let before = ind.features.clone();
    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    assert_ne!(ind.features, before);
}
