use crate::data::Individual;
use crate::sort::Nsga2Sorter;

pub fn select_next_generation(
    population: &mut [Individual],
    population_size: usize,
) -> Vec<Individual> {
    let fronts = Nsga2Sorter::fast_nondominated_sort(population);
    let mut next = Vec::with_capacity(population_size);

    for front in fronts {
        if next.len() + front.len() <= population_size {
            for i in front {
                next.push(population[i].clone());
            }
        } else {
            let mut last: Vec<_> = front.into_iter().map(|i| population[i].clone()).collect();
            Nsga2Sorter::calculate_crowding_distance(&mut last);
            last.sort_unstable_by(|a, b| {
                b.crowding_distance
                    .partial_cmp(&a.crowding_distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            next.extend(last.into_iter().take(population_size - next.len()));
            break;
        }
    }
    next
}
