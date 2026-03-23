use rs_nsga2::data::Individual;
use rs_nsga2::sort::Nsga2Sorter;

fn ind(obj1: f64, obj2: f64) -> Individual {
    let mut i = Individual::new(vec![]);
    i.objectives = vec![obj1, obj2];
    i
}

#[test]
fn test_fronts_cover_all_indices_exactly_once() {
    let mut pop = vec![
        ind(1.0, 5.0),
        ind(2.0, 4.0),
        ind(3.0, 3.0),
        ind(4.0, 2.0),
        ind(5.0, 1.0),
    ];
    let fronts = Nsga2Sorter::fast_nondominated_sort(&mut pop);
    let mut seen = vec![false; pop.len()];
    for front in &fronts {
        for &idx in front {
            assert!(!seen[idx], "Index appears in multiple fronts");
            seen[idx] = true;
        }
    }
    assert!(
        seen.iter().all(|x| *x),
        "Not all individuals were assigned to a front"
    );
}

#[test]
fn test_each_front_is_mutually_non_dominating() {
    let mut pop = vec![
        ind(1.0, 5.0),
        ind(2.0, 4.0),
        ind(3.0, 3.0),
        ind(4.0, 2.0),
        ind(5.0, 1.0),
    ];
    let fronts = Nsga2Sorter::fast_nondominated_sort(&mut pop);
    for front in fronts {
        for &i in &front {
            for &j in &front {
                if i != j {
                    assert!(
                        !pop[i].dominates(&pop[j]) && !pop[j].dominates(&pop[i]),
                        "Front contains dominated pair"
                    );
                }
            }
        }
    }
}

#[test]
fn test_dominators_have_lower_rank() {
    let mut pop = vec![
        ind(1.0, 1.0), // dominates all
        ind(2.0, 2.0),
        ind(3.0, 3.0),
        ind(4.0, 4.0),
    ];
    let _fronts = Nsga2Sorter::fast_nondominated_sort(&mut pop);
    for i in 0..pop.len() {
        for j in 0..pop.len() {
            if pop[i].dominates(&pop[j]) {
                assert!(
                    pop[i].rank < pop[j].rank,
                    "Dominating individual must have lower rank"
                );
            }
        }
    }
}

#[test]
fn test_crowding_distance_two_points_are_infinite() {
    let mut front = vec![ind(1.0, 2.0), ind(2.0, 1.0)];
    Nsga2Sorter::calculate_crowding_distance(&mut front);
    assert!(front[0].crowding_distance.is_infinite());
    assert!(front[1].crowding_distance.is_infinite());
}

#[test]
fn test_crowding_distance_three_points_boundaries_infinite() {
    let mut front = vec![ind(1.0, 3.0), ind(2.0, 2.0), ind(3.0, 1.0)];
    Nsga2Sorter::calculate_crowding_distance(&mut front);
    assert!(front[0].crowding_distance.is_infinite());
    assert!(front[2].crowding_distance.is_infinite());
    assert!(front[1].crowding_distance.is_finite());
}

#[test]
fn test_crowding_distance_non_negative() {
    let mut front = vec![ind(1.0, 10.0), ind(2.0, 5.0), ind(3.0, 4.9), ind(4.0, 1.0)];
    Nsga2Sorter::calculate_crowding_distance(&mut front);
    for ind in &front {
        assert!(
            ind.crowding_distance >= 0.0 || ind.crowding_distance.is_infinite(),
            "Crowding distance must be >= 0 or ∞"
        );
    }
}

#[test]
fn test_crowding_distance_boundaries_are_infinite() {
    let mut front = vec![ind(0.0, 10.0), ind(1.0, 5.0), ind(2.0, 1.0)];
    Nsga2Sorter::calculate_crowding_distance(&mut front);
    assert!(front[0].crowding_distance.is_infinite());
    assert!(front[2].crowding_distance.is_infinite());
}

#[test]
fn test_sort_all_identical() {
    let mut pop = vec![ind(1.0, 1.0); 10];
    let fronts = Nsga2Sorter::fast_nondominated_sort(&mut pop);
    assert_eq!(fronts.len(), 1);
}

#[test]
fn test_sort_single_individual() {
    let mut pop = vec![ind(2.0, 3.0)];
    let fronts = Nsga2Sorter::fast_nondominated_sort(&mut pop);
    assert_eq!(fronts[0], vec![0]);
}

#[test]
fn test_crowding_distance_zero_range() {
    let mut front = vec![ind(1.0, 1.0), ind(1.0, 1.0), ind(1.0, 1.0)];
    Nsga2Sorter::calculate_crowding_distance(&mut front);
    for ind in &front {
        assert!(!ind.crowding_distance.is_nan());
    }
}
