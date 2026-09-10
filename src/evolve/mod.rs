use crate::data::Individual;
use crate::metrics::{generational_distance, hypervolume_2d, igd};
use crate::problem::Problem;
use crate::sort::Nsga2Sorter;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

pub mod initialization;
pub mod offspring;
pub mod operators;
pub mod selection;

use self::initialization::initialize_population;
use self::offspring::create_offspring;
use self::selection::select_next_generation;

pub struct Evolution<P: Problem> {
    pub problem: P,
    pub population_size: usize,
    pub num_generations: usize,
    crossover_param: f64,
    mutation_param: f64,
    mutation_prob: f64,
    reference_point: Option<Vec<f64>>,
    true_front: Option<Vec<Vec<f64>>>, // For IGD and GD
    convergence_threshold: Option<(usize, f64)>,
    seed: Option<u64>,
    parallel: bool,
    num_variables: usize,
    ranges: Vec<(f64, f64)>,
}

pub struct RunResult {
    pub pareto_front: Vec<Individual>,
    pub history: Vec<Vec<Individual>>,
    pub hypervolume_history: Vec<f64>,
    pub igd_history: Vec<f64>,
    pub gd_history: Vec<f64>,
    pub generations_completed: usize,
}

impl<P: Problem> Evolution<P> {
    pub fn new(problem: P, population_size: usize, num_generations: usize) -> Self {
        let num_variables = problem.num_variables();
        let ranges = problem.variable_ranges().to_vec();

        Self {
            problem,
            population_size,
            num_generations,
            crossover_param: 20.0,
            mutation_param: 20.0,
            mutation_prob: 1.0 / num_variables as f64,
            reference_point: None,
            true_front: None,
            convergence_threshold: None,
            seed: None,
            parallel: true,
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

    pub fn with_mutation_probability(mut self, prob: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&prob),
            "mutation probability must be in [0,1], got {}",
            prob
        );
        self.mutation_prob = prob;
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

    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn evolve(&self) -> RunResult {
        let mut rng = match self.seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => {
                let mut tr = thread_rng();
                ChaCha8Rng::seed_from_u64(tr.r#gen::<u64>())
            }
        };

        let mut population = initialize_population(
            &self.problem,
            self.population_size,
            self.num_variables,
            &self.ranges,
            &mut rng,
        );

        let mut history = Vec::with_capacity(self.num_generations);
        let mut hypervolume_history = Vec::with_capacity(self.num_generations);
        let mut igd_history = Vec::with_capacity(self.num_generations);
        let mut gd_history = Vec::with_capacity(self.num_generations);

        for _ in 0..self.num_generations {
            let mut offspring = create_offspring(
                &population,
                self.population_size,
                self.crossover_param,
                self.mutation_param,
                self.mutation_prob,
                &self.ranges,
                &mut rng,
            );

            if self.parallel {
                offspring.par_iter_mut().for_each(|ind| {
                    self.evaluate_individual(ind);
                });
            } else {
                offspring.iter_mut().for_each(|ind| {
                    self.evaluate_individual(ind);
                });
            }

            population.extend(offspring);

            let fronts_snapshot = Nsga2Sorter::fast_nondominated_sort(&mut population);
            let front_snapshot: Vec<Individual> = fronts_snapshot[0]
                .iter()
                .map(|&i| population[i].clone())
                .collect();

            population = select_next_generation(&mut population, self.population_size);

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
                gd_history.push(generational_distance(true_f, &current_objectives));
            } else {
                igd_history.push(f64::NAN);
                gd_history.push(f64::NAN);
            }

            history.push(front_snapshot);

            if let Some((window, min_delta)) = self.convergence_threshold
                && hypervolume_history.len() >= window
            {
                let recent = &hypervolume_history[hypervolume_history.len() - window..];
                if (recent.last().unwrap() - recent.first().unwrap()).abs() < min_delta {
                    break;
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
            gd_history,
            generations_completed,
        }
    }

    #[inline]
    fn evaluate_individual(&self, ind: &mut Individual) {
        ind.objectives = self.problem.calculate_objectives(&ind.features);
        ind.constraint_violations = self.problem.constraint_violations(&ind.features);
        ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
    }
}
