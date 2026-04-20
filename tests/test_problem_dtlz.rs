use rs_nsga2::problem::{Problem, DTLZ1, DTLZ2, DTLZ3};

//
// DTLZ1
//
#[test]
fn dtlz1_num_variables_matches_constructor() {
    let p = DTLZ1::new(3, 7);
    assert_eq!(p.num_variables(), 7);
}

#[test]
fn dtlz1_num_objectives_matches_constructor() {
    let p = DTLZ1::new(5, 12);
    assert_eq!(p.num_objectives(), 5);
}

#[test]
fn dtlz1_variable_ranges_are_correct() {
    let p = DTLZ1::new(3, 6);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 6);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn dtlz1_objectives_length_matches_num_objectives() {
    let p = DTLZ1::new(3, 7);
    let out = p.calculate_objectives(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn dtlz1_objectives_are_finite_for_sample_points() {
    let p = DTLZ1::new(3, 7);

    let samples = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9, 0.3, 0.7, 0.1],
        [0.3, 0.7, 0.1, 0.4, 0.8, 0.2, 0.6],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        for v in out {
            assert!(v.is_finite());
        }
    }
}

#[test]
#[should_panic]
fn dtlz1_panics_on_wrong_input_length() {
    let p = DTLZ1::new(3, 7);
    let _ = p.calculate_objectives(&[0.1, 0.2]); // too short
}

//
// DTLZ2
//
#[test]
fn dtlz2_num_variables_matches_constructor() {
    let p = DTLZ2::new(3, 10);
    assert_eq!(p.num_variables(), 10);
}

#[test]
fn dtlz2_num_objectives_matches_constructor() {
    let p = DTLZ2::new(4, 12);
    assert_eq!(p.num_objectives(), 4);
}

#[test]
fn dtlz2_variable_ranges_are_correct() {
    let p = DTLZ2::new(3, 5);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 5);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn dtlz2_objectives_length_matches_num_objectives() {
    let p = DTLZ2::new(3, 7);
    let out = p.calculate_objectives(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn dtlz2_objectives_are_finite_for_sample_points() {
    let p = DTLZ2::new(3, 7);

    let samples = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9, 0.3, 0.7, 0.1],
        [0.3, 0.7, 0.1, 0.4, 0.8, 0.2, 0.6],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        for v in out {
            assert!(v.is_finite());
        }
    }
}

#[test]
#[should_panic]
fn dtlz2_panics_on_wrong_input_length() {
    let p = DTLZ2::new(3, 7);
    let _ = p.calculate_objectives(&[0.1, 0.2]); // too short
}

//
// DTLZ3
//
#[test]
fn dtlz3_num_variables_matches_constructor() {
    let p = DTLZ3::new(3, 9);
    assert_eq!(p.num_variables(), 9);
}

#[test]
fn dtlz3_num_objectives_matches_constructor() {
    let p = DTLZ3::new(5, 11);
    assert_eq!(p.num_objectives(), 5);
}

#[test]
fn dtlz3_variable_ranges_are_correct() {
    let p = DTLZ3::new(3, 6);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 6);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn dtlz3_objectives_length_matches_num_objectives() {
    let p = DTLZ3::new(3, 7);
    let out = p.calculate_objectives(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn dtlz3_objectives_are_finite_for_sample_points() {
    let p = DTLZ3::new(3, 7);

    let samples = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9, 0.3, 0.7, 0.1],
        [0.3, 0.7, 0.1, 0.4, 0.8, 0.2, 0.6],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        for v in out {
            assert!(v.is_finite());
        }
    }
}

#[test]
#[should_panic]
fn dtlz3_panics_on_wrong_input_length() {
    let p = DTLZ3::new(3, 7);
    let _ = p.calculate_objectives(&[0.1, 0.2]); // too short
}
