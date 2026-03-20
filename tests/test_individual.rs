use rs_nsga2::data::{crowding_operator, Individual};
use std::cmp::Ordering;

fn ind(obj1: f64, obj2: f64) -> Individual {
    let mut i = Individual::new(vec![]);
    i.objectives = vec![obj1, obj2];
    i
}

#[test]
fn test_dominates_strictly_better_in_all() {
    let a = ind(1.0, 1.0);
    let b = ind(2.0, 2.0);

    assert!(a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[test]
fn test_dominates_better_in_one_equal_in_other() {
    let a = ind(1.0, 2.0);
    let b = ind(1.0, 3.0);

    assert!(a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[test]
fn test_no_dominance_when_tradeoff() {
    let a = ind(1.0, 3.0);
    let b = ind(2.0, 2.0);

    assert!(!a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[test]
fn test_no_dominance_when_identical() {
    let a = ind(1.0, 2.0);
    let b = ind(1.0, 2.0);

    assert!(!a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[test]
fn test_dominance_multiple_objectives() {
    let mut a = Individual::new(vec![]);
    a.objectives = vec![1.0, 2.0, 3.0];

    let mut b = Individual::new(vec![]);
    b.objectives = vec![2.0, 2.0, 4.0];

    assert!(a.dominates(&b));
    assert!(!b.dominates(&a));
}

#[test]
fn test_crowding_operator_prefers_lower_rank() {
    let mut a = ind(1.0, 1.0);
    let mut b = ind(2.0, 2.0);

    a.rank = 0;
    b.rank = 1;

    assert_eq!(crowding_operator(&a, &b), Ordering::Less);
    assert_eq!(crowding_operator(&b, &a), Ordering::Greater);
}

#[test]
fn test_crowding_operator_prefers_higher_crowding_distance_when_same_rank() {
    let mut a = ind(1.0, 1.0);
    let mut b = ind(2.0, 2.0);

    a.rank = 1;
    b.rank = 1;

    a.crowding_distance = 5.0;
    b.crowding_distance = 2.0;

    assert_eq!(crowding_operator(&a, &b), Ordering::Less);
    assert_eq!(crowding_operator(&b, &a), Ordering::Greater);
}

#[test]
fn test_crowding_operator_equal_when_same_rank_and_distance() {
    let mut a = ind(1.0, 1.0);
    let mut b = ind(2.0, 2.0);

    a.rank = 1;
    b.rank = 1;

    a.crowding_distance = 3.0;
    b.crowding_distance = 3.0;

    assert_eq!(crowding_operator(&a, &b), Ordering::Equal);
    assert_eq!(crowding_operator(&b, &a), Ordering::Equal);
}

#[test]
fn test_crowding_operator_handles_nan_safely() {
    let mut a = ind(1.0, 1.0);
    let mut b = ind(2.0, 2.0);

    a.rank = 1;
    b.rank = 1;

    a.crowding_distance = f64::NAN;
    b.crowding_distance = 5.0;

    // unwrap_or(Ordering::Equal) means NaN comparisons default to Equal
    assert_eq!(crowding_operator(&a, &b), Ordering::Equal);
}
