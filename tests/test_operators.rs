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
fn test_tournament_prefers_lower_rank() {
    // Create individuals with artificial ranks
    let mut p1 = ind(0.0, 0.0);
    let mut p2 = ind(5.0, 5.0);

    p1.rank = 0;
    p2.rank = 1;

    let pop = vec![p1, p2];
    let mut rng = ChaCha8Rng::seed_from_u64(999);

    let mut count0 = 0;
    let mut count1 = 0;

    for _ in 0..500 {
        let idx = tournament(&pop, &mut rng);
        if idx == 0 { count0 += 1 } else { count1 += 1 }
    }

    assert!(count0 > count1);
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
fn test_polynomial_mutation_zero_probability_no_change() {
    let mut ind = ind(0.3, 0.7);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(7);

    let before = ind.features.clone();
    polynomial_mutation(&mut ind, 20.0, &ranges, 0.0, &mut rng);

    assert_eq!(ind.features, before);
}

#[test]
fn test_sbx_deterministic_with_seed() {
    let p1 = ind(0.2, 0.8);
    let p2 = ind(0.7, 0.3);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];

    let mut rng1 = ChaCha8Rng::seed_from_u64(123);
    let mut rng2 = ChaCha8Rng::seed_from_u64(123);

    let (c1a, c2a) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng1);
    let (c1b, c2b) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng2);

    assert_eq!(c1a.features, c1b.features);
    assert_eq!(c2a.features, c2b.features);
}

#[test]
fn test_polynomial_mutation_deterministic_with_seed() {
    let mut i1 = ind(0.5, 0.5);
    let mut i2 = ind(0.5, 0.5);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];

    let mut rng1 = ChaCha8Rng::seed_from_u64(999);
    let mut rng2 = ChaCha8Rng::seed_from_u64(999);

    polynomial_mutation(&mut i1, 20.0, &ranges, 1.0, &mut rng1);
    polynomial_mutation(&mut i2, 20.0, &ranges, 1.0, &mut rng2);

    assert_eq!(i1.features, i2.features);
}

#[test]
fn test_sbx_symmetry() {
    let p1 = ind(0.2, 0.8);
    let p2 = ind(0.7, 0.3);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(321);

    let (c1, c2) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng);

    // Swap parents, same seed → children swapped
    let mut rng2 = ChaCha8Rng::seed_from_u64(321);
    let (d1, d2) = sbx_crossover(&p2, &p1, 20.0, &ranges, &mut rng2);

    assert_eq!(c1.features, d2.features);
    assert_eq!(c2.features, d1.features);
}

#[test]
fn test_tournament_large_population() {
    let pop: Vec<_> = (0..10_000).map(|i| ind(i as f64, i as f64)).collect();
    let mut rng = ChaCha8Rng::seed_from_u64(123);

    for _ in 0..500 {
        let idx = tournament(&pop, &mut rng);
        assert!(idx < pop.len());
    }
}

#[test]
fn test_polynomial_mutation_probability_one_mutates_all() {
    let mut ind = ind(0.5, 0.5);
    let ranges = vec![(0.0, 1.0), (0.0, 1.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(888);

    let before = ind.features.clone();
    polynomial_mutation(&mut ind, 20.0, &ranges, 1.0, &mut rng);

    assert_ne!(ind.features, before);
}

#[test]
fn test_sbx_no_nan_or_inf() {
    let p1 = ind(0.0, 1e9);
    let p2 = ind(1e9, 0.0);
    let ranges = vec![(0.0, 1e9), (0.0, 1e9)];
    let mut rng = ChaCha8Rng::seed_from_u64(123);

    let (c1, c2) = sbx_crossover(&p1, &p2, 20.0, &ranges, &mut rng);

    for f in c1.features.iter().chain(c2.features.iter()) {
        assert!(f.is_finite());
    }
}
