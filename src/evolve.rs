use crate::data::{crowding_operator, Individual};
use crate::problem::Problem;
use crate::sort::Nsga2Sorter;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Evolution<P: Problem> {
    pub problem: P,
    pub population_size: usize,
    pub num_generations: usize,
    pub crossover_param: f64,
    pub mutation_param: f64,
}

impl<P: Problem> Evolution<P> {
    pub fn new(problem: P, population_size: usize, num_generations: usize) -> Self {
        Self {
            problem,
            population_size,
            num_generations,
            crossover_param: 20.0,
            mutation_param: 20.0,
        }
    }

    pub fn evolve(&self) -> Vec<Individual> {
        let mut population = self.initialize_population();

        for _ in 0..self.num_generations {
            let mut offspring = self.create_offspring(&population);

            offspring.par_iter_mut().for_each(|ind| {
                ind.objectives = self.problem.calculate_objectives(&ind.features);
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
        }

        let fronts = Nsga2Sorter::fast_nondominated_sort(&mut population);
        fronts[0].iter().map(|&i| population[i].clone()).collect()
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
        let j = rng.gen_range(0..pop.len());
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
