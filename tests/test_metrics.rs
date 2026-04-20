use rs_nsga2::metrics::hypervolume_2d;
use rs_nsga2::metrics::igd;

#[test]
fn test_hypervolume_single_point() {
    let front = vec![vec![1.0, 1.0]];
    let reference = vec![3.0, 3.0];
    // rectangle: width=2, height=2 => 4.0
    assert!((hypervolume_2d(&front, &reference) - 4.0).abs() < 1e-10);
}

#[test]
fn test_hypervolume_two_points() {
    let front = vec![vec![1.0, 3.0], vec![3.0, 1.0]];
    let reference = vec![4.0, 4.0];
    // point (1,3): width=2, height=1 => 2.0
    // point (3,1): width=1, height=3 => 3.0
    // total: 5.0
    assert!((hypervolume_2d(&front, &reference) - 5.0).abs() < 1e-10);
}

#[test]
fn test_hypervolume_empty_front() {
    let front: Vec<Vec<f64>> = vec![];
    let reference = vec![2.0, 2.0];
    assert_eq!(hypervolume_2d(&front, &reference), 0.0);
}

#[test]
fn test_hypervolume_history_length() {
    use rs_nsga2::evolve::Evolution;
    use rs_nsga2::problem::Schaffer;

    let evo = Evolution::new(Schaffer::default(), 50, 10).with_reference_point(vec![100.0, 100.0]);
    let result = evo.evolve();
    assert_eq!(result.hypervolume_history.len(), 10);
}

#[test]
fn test_hypervolume_history_non_decreasing() {
    use rs_nsga2::evolve::Evolution;
    use rs_nsga2::problem::Schaffer;

    let evo = Evolution::new(Schaffer::default(), 100, 30).with_reference_point(vec![10.0, 10.0]);
    let result = evo.evolve();
    let hv = &result.hypervolume_history;
    let early: f64 = hv[..5].iter().sum::<f64>() / 5.0;
    let late: f64 = hv[25..].iter().sum::<f64>() / 5.0;
    assert!(late >= early, "Hypervolume should improve over generations");
}

#[test]
fn test_hypervolume_nan_when_no_reference_point() {
    use rs_nsga2::evolve::Evolution;
    use rs_nsga2::problem::Schaffer;

    let evo = Evolution::new(Schaffer::default(), 50, 5);
    let result = evo.evolve();
    for hv in &result.hypervolume_history {
        assert!(f64::is_nan(*hv));
    }
}

#[test]
#[should_panic(expected = "Reference point must be dominated by all front points")]
fn test_hypervolume_reference_not_dominating_panics() {
    let front = vec![vec![5.0, 1.0]];
    let reference = vec![3.0, 3.0]; // invalid: 5.0 > 3.0
    hypervolume_2d(&front, &reference);
}

#[test]
#[should_panic(expected = "Each point must have 2 objectives")]
fn test_hypervolume_wrong_dimension_panics() {
    let front = vec![vec![1.0, 2.0, 3.0]];
    let reference = vec![5.0, 5.0];
    hypervolume_2d(&front, &reference);
}

#[test]
fn test_hypervolume_ordering_invariant() {
    let front1 = vec![vec![1.0, 3.0], vec![3.0, 1.0]];
    let front2 = vec![vec![3.0, 1.0], vec![1.0, 3.0]];
    let reference = vec![4.0, 4.0];

    let hv1 = hypervolume_2d(&front1, &reference);
    let hv2 = hypervolume_2d(&front2, &reference);

    assert!((hv1 - hv2).abs() < 1e-12);
}

#[test]
fn test_igd_simple() {
    let true_front = vec![vec![0.0, 0.0]];
    let obtained = vec![vec![3.0, 4.0]];

    let igd_val = igd(&true_front, &obtained);
    assert!((igd_val - 5.0).abs() < 1e-12);
}

#[test]
fn test_igd_multi_point() {
    let true_front = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
    let obtained = vec![vec![0.0, 1.0], vec![1.0, 0.0]];

    // distances:
    // (0,0) -> min(1,1) = 1
    // (1,1) -> min(1,1) = 1
    // IGD = (1+1)/2 = 1
    let igd_val = igd(&true_front, &obtained);
    assert!((igd_val - 1.0).abs() < 1e-12);
}

#[test]
fn test_igd_empty_returns_nan() {
    let true_front = vec![vec![1.0, 1.0]];
    let obtained: Vec<Vec<f64>> = vec![];

    assert!(igd(&true_front, &obtained).is_nan());
    assert!(igd(&[], &true_front).is_nan());
}
