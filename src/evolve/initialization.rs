use crate::data::Individual;
use crate::problem::Problem;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn initialize_population<P: Problem>(
    problem: &P,
    population_size: usize,
    num_variables: usize,
    ranges: &[(f64, f64)],
    rng: &mut ChaCha8Rng,
) -> Vec<Individual> {
    (0..population_size)
        .map(|_| {
            let features = (0..num_variables)
                .map(|i| {
                    let (min, max) = ranges[i];
                    let u: f64 = rng.r#gen();
                    min + (max - min) * u
                })
                .collect::<Vec<f64>>();

            let mut ind = Individual::new(features);
            ind.objectives = problem.calculate_objectives(&ind.features);
            ind.constraint_violations = problem.constraint_violations(&ind.features);
            ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
            ind
        })
        .collect()
}
