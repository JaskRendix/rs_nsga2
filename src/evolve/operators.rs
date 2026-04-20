use crate::data::{crowding_operator, Individual};
use rand::RngCore;

pub fn tournament(pop: &[Individual], rng: &mut dyn RngCore) -> usize {
    let n = pop.len();
    let i = (rng.next_u64() as usize) % n;
    let mut j = (rng.next_u64() as usize) % n;
    while i == j {
        j = (rng.next_u64() as usize) % n;
    }

    match crowding_operator(&pop[i], &pop[j]) {
        std::cmp::Ordering::Less => i,
        _ => j,
    }
}

pub fn sbx_crossover(
    p1: &Individual,
    p2: &Individual,
    eta: f64,
    ranges: &[(f64, f64)],
    rng: &mut dyn RngCore,
) -> (Individual, Individual) {
    let mut c1 = Individual::new(p1.features.clone());
    let mut c2 = Individual::new(p2.features.clone());
    let inv = 1.0 / (eta + 1.0);

    for (i, &(min, max)) in ranges.iter().enumerate() {
        let x1 = p1.features[i];
        let x2 = p2.features[i];
        let rand_val = (rng.next_u64() as f64) / (u64::MAX as f64);

        if rand_val <= 0.5 && (x1 - x2).abs() > f64::EPSILON {
            let (y1, y2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let u = (rng.next_u64() as f64) / (u64::MAX as f64);
            let beta = if u <= 0.5 {
                (2.0 * u).powf(inv)
            } else {
                (1.0 / (2.0 * (1.0 - u))).powf(inv)
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

pub fn polynomial_mutation(
    ind: &mut Individual,
    eta: f64,
    ranges: &[(f64, f64)],
    mutation_prob: f64,
    rng: &mut dyn RngCore,
) {
    let inv = 1.0 / (eta + 1.0);

    for (i, &(min, max)) in ranges.iter().enumerate() {
        let rand_prob = (rng.next_u64() as f64) / (u64::MAX as f64);
        if rand_prob > mutation_prob {
            continue;
        }

        let x = ind.features[i];
        let u = (rng.next_u64() as f64) / (u64::MAX as f64);
        let delta = if u < 0.5 {
            let bl = ((x - min) / (max - min)).clamp(0.0, 1.0);
            let b = 2.0 * u + (1.0 - 2.0 * u) * (1.0 - bl).powf(eta + 1.0);
            b.powf(inv) - 1.0
        } else {
            let bu = ((max - x) / (max - min)).clamp(0.0, 1.0);
            let b = 2.0 * (1.0 - u) + 2.0 * (u - 0.5) * (1.0 - bu).powf(eta + 1.0);
            1.0 - b.powf(inv)
        };

        ind.features[i] = (x + delta * (max - min)).clamp(min, max);
    }
}
