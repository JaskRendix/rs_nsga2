use super::operators::{polynomial_mutation, sbx_crossover, tournament};
use crate::data::Individual;
use rand_chacha::ChaCha8Rng;

pub fn create_offspring(
    parents: &[Individual],
    population_size: usize,
    crossover_param: f64,
    mutation_param: f64,
    mutation_prob: f64,
    ranges: &[(f64, f64)],
    rng: &mut ChaCha8Rng,
) -> Vec<Individual> {
    let mut offspring = Vec::with_capacity(population_size);

    while offspring.len() < population_size {
        let p1 = tournament(parents, rng);
        let p2 = tournament(parents, rng);

        let (mut c1, mut c2) =
            sbx_crossover(&parents[p1], &parents[p2], crossover_param, ranges, rng);

        polynomial_mutation(&mut c1, mutation_param, ranges, mutation_prob, rng);
        polynomial_mutation(&mut c2, mutation_param, ranges, mutation_prob, rng);

        offspring.push(c1);
        offspring.push(c2);
    }

    offspring
}
