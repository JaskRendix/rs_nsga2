use rs_nsga2::metrics::hypervolume_2d;

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

    let evo = Evolution::new(Schaffer, 50, 10).with_reference_point(vec![100.0, 100.0]);
    let result = evo.evolve();
    assert_eq!(result.hypervolume_history.len(), 10);
}

#[test]
fn test_hypervolume_history_non_decreasing() {
    use rs_nsga2::evolve::Evolution;
    use rs_nsga2::problem::Schaffer;

    let evo = Evolution::new(Schaffer, 100, 30).with_reference_point(vec![10.0, 10.0]);
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

    let evo = Evolution::new(Schaffer, 50, 5);
    let result = evo.evolve();
    for hv in &result.hypervolume_history {
        assert!(hv.is_nan());
    }
}
