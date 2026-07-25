#[cfg(test)]
mod tests {
    use rs_nsga2::problem::{Problem, Schaffer};

    #[test]
    fn schaffer_num_variables_is_one() {
        let p = Schaffer::default();
        assert_eq!(p.num_variables(), 1);
    }

    #[test]
    fn schaffer_num_objectives_is_two() {
        let p = Schaffer::default();
        assert_eq!(p.num_objectives(), 2);
    }

    #[test]
    fn schaffer_variable_ranges_are_correct() {
        let p = Schaffer::default();
        let ranges = p.variable_ranges();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], (-55.0, 55.0));
    }

    #[test]
    fn schaffer_objectives_match_definition() {
        let p = Schaffer::default();

        let xs: [f64; 4] = [0.0, 1.0, -3.5, 10.0];

        for &x in &xs {
            let out = p.calculate_objectives(&[x]);

            let expected_f1 = x.powi(2);
            let expected_f2 = (x - 2.0).powi(2);

            assert_eq!(out.len(), 2);
            assert!((out[0] - expected_f1).abs() < 1e-12);
            assert!((out[1] - expected_f2).abs() < 1e-12);
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn schaffer_panics_on_wrong_input_length() {
        let p = Schaffer::default();
        let _ = p.calculate_objectives(&[]);
    }

    #[test]
    fn schaffer_objectives_length_matches_num_objectives() {
        let p = Schaffer::default();
        let out = p.calculate_objectives(&[1.0]);
        assert_eq!(out.len(), p.num_objectives());
    }

    #[test]
    fn schaffer_name_and_description_are_correct() {
        let p = Schaffer::default();
        assert_eq!(p.name(), "Schaffer");
        assert!(p.description().contains("benchmark"));
    }

    #[test]
    fn schaffer_in_place_matches_allocate() {
        let p = Schaffer::default();
        let x = [3.7];

        let expected = p.calculate_objectives(&x);

        let mut out = vec![0.0; p.num_objectives()];
        p.calculate_objectives_in_place(&x, &mut out);

        assert_eq!(out, expected);
    }

    #[test]
    fn schaffer_repair_solution_clamps_values() {
        let p = Schaffer::default();
        let mut x = [-100.0]; // outside [-55, 55]

        p.repair_solution(&mut x);

        assert!(x[0] >= -55.0 && x[0] <= 55.0);
    }
}
