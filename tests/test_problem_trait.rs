use rs_nsga2::problem::Problem;

struct MockProblem;

impl Problem for MockProblem {
    fn num_variables(&self) -> usize {
        3
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        static RANGES: [(f64, f64); 3] = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        &RANGES
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0] + x[1], x[2] * 2.0]
    }
}

#[test]
fn problem_num_variables_works() {
    let p = MockProblem;
    assert_eq!(p.num_variables(), 3);
}

#[test]
fn problem_num_objectives_works() {
    let p = MockProblem;
    assert_eq!(p.num_objectives(), 2);
}

#[test]
fn problem_variable_ranges_works() {
    let p = MockProblem;
    let ranges = p.variable_ranges();
    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0], (0.0, 1.0));
}

#[test]
fn problem_calculate_objectives_works() {
    let p = MockProblem;
    let out = p.calculate_objectives(&[0.2, 0.3, 0.4]);
    assert_eq!(out, vec![0.5, 0.8]);
}

#[test]
fn problem_default_constraint_violations_is_empty() {
    let p = MockProblem;
    let cv = p.constraint_violations(&[0.1, 0.2, 0.3]);
    assert!(cv.is_empty());
}

#[test]
fn problem_default_is_feasible_is_true() {
    let p = MockProblem;
    assert!(p.is_feasible(&[0.1, 0.2, 0.3]));
}

#[test]
fn problem_in_place_objectives_matches_allocate() {
    let p = MockProblem;
    let x = [0.2, 0.3, 0.4];

    let expected = p.calculate_objectives(&x);

    let mut out = vec![0.0; p.num_objectives()];
    p.calculate_objectives_in_place(&x, &mut out);

    assert_eq!(out, expected);
}

#[test]
fn problem_in_place_constraint_violations_matches_allocate() {
    let p = MockProblem;
    let x = [0.2, 0.3, 0.4];

    let expected = p.constraint_violations(&x);

    let mut out = vec![0.0; expected.len()];
    p.constraint_violations_in_place(&x, &mut out);

    assert_eq!(out, expected);
}

#[test]
fn problem_default_name_is_correct() {
    let p = MockProblem;
    assert_eq!(p.name(), "UnnamedProblem");
}

#[test]
fn problem_default_description_is_correct() {
    let p = MockProblem;
    assert_eq!(p.description(), "No description provided.");
}

#[test]
fn problem_default_initial_solution_is_none() {
    let p = MockProblem;
    assert!(p.initial_solution().is_none());
}

#[test]
fn problem_default_repair_solution_does_nothing() {
    let p = MockProblem;
    let mut x = [10.0, -5.0, 2.0];
    p.repair_solution(&mut x);
    assert_eq!(x, [10.0, -5.0, 2.0]);
}

#[test]
fn problem_default_custom_dominance_is_none() {
    let p = MockProblem;
    let a = [1.0, 2.0];
    let b = [2.0, 1.0];
    let cv = [];
    assert!(p.custom_dominance(&a, &b, &cv, &cv).is_none());
}
