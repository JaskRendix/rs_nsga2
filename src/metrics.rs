pub fn hypervolume_2d(front: &[Vec<f64>], reference: &[f64]) -> f64 {
    assert_eq!(reference.len(), 2, "Reference point must have 2 objectives");

    let mut points: Vec<(f64, f64)> = front
        .iter()
        .map(|p| {
            assert_eq!(p.len(), 2, "Each point must have 2 objectives");
            assert!(
                p[0] < reference[0] && p[1] < reference[1],
                "Reference point must be dominated by all front points"
            );
            (p[0], p[1])
        })
        .collect();

    if points.is_empty() {
        return 0.0;
    }

    // sort by first objective ascending
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // sweep: each point contributes a rectangle from its f1 to the next
    // point's f1, and from its f2 down to the reference f2
    let mut hv = 0.0;
    for i in 0..points.len() {
        let width = if i + 1 < points.len() {
            points[i + 1].0 - points[i].0
        } else {
            reference[0] - points[i].0
        };
        let height = reference[1] - points[i].1;
        hv += width * height;
    }

    hv
}
