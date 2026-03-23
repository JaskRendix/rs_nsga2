use crate::data::Individual;
use rayon::prelude::*;

pub struct Nsga2Sorter;

struct SortState {
    domination_count: i32,
    dominated_indices: Vec<usize>,
}

impl Nsga2Sorter {
    pub fn fast_nondominated_sort(population: &mut [Individual]) -> Vec<Vec<usize>> {
        let n = population.len();

        // Build domination data in parallel — each row i is independent
        let states: Vec<SortState> = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut domination_count = 0i32;
                let mut dominated_indices = Vec::new();
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    if population[i].dominates(&population[j]) {
                        dominated_indices.push(j);
                    } else if population[j].dominates(&population[i]) {
                        domination_count += 1;
                    }
                }
                SortState {
                    domination_count,
                    dominated_indices,
                }
            })
            .collect();

        // Front extraction is sequential — has data dependencies
        let mut states: Vec<_> = states
            .into_iter()
            .map(|s| (s.domination_count, s.dominated_indices))
            .collect();
        let mut fronts = vec![Vec::new()];

        for i in 0..n {
            if states[i].0 == 0 {
                population[i].rank = 0;
                fronts[0].push(i);
            }
        }

        let mut i = 0;
        while i < fronts.len() && !fronts[i].is_empty() {
            let mut next = Vec::new();
            for &p in &fronts[i] {
                for qi in 0..states[p].1.len() {
                    let q = states[p].1[qi];
                    states[q].0 -= 1;
                    if states[q].0 == 0 {
                        population[q].rank = i + 1;
                        next.push(q);
                    }
                }
            }
            if !next.is_empty() {
                fronts.push(next);
            }
            i += 1;
        }

        fronts
    }

    pub fn calculate_crowding_distance(front: &mut [Individual]) {
        let n = front.len();
        if n == 0 {
            return;
        }
        if n <= 2 {
            for ind in front.iter_mut() {
                ind.crowding_distance = f64::INFINITY;
            }
            return;
        }

        for ind in front.iter_mut() {
            ind.crowding_distance = 0.0;
        }

        let m = front[0].objectives.len();
        for obj in 0..m {
            front.sort_by(|a, b| a.objectives[obj].partial_cmp(&b.objectives[obj]).unwrap());
            front[0].crowding_distance = f64::INFINITY;
            front[n - 1].crowding_distance = f64::INFINITY;

            let min = front[0].objectives[obj];
            let max = front[n - 1].objectives[obj];
            let range = max - min;

            if range > 0.0 {
                for i in 1..n - 1 {
                    let dist =
                        (front[i + 1].objectives[obj] - front[i - 1].objectives[obj]) / range;
                    front[i].crowding_distance += dist;
                }
            }
        }
    }
}
