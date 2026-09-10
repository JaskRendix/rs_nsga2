use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rs_nsga2::data::Individual;
use rs_nsga2::evolve::offspring::create_offspring;

fn create_dummy_parents(count: usize) -> Vec<Individual> {
    (0..count)
        .map(|i| {
            let val = i as f64 * 0.5;
            Individual {
                features: vec![val, val + 1.0],
                objectives: vec![val, val],
                constraint_violations: vec![],
                feasible: true,
                rank: 0,
                crowding_distance: 1.0,
            }
        })
        .collect()
}

#[test]
fn test_create_offspring_size_even() {
    let parents = create_dummy_parents(10);
    let ranges = vec![(0.0, 5.0), (0.0, 5.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let population_size = 20;
    let offspring = create_offspring(
        &parents,
        population_size,
        20.0,
        20.0,
        0.5,
        &ranges,
        &mut rng,
    );

    // Since it pushes in pairs, an even population size matches exactly
    assert_eq!(offspring.len(), population_size);
}

#[test]
fn test_create_offspring_size_odd() {
    let parents = create_dummy_parents(10);
    let ranges = vec![(0.0, 5.0), (0.0, 5.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let population_size = 21;
    let offspring = create_offspring(
        &parents,
        population_size,
        20.0,
        20.0,
        0.5,
        &ranges,
        &mut rng,
    );

    // With pairs, an odd population size will slightly overshoot to satisfy the minimum requirement
    assert!(offspring.len() >= population_size);
}

#[test]
fn test_create_offspring_reproducibility() {
    let parents = create_dummy_parents(10);
    let ranges = vec![(0.0, 5.0), (0.0, 5.0)];

    let mut rng1 = ChaCha8Rng::seed_from_u64(999);
    let offspring1 = create_offspring(&parents, 10, 20.0, 20.0, 0.5, &ranges, &mut rng1);

    let mut rng2 = ChaCha8Rng::seed_from_u64(999);
    let offspring2 = create_offspring(&parents, 10, 20.0, 20.0, 0.5, &ranges, &mut rng2);

    assert_eq!(offspring1.len(), offspring2.len());
    for (c1, c2) in offspring1.iter().zip(offspring2.iter()) {
        assert_eq!(c1.features, c2.features);
    }
}

#[test]
fn test_create_offspring_feature_bounds() {
    let parents = create_dummy_parents(10);
    let ranges = vec![(1.0, 3.0), (2.0, 4.0)];
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let offspring = create_offspring(&parents, 20, 20.0, 20.0, 1.0, &ranges, &mut rng);

    for ind in offspring {
        assert_eq!(ind.features.len(), 2);
        assert!(ind.features[0] >= 1.0 && ind.features[0] <= 3.0);
        assert!(ind.features[1] >= 2.0 && ind.features[1] <= 4.0);
    }
}
