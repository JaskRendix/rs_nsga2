use rs_nsga2::problem::{Problem, ZDT1, ZDT2, ZDT3};

//
// ZDT1
//
#[test]
fn zdt1_num_variables_matches_constructor() {
    let p = ZDT1::new(30);
    assert_eq!(p.num_variables(), 30);
}

#[test]
fn zdt1_num_objectives_is_two() {
    let p = ZDT1::new(10);
    assert_eq!(p.num_objectives(), 2);
}

#[test]
fn zdt1_variable_ranges_are_correct() {
    let p = ZDT1::new(5);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 5);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn zdt1_objectives_length_matches_num_objectives() {
    let p = ZDT1::new(4);
    let out = p.calculate_objectives(&[0.1, 0.2, 0.3, 0.4]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn zdt1_objectives_are_finite_for_sample_points() {
    let p = ZDT1::new(4);

    let samples = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9],
        [0.3, 0.7, 0.1, 0.4],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        assert!(out[0].is_finite());
        assert!(out[1].is_finite());
    }
}

//
// ZDT2
//
#[test]
fn zdt2_num_variables_matches_constructor() {
    let p = ZDT2::new(20);
    assert_eq!(p.num_variables(), 20);
}

#[test]
fn zdt2_num_objectives_is_two() {
    let p = ZDT2::new(12);
    assert_eq!(p.num_objectives(), 2);
}

#[test]
fn zdt2_variable_ranges_are_correct() {
    let p = ZDT2::new(6);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 6);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn zdt2_objectives_length_matches_num_objectives() {
    let p = ZDT2::new(4);
    let out = p.calculate_objectives(&[0.2, 0.4, 0.6, 0.8]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn zdt2_objectives_are_finite_for_sample_points() {
    let p = ZDT2::new(4);

    let samples = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9],
        [0.3, 0.7, 0.1, 0.4],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        assert!(out[0].is_finite());
        assert!(out[1].is_finite());
    }
}

//
// ZDT3
//
#[test]
fn zdt3_num_variables_matches_constructor() {
    let p = ZDT3::new(10);
    assert_eq!(p.num_variables(), 10);
}

#[test]
fn zdt3_num_objectives_is_two() {
    let p = ZDT3::new(7);
    assert_eq!(p.num_objectives(), 2);
}

#[test]
fn zdt3_variable_ranges_are_correct() {
    let p = ZDT3::new(5);
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 5);
    for &(low, high) in ranges {
        assert_eq!(low, 0.0);
        assert_eq!(high, 1.0);
    }
}

#[test]
fn zdt3_objectives_length_matches_num_objectives() {
    let p = ZDT3::new(4);
    let out = p.calculate_objectives(&[0.1, 0.2, 0.3, 0.4]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn zdt3_objectives_are_finite_for_sample_points() {
    let p = ZDT3::new(4);

    let samples = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.5, 0.2, 0.9],
        [0.3, 0.7, 0.1, 0.4],
    ];

    for x in samples {
        let out = p.calculate_objectives(&x);
        assert!(out[0].is_finite());
        assert!(out[1].is_finite());
    }
}
