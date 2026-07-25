use crate::data::{crowding_operator, Individual};
use rand::Rng;
use rand::RngCore;

#[inline]
pub fn tournament(pop: &[Individual], rng: &mut dyn RngCore) -> usize {
    let n = pop.len();

    if n == 0 {
        panic!("tournament() called with empty population");
    }
    if n == 1 {
        return 0;
    }

    let i = rng.gen::<usize>() % n;
    let mut j = rng.gen::<usize>() % n;

    while i == j {
        j = rng.gen::<usize>() % n;
    }

    match crowding_operator(&pop[i], &pop[j]) {
        std::cmp::Ordering::Less => i,
        _ => j,
    }
}

#[inline]
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
        let rand_val: f64 = rng.gen();

        if rand_val <= 0.5 && (x1 - x2).abs() > f64::EPSILON {
            let (y1, y2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let u: f64 = rng.gen();

            let beta = if u <= 0.5 {
                (2.0 * u).powf(inv)
            } else {
                (1.0 / (2.0 * (1.0 - u))).powf(inv)
            };

            let mid = 0.5 * (y1 + y2);
            let diff = 0.5 * beta * (y2 - y1);

            c1.features[i] = (mid - diff).clamp(min, max);
            c2.features[i] = (mid + diff).clamp(min, max);
        } else {
            c1.features[i] = x1;
            c2.features[i] = x2;
        }
    }

    (c1, c2)
}

#[inline]
pub fn polynomial_mutation(
    ind: &mut Individual,
    eta: f64,
    ranges: &[(f64, f64)],
    mutation_prob: f64,
    rng: &mut dyn RngCore,
) {
    let inv = 1.0 / (eta + 1.0);

    for (i, &(min, max)) in ranges.iter().enumerate() {
        let span = max - min;

        // Skip zero-range dimensions
        if span.abs() <= f64::EPSILON {
            ind.features[i] = min;
            continue;
        }

        // Mutation probability
        let u: f64 = rng.gen();
        if !(0.0..=mutation_prob).contains(&u) {
            continue;
        }

        let x = ind.features[i];
        let u2: f64 = rng.gen();

        let delta = if u2 < 0.5 {
            let bl = (x - min) / span;
            let b = 2.0 * u2 + (1.0 - 2.0 * u2) * (1.0 - bl).powf(eta + 1.0);
            b.powf(inv) - 1.0
        } else {
            let bu = (max - x) / span;
            let b = 2.0 * (1.0 - u2) + 2.0 * (u2 - 0.5) * (1.0 - bu).powf(eta + 1.0);
            1.0 - b.powf(inv)
        };

        ind.features[i] = (x + delta * span).clamp(min, max);
    }
}
