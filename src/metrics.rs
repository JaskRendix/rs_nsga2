use std::cmp::Ordering;

//
// STRICT HYPERVOLUME (NSGA-II STANDARD)
//
pub fn hypervolume_2d_strict(front: &[Vec<f64>], reference: &[f64]) -> f64 {
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

    // Deterministic sort with tie-break
    points.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
    });

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

//
// AUTO‑ADJUSTING HYPERVOLUME
//
pub fn hypervolume_2d_auto(front: &[Vec<f64>], reference: &[f64]) -> f64 {
    assert_eq!(reference.len(), 2, "Reference point must have 2 objectives");

    if front.is_empty() {
        return 0.0;
    }

    // Filter invalid points
    let mut points: Vec<(f64, f64)> = front
        .iter()
        .filter(|p| p.len() == 2 && p[0].is_finite() && p[1].is_finite())
        .map(|p| (p[0], p[1]))
        .collect();

    if points.is_empty() {
        return 0.0;
    }

    let mut ref0 = reference[0];
    let mut ref1 = reference[1];

    for &(x, y) in &points {
        if x >= ref0 {
            ref0 = x + 1.0;
        }
        if y >= ref1 {
            ref1 = y + 1.0;
        }
    }

    // Deterministic sort
    points.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
    });

    let mut hv = 0.0;
    for i in 0..points.len() {
        let width = if i + 1 < points.len() {
            points[i + 1].0 - points[i].0
        } else {
            ref0 - points[i].0
        };

        let height = ref1 - points[i].1;

        hv += width.max(0.0) * height.max(0.0);
    }

    hv
}

//
// DEFAULT ALIAS
//
pub fn hypervolume_2d(front: &[Vec<f64>], reference: &[f64]) -> f64 {
    hypervolume_2d_strict(front, reference)
}

//
// IGD — EXACT EUCLIDEAN DISTANCE
//
pub fn igd(true_front: &[Vec<f64>], obtained_front: &[Vec<f64>]) -> f64 {
    if true_front.is_empty() || obtained_front.is_empty() {
        return f64::NAN;
    }

    let mut sum_dist = 0.0;

    for target in true_front {
        let min_dist = obtained_front
            .iter()
            .map(|point| {
                target
                    .iter()
                    .zip(point.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .fold(f64::INFINITY, |a, b| a.min(b));

        sum_dist += min_dist;
    }

    sum_dist / true_front.len() as f64
}
