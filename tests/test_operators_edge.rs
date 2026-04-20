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
fn test_tournament_handles_single_element_population() {
    let pop = vec![ind(0.0, 0.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let idx = tournament(&pop, &mut rng);
    assert_eq!(idx, 0);
}

#[test]
fn test_sbx_identical_parents_produces_identical_children() {
    let p = ind(0.4, 0.6);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(2);

    let (c1, c2) = sbx_crossover(&p, &p, 20.0, &ranges, &mut rng);

    assert_eq!(c1.features, p.features);
    assert_eq!(c2.features, p.features);
}

#[test]
fn test_sbx_handles_zero_range_dimension() {
    let p1 = ind(0.2, 0.5);
    let p2 = ind(0.8, 0.5);
    let ranges = vec![(0.0, 1.0), (0.5, 0.5)];
    let mut rng = ChaCha8Rng::seed_from_u64(3);

    let (c1, c2) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng);

    assert_eq!(c1.features[1], 0.5);
    assert_eq!(c2.features[1], 0.5);
}

#[test]
fn test_polynomial_mutation_zero_range_dimension_no_change() {
    let mut ind = ind(0.3, 0.5);
    let ranges = vec![(0.0, 1.0), (0.5, 0.5)];
    let mut rng = ChaCha8Rng::seed_from_u64(4);

    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    assert_eq!(ind.features[1], 0.5);
    assert!((0.0..=1.0).contains(&ind.features[0]));
}

#[test]
fn test_polynomial_mutation_at_lower_bound_stays_in_bounds() {
    let mut ind = ind(0.0, 0.0);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(5);

    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    assert!((0.0..=1.0).contains(&ind.features[0]));
    assert!((0.0..=1.0).contains(&ind.features[1]));
}

#[test]
fn test_polynomial_mutation_at_upper_bound_stays_in_bounds() {
    let mut ind = ind(1.0, 1.0);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(6);

    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    assert!((0.0..=1.0).contains(&ind.features[0]));
    assert!((0.0..=1.0).contains(&ind.features[1]));
}

#[test]
fn test_polynomial_mutation_zero_probability_no_change() {
    let mut ind = ind(0.3, 0.7);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(7);

    let before = ind.features.clone();
    polynomial_mutation(&mut ind, 20.0, &ranges, 0.0, &mut rng);

    assert_eq!(ind.features, before);
}
