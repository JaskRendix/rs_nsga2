use rs_nsga2::data::Individual;
use rs_nsga2::evolve::selection::select_next_generation;

#[test]
fn test_select_exact_fit() {
    // Create individuals where fronts fit the population_size exactly
    let mut pop = vec![
        Individual {
            features: vec![1.0],
            objectives: vec![1.0, 4.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![2.0],
            objectives: vec![2.0, 3.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![3.0],
            objectives: vec![3.0, 2.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![4.0],
            objectives: vec![4.0, 1.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
    ];

    let selected = select_next_generation(&mut pop, 2);
    assert_eq!(selected.len(), 2);
}

#[test]
fn test_select_with_truncation_crowding() {
    // Front 0 has 2 elements, but we want a population size of 3, requiring partial selection from Front 1
    let mut pop = vec![
        // Front 0
        Individual {
            features: vec![1.0],
            objectives: vec![1.0, 5.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![2.0],
            objectives: vec![5.0, 1.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        // Front 1 (dominated by Front 0, but needs crowding distance sorting)
        Individual {
            features: vec![3.0],
            objectives: vec![2.0, 6.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 1,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![4.0],
            objectives: vec![6.0, 2.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 1,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![5.0],
            objectives: vec![3.0, 4.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 1,
            crowding_distance: 0.0,
        },
    ];

    let selected = select_next_generation(&mut pop, 3);
    assert_eq!(selected.len(), 3);
}

#[test]
fn test_select_single_front_overflow() {
    // All individuals belong to rank 0, population size forces crowding distance truncation on a single front
    let mut pop = vec![
        Individual {
            features: vec![1.0],
            objectives: vec![1.0, 10.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![2.0],
            objectives: vec![2.0, 8.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![3.0],
            objectives: vec![5.0, 5.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![4.0],
            objectives: vec![8.0, 2.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
        Individual {
            features: vec![5.0],
            objectives: vec![10.0, 1.0],
            constraint_violations: vec![],
            feasible: true,
            rank: 0,
            crowding_distance: 0.0,
        },
    ];

    let selected = select_next_generation(&mut pop, 3);
    assert_eq!(selected.len(), 3);
}

#[test]
fn test_select_edge_case_single_element() {
    let mut pop = vec![Individual {
        features: vec![1.0],
        objectives: vec![1.0, 1.0],
        constraint_violations: vec![],
        feasible: true,
        rank: 0,
        crowding_distance: 0.0,
    }];

    let selected = select_next_generation(&mut pop, 1);
    assert_eq!(selected.len(), 1);
}
