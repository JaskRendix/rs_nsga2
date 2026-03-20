#[cfg(test)]
mod tests {
    use rs_nsga2::problem::{Problem, Schaffer};

    #[test]
    fn schaffer_num_variables_is_one() {
        let p = Schaffer;
        assert_eq!(p.num_variables(), 1);
    }

    #[test]
    fn schaffer_variable_ranges_are_correct() {
        let p = Schaffer;
        let ranges = p.variable_ranges();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], (-55.0, 55.0));
    }

    #[test]
    fn schaffer_objectives_match_definition() {
        let p = Schaffer;

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
        let p = Schaffer;
        let _ = p.calculate_objectives(&[]);
    }
}
