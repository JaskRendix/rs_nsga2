use crate::data::{crowding_operator, Individual};
use crate::metrics::hypervolume_2d;
use crate::problem::Problem;
use crate::sort::Nsga2Sorter;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Evolution<P: Problem> {
    pub problem: P,
    pub population_size: usize,
    pub num_generations: usize,
    crossover_param: f64,
    mutation_param: f64,
    reference_point: Option<Vec<f64>>,
    convergence_threshold: Option<(usize, f64)>,
}

pub struct RunResult {
    pub pareto_front: Vec<Individual>,
    pub history: Vec<Vec<Individual>>,
    pub hypervolume_history: Vec<f64>,
    pub generations_completed: usize,
}

impl<P: Problem> Evolution<P> {
    pub fn new(problem: P, population_size: usize, num_generations: usize) -> Self {
        Self {
            problem,
            population_size,
            num_generations,
            crossover_param: 20.0,
            mutation_param: 20.0,
            reference_point: None,
            convergence_threshold: None,
        }
    }

    pub fn with_reference_point(mut self, point: Vec<f64>) -> Self {
        self.reference_point = Some(point);
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
        assert!(
            window >= 2,
            "convergence window must be >= 2, got {}",
            window
        );
        assert!(
            min_delta >= 0.0,
            "min_delta must be >= 0.0, got {}",
            min_delta
        );
        self.convergence_threshold = Some((window, min_delta));
        self
    }

    pub fn evolve(&self) -> RunResult {
        assert!(
            self.convergence_threshold.is_none() || self.reference_point.is_some(),
            "convergence_threshold requires a reference_point"
        );

        let mut population = self.initialize_population();
        let mut history = Vec::with_capacity(self.num_generations);
        let mut hypervolume_history = Vec::with_capacity(self.num_generations);

        for _ in 0..self.num_generations {
            let mut offspring = self.create_offspring(&population);

            offspring.par_iter_mut().for_each(|ind| {
                ind.objectives = self.problem.calculate_objectives(&ind.features);
                ind.constraint_violations = self.problem.constraint_violations(&ind.features);
                ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
            });

            population.extend(offspring);

            let fronts = Nsga2Sorter::fast_nondominated_sort(&mut population);

            let mut next = Vec::with_capacity(self.population_size);
            for front in fronts {
                if next.len() + front.len() <= self.population_size {
                    for &i in &front {
                        next.push(population[i].clone());
                    }
                } else {
                    let mut last: Vec<_> = front.iter().map(|&i| population[i].clone()).collect();
                    Nsga2Sorter::calculate_crowding_distance(&mut last);
                    last.sort_by(|a, b| {
                        b.crowding_distance
                            .partial_cmp(&a.crowding_distance)
                            .unwrap()
                    });
                    next.extend(last.into_iter().take(self.population_size - next.len()));
                    break;
                }
            }

            population = next;

            let mut snap = population.clone();
            let fronts = Nsga2Sorter::fast_nondominated_sort(&mut snap);
            let front_snapshot: Vec<Individual> =
                fronts[0].iter().map(|&i| snap[i].clone()).collect();

            if let Some(ref ref_point) = self.reference_point {
                let objectives: Vec<Vec<f64>> = front_snapshot
                    .iter()
                    .map(|ind| ind.objectives.clone())
                    .collect();
                hypervolume_history.push(hypervolume_2d(&objectives, ref_point));
            } else {
                hypervolume_history.push(f64::NAN);
            }

            history.push(front_snapshot);

            if let Some((window, min_delta)) = self.convergence_threshold {
                if hypervolume_history.len() >= window {
                    let recent = &hypervolume_history[hypervolume_history.len() - window..];
                    let improvement = recent.last().unwrap() - recent.first().unwrap();
                    if improvement < min_delta {
                        break;
                    }
                }
            }
        }

        let generations_completed = history.len();

        let mut final_pop = population.clone();
        let fronts = Nsga2Sorter::fast_nondominated_sort(&mut final_pop);
        let pareto_front = fronts[0].iter().map(|&i| final_pop[i].clone()).collect();

        RunResult {
            pareto_front,
            history,
            hypervolume_history,
            generations_completed,
        }
    }

    fn initialize_population(&self) -> Vec<Individual> {
        let mut rng = thread_rng();
        let n = self.problem.num_variables();
        let ranges = self.problem.variable_ranges();

        assert_eq!(n, ranges.len(), "Ranges must match number of variables");

        (0..self.population_size)
            .map(|_| {
                let features = (0..n)
                    .map(|i| {
                        let (a, b) = ranges[i];
                        rng.gen_range(a..b)
                    })
                    .collect::<Vec<f64>>();

                let mut ind = Individual::new(features);
                ind.objectives = self.problem.calculate_objectives(&ind.features);
                debug_assert_eq!(
                    ind.objectives.len(),
                    self.problem.num_objectives(),
                    "calculate_objectives returned wrong number of objectives"
                );
                ind.constraint_violations = self.problem.constraint_violations(&ind.features);
                ind.feasible = ind.constraint_violations.iter().all(|&v| v <= 0.0);
                ind
            })
            .collect()
    }

    fn create_offspring(&self, parents: &[Individual]) -> Vec<Individual> {
        let mut rng = thread_rng();
        let mut offspring = Vec::with_capacity(self.population_size);

        while offspring.len() < self.population_size {
            let p1 = self.tournament(parents, &mut rng);
            let p2 = self.tournament(parents, &mut rng);

            let (mut c1, mut c2) = self.sbx_crossover(&parents[p1], &parents[p2], &mut rng);
            self.polynomial_mutation(&mut c1, &mut rng);
            self.polynomial_mutation(&mut c2, &mut rng);

            offspring.push(c1);
            offspring.push(c2);
        }

        offspring
    }

    fn tournament(&self, pop: &[Individual], rng: &mut impl Rng) -> usize {
        let i = rng.gen_range(0..pop.len());
        let j = loop {
            let k = rng.gen_range(0..pop.len());
            if k != i {
                break k;
            }
        };
        match crowding_operator(&pop[i], &pop[j]) {
            std::cmp::Ordering::Less => i,
            _ => j,
        }
    }

    fn sbx_crossover(
        &self,
        p1: &Individual,
        p2: &Individual,
        rng: &mut impl Rng,
    ) -> (Individual, Individual) {
        let eta = self.crossover_param;
        let ranges = self.problem.variable_ranges();

        let mut c1 = Individual::new(p1.features.clone());
        let mut c2 = Individual::new(p2.features.clone());

        for (i, &(min, max)) in ranges.iter().enumerate() {
            let x1 = p1.features[i];
            let x2 = p2.features[i];

            if rng.gen::<f64>() <= 0.5 && (x1 - x2).abs() > 1e-14 {
                let (y1, y2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

                let u = rng.gen::<f64>();
                let beta = if u <= 0.5 {
                    (2.0 * u).powf(1.0 / (eta + 1.0))
                } else {
                    (1.0 / (2.0 * (1.0 - u))).powf(1.0 / (eta + 1.0))
                };

                c1.features[i] = (0.5 * ((y1 + y2) - beta * (y2 - y1))).clamp(min, max);
                c2.features[i] = (0.5 * ((y1 + y2) + beta * (y2 - y1))).clamp(min, max);
            } else {
                c1.features[i] = x1;
                c2.features[i] = x2;
            }
        }

        (c1, c2)
    }

    fn polynomial_mutation(&self, ind: &mut Individual, rng: &mut impl Rng) {
        let eta = self.mutation_param;
        let n = self.problem.num_variables();
        let ranges = self.problem.variable_ranges();
        let mutation_prob = 1.0 / n as f64;

        for (i, &(min, max)) in ranges.iter().enumerate() {
            if rng.gen::<f64>() > mutation_prob {
                continue;
            }

            let x = ind.features[i];
            let u = rng.gen::<f64>();
            let delta = if u < 0.5 {
                let bl = (x - min) / (max - min);
                let b = 2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(eta + 1.0);
                b.powf(1.0 / (eta + 1.0)) - 1.0
            } else {
                let bu = (max - x) / (max - min);
                let b = 2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(eta + 1.0);
                1.0 - b.powf(1.0 / (eta + 1.0))
            };

            ind.features[i] = (x + delta * (max - min)).clamp(min, max);
        }
    }
}
