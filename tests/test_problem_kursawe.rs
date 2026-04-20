use rs_nsga2::problem::{Kursawe, Problem};

#[test]
fn kursawe_num_variables_is_three() {
    let p = Kursawe::default();
    assert_eq!(p.num_variables(), 3);
}

#[test]
fn kursawe_num_objectives_is_two() {
    let p = Kursawe::default();
    assert_eq!(p.num_objectives(), 2);
}

#[test]
fn kursawe_variable_ranges_are_correct() {
    let p = Kursawe::default();
    let ranges = p.variable_ranges();

    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0], (-5.0, 5.0));
    assert_eq!(ranges[1], (-5.0, 5.0));
    assert_eq!(ranges[2], (-5.0, 5.0));
}

#[test]
fn kursawe_objectives_length_matches_num_objectives() {
    let p = Kursawe::default();
    let out = p.calculate_objectives(&[0.0, 1.0, -1.0]);
    assert_eq!(out.len(), p.num_objectives());
}

#[test]
fn kursawe_objectives_are_finite_for_sample_points() {
    let p = Kursawe::default();

    let samples = [[0.0, 0.0, 0.0], [1.0, -1.0, 2.0], [-3.5, 4.2, -0.7]];

    for x in samples {
        let out = p.calculate_objectives(&x);
        assert!(out[0].is_finite());
        assert!(out[1].is_finite());
    }
}

#[test]
#[should_panic]
fn kursawe_panics_on_wrong_input_length() {
    let p = Kursawe::default();
    let _ = p.calculate_objectives(&[1.0, 2.0]); // only 2 vars
}
