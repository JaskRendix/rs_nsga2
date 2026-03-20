use crate::data::Individual;

pub struct Nsga2Sorter;
struct SortState {
    domination_count: i32,
    dominated_indices: Vec<usize>,
}
impl Nsga2Sorter {
    pub fn fast_nondominated_sort(population: &mut [Individual]) -> Vec<Vec<usize>> {
        let n = population.len();
        let mut states: Vec<SortState> = (0..n)
            .map(|_| SortState {
                domination_count: 0,
                dominated_indices: Vec::new(),
            })
            .collect();

        let mut fronts = vec![Vec::new()];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if population[i].dominates(&population[j]) {
                    states[i].dominated_indices.push(j);
                } else if population[j].dominates(&population[i]) {
                    states[i].domination_count += 1;
                }
            }
            if states[i].domination_count == 0 {
                population[i].rank = 0;
                fronts[0].push(i);
            }
        }

        let mut i = 0;
        while i < fronts.len() && !fronts[i].is_empty() {
            let mut next = Vec::new();
            for &p in &fronts[i] {
                let dominated: Vec<usize> = states[p].dominated_indices.clone();
                for &q in &dominated {
                    states[q].domination_count -= 1;
                    if states[q].domination_count == 0 {
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
