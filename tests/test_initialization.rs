use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rs_nsga2::evolve::initialization::initialize_population;
use rs_nsga2::problem::{Problem, Schaffer};

#[test]
fn test_initialize_population_size_and_bounds() {
    let problem = Schaffer::default();
    let num_variables = problem.num_variables();
    let ranges = problem.variable_ranges().to_vec();
    let population_size = 50;

    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let population =
        initialize_population(&problem, population_size, num_variables, &ranges, &mut rng);

    assert_eq!(population.len(), population_size);

    for ind in &population {
        assert_eq!(ind.features.len(), num_variables);
        for (val, &(min, max)) in ind.features.iter().zip(&ranges) {
            assert!(
                *val >= min && *val <= max,
                "Feature value {} out of bounds [{}, {}]",
                val,
                min,
                max
            );
        }
        assert!(!ind.objectives.is_empty());
    }
}

#[test]
fn test_initialize_population_reproducibility() {
    let problem = Schaffer::default();
    let num_variables = problem.num_variables();
    let ranges = problem.variable_ranges().to_vec();
    let population_size = 20;

    let mut rng1 = ChaCha8Rng::seed_from_u64(123);
    let pop1 = initialize_population(&problem, population_size, num_variables, &ranges, &mut rng1);

    let mut rng2 = ChaCha8Rng::seed_from_u64(123);
    let pop2 = initialize_population(&problem, population_size, num_variables, &ranges, &mut rng2);

    assert_eq!(pop1.len(), pop2.len());
    for (i1, i2) in pop1.iter().zip(pop2.iter()) {
        assert_eq!(i1.features, i2.features);
        assert_eq!(i1.objectives, i2.objectives);
    }
}

#[test]
fn test_initialize_population_zero_size() {
    let problem = Schaffer::default();
    let num_variables = problem.num_variables();
    let ranges = problem.variable_ranges().to_vec();

    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let population = initialize_population(&problem, 0, num_variables, &ranges, &mut rng);

    assert!(population.is_empty());
}
