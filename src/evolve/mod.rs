use crate::data::Individual;
use crate::metrics::{hypervolume_2d, igd};
use crate::problem::Problem;
use crate::sort::Nsga2Sorter;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

pub mod operators;
use self::operators::{polynomial_mutation, sbx_crossover, tournament};

pub struct Evolution<P: Problem> {
    pub problem: P,
    pub population_size: usize,
    pub num_generations: usize,
    crossover_param: f64,
    mutation_param: f64,
    reference_point: Option<Vec<f64>>,
    true_front: Option<Vec<Vec<f64>>>, // For IGD
    convergence_threshold: Option<(usize, f64)>,
    seed: Option<u64>,
    num_variables: usize,
    ranges: Vec<(f64, f64)>,
}

pub struct RunResult {
    pub pareto_front: Vec<Individual>,
    pub history: Vec<Vec<Individual>>,
    pub hypervolume_history: Vec<f64>,
    pub igd_history: Vec<f64>,
    pub generations_completed: usize,
}

impl<P: Problem> Evolution<P> {
    pub fn new(problem: P, population_size: usize, num_generations: usize) -> Self {
        let num_variables = problem.num_variables();
        let ranges = problem.variable_ranges();

        Self {
            problem,
            population_size,
            num_generations,
            crossover_param: 20.0,
            mutation_param: 20.0,
            reference_point: None,
            true_front: None,
            convergence_threshold: None,
            seed: None,
            num_variables,
            ranges,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_reference_point(mut self, point: Vec<f64>) -> Self {
        self.reference_point = Some(point);
        self
    }

    pub fn with_true_front(mut self, front: Vec<Vec<f64>>) -> Self {
        self.true_front = Some(front);
        self
    }

    pub fn with_crossover_param(mut self, eta: f64) -> Self {
        assert!(eta > 0.0, "crossover_param must be positive, got {}", eta);
        self.crossover_param = eta;
        self
    }

    pub fn with_mutation_param(mut self, eta: f64) -> Self {
        assert!(eta > 0.0, "mutation_param must be positive, got {}", eta);
        self.mutation_param = eta;
        self
    }

    pub fn with_convergence_threshold(mut self, window: usize, min_delta: f64) -> Self {
        assert!(
            self.reference_point.is_some(),
            "convergence_threshold requires a reference_point to be set first"
        );
        assert!(window >= 2, "convergence window must be >= 2");
        assert!(min_delta >= 0.0, "min_delta must be >= 0.0");

        self.convergence_threshold = Some((window, min_delta));
        self
    }

    pub fn evolve(&self) -> RunResult {
        let mut rng: Box<dyn RngCore> = match self.seed {
            Some(s) => Box::new(ChaCha8Rng::seed_from_u64(s)),
            None => Box::new(thread_rng()),
        };

        let mut population = self.initialize_population(&mut *rng);
        let mut history = Vec::with_capacity(self.num_generations);
        let mut hypervolume_history = Vec::with_capacity(self.num_generations);
        let mut igd_history = Vec::with_capacity(self.num_generations);

        for _ in 0..self.num_generations {
            let mut offspring = self.create_offspring(&population, &mut *rng);

            // Parallel evaluation
            offspring.par_iter_mut().for_each(|ind| {
                ind.objectives = self.problem.calculate_objectives(&ind.features);
                ind.constraint_violations = self.problem.constraint_violations(&ind.features);
                ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
            });

            population.extend(offspring);

            // Sort and select next generation
            let fronts = Nsga2Sorter::fast_nondominated_sort(&mut population);

            // Extract the Rank 0 front for history BEFORE we truncate the population
            let current_front_indices = &fronts[0];
            let front_snapshot: Vec<Individual> = current_front_indices
                .iter()
                .map(|&i| population[i].clone())
                .collect();

            // Truncate population for next gen
            let mut next = Vec::with_capacity(self.population_size);
            for front in fronts.into_iter() {
                if next.len() + front.len() <= self.population_size {
                    for i in front {
                        next.push(population[i].clone());
                    }
                } else {
                    let mut last: Vec<_> =
                        front.into_iter().map(|i| population[i].clone()).collect();
                    Nsga2Sorter::calculate_crowding_distance(&mut last);
                    last.sort_by(|a, b| {
                        b.crowding_distance
                            .partial_cmp(&a.crowding_distance)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    next.extend(last.into_iter().take(self.population_size - next.len()));
                    break;
                }
            }
            population = next;

            // Metrics Calculation
            let current_objectives: Vec<Vec<f64>> = front_snapshot
                .iter()
                .map(|ind| ind.objectives.clone())
                .collect();

            if let Some(ref ref_point) = self.reference_point {
                hypervolume_history.push(hypervolume_2d(&current_objectives, ref_point));
            } else {
                hypervolume_history.push(f64::NAN);
            }

            if let Some(ref true_f) = self.true_front {
                igd_history.push(igd(true_f, &current_objectives));
            } else {
                igd_history.push(f64::NAN);
            }

            history.push(front_snapshot);

            // Early Stopping
            if let Some((window, min_delta)) = self.convergence_threshold {
                if hypervolume_history.len() >= window {
                    let recent = &hypervolume_history[hypervolume_history.len() - window..];
                    if (recent.last().unwrap() - recent.first().unwrap()).abs() < min_delta {
                        break;
                    }
                }
            }
        }

        let final_front = history.last().cloned().unwrap_or_default();
        let generations_completed = hypervolume_history.len();

        RunResult {
            pareto_front: final_front,
            history,
            hypervolume_history,
            igd_history,
            generations_completed,
        }
    }

    fn initialize_population(&self, rng: &mut dyn RngCore) -> Vec<Individual> {
        (0..self.population_size)
            .map(|_| {
                let features = (0..self.num_variables)
                    .map(|i| {
                        let (min, max) = self.ranges[i];
                        min + (max - min) * (rng.next_u64() as f64 / u64::MAX as f64)
                    })
                    .collect::<Vec<f64>>();

                let mut ind = Individual::new(features);
                ind.objectives = self.problem.calculate_objectives(&ind.features);
                ind.constraint_violations = self.problem.constraint_violations(&ind.features);
                ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
                ind
            })
            .collect()
    }

    fn create_offspring(&self, parents: &[Individual], rng: &mut dyn RngCore) -> Vec<Individual> {
        let mut offspring = Vec::with_capacity(self.population_size);
        let mutation_prob = 1.0 / self.num_variables as f64;

        while offspring.len() < self.population_size {
            let p1 = tournament(parents, rng);
            let p2 = tournament(parents, rng);

            let (mut c1, mut c2) = sbx_crossover(
                &parents[p1],
                &parents[p2],
                self.crossover_param,
                &self.ranges,
                rng,
            );

            polynomial_mutation(
                &mut c1,
                self.mutation_param,
                &self.ranges,
                mutation_prob,
                rng,
            );
            polynomial_mutation(
                &mut c2,
                self.mutation_param,
                &self.ranges,
                mutation_prob,
                rng,
            );

            offspring.push(c1);
            offspring.push(c2);
        }

        offspring
    }
}
