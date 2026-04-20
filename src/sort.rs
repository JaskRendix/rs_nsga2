use crate::data::Individual;
use rayon::prelude::*;

pub struct Nsga2Sorter;

#[derive(Clone)]
struct DomInfo {
    domination_count: i32,
    dominated: Vec<usize>,
}

impl Nsga2Sorter {
    pub fn fast_nondominated_sort(pop: &mut [Individual]) -> Vec<Vec<usize>> {
        let n = pop.len();

        // Compute domination info in parallel
        let dom_info: Vec<DomInfo> = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut count = 0;
                let mut dominated = Vec::new();

                for j in 0..n {
                    if i == j {
                        continue;
                    }

                    match pop[i].dominance_relation(&pop[j]) {
                        crate::data::DomRelation::IDominatesJ => dominated.push(j),
                        crate::data::DomRelation::JDominatesI => count += 1,
                        crate::data::DomRelation::None => {}
                    }
                }

                DomInfo {
                    domination_count: count,
                    dominated,
                }
            })
            .collect();

        // Sequential front extraction
        let mut dom_info = dom_info;
        let mut fronts: Vec<Vec<usize>> = vec![Vec::new()];

        for i in 0..n {
            if dom_info[i].domination_count == 0 {
                pop[i].rank = 0;
                fronts[0].push(i);
            }
        }

        let mut f = 0;
        while f < fronts.len() && !fronts[f].is_empty() {
            let mut next = Vec::new();

            for &p in &fronts[f] {
                // Clone dominated list to avoid overlapping borrows
                let dominated = dom_info[p].dominated.clone();

                for q in dominated {
                    dom_info[q].domination_count -= 1;
                    if dom_info[q].domination_count == 0 {
                        pop[q].rank = f + 1;
                        next.push(q);
                    }
                }
            }

            if !next.is_empty() {
                fronts.push(next);
            }

            f += 1;
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
            // Stable, NaN‑safe sort
            front.sort_by(|a, b| {
                a.objectives[obj]
                    .partial_cmp(&b.objectives[obj])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

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
