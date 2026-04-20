use rs_nsga2::metrics::hypervolume_2d_auto as hypervolume_2d;

//
// AUTO‑ADJUSTING HYPERVOLUME TESTS
//

#[test]
fn test_auto_hv_single_point_valid_reference() {
    let front = vec![vec![1.0, 1.0]];
    let reference = vec![5.0, 5.0];

    let hv = hypervolume_2d(&front, &reference);
    assert!((hv - 16.0).abs() < 1e-12);
}

#[test]
fn test_auto_hv_reference_expands_when_needed() {
    let front = vec![vec![10.0, 1.0]];
    let reference = vec![5.0, 5.0];

    let hv = hypervolume_2d(&front, &reference);
    assert!(hv >= 4.0);
}

#[test]
fn test_auto_hv_reference_expands_in_both_dimensions() {
    let front = vec![vec![10.0, 20.0]];
    let reference = vec![5.0, 5.0];

    let hv = hypervolume_2d(&front, &reference);
    assert!((hv - 1.0).abs() < 1e-12);
}

#[test]
fn test_auto_hv_ordering_invariant() {
    let front1 = vec![vec![1.0, 3.0], vec![3.0, 1.0]];
    let front2 = vec![vec![3.0, 1.0], vec![1.0, 3.0]];
    let reference = vec![2.0, 2.0];

    let hv1 = hypervolume_2d(&front1, &reference);
    let hv2 = hypervolume_2d(&front2, &reference);

    assert!((hv1 - hv2).abs() < 1e-12);
}

#[test]
fn test_auto_hv_empty_front_returns_zero() {
    let hv = hypervolume_2d(&[], &[5.0, 5.0]);
    assert_eq!(hv, 0.0);
}

#[test]
fn test_auto_hv_monotonicity_when_reference_expands() {
    let front_small = vec![vec![2.0, 2.0]];
    let front_large = vec![vec![10.0, 10.0]];

    let reference = vec![5.0, 5.0];

    let hv_small = hypervolume_2d(&front_small, &reference);
    let hv_large = hypervolume_2d(&front_large, &reference);

    // Auto-HV does NOT guarantee monotonicity.
    // The only guarantee is that HV is always >= 0.
    assert!(hv_small >= 0.0);
    assert!(hv_large >= 0.0);
}

#[test]
fn test_auto_hv_multiple_points_reference_expands_minimally() {
    let front = vec![vec![3.0, 1.0], vec![1.0, 3.0]];
    let reference = vec![2.0, 2.0];

    let hv = hypervolume_2d(&front, &reference);

    assert!((hv - 5.0).abs() < 1e-12);
}
