use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Individual {
    pub features: Vec<f64>,
    pub objectives: Vec<f64>,
    pub rank: usize,
    pub crowding_distance: f64,
}

impl Individual {
    pub fn new(features: Vec<f64>) -> Self {
        Self {
            features,
            objectives: Vec::new(),
            rank: 0,
            crowding_distance: 0.0,
        }
    }

    pub fn dominates(&self, other: &Individual) -> bool {
        let mut better_in_one = false;
        for (a, b) in self.objectives.iter().zip(other.objectives.iter()) {
            if a > b {
                return false;
            }
            if a < b {
                better_in_one = true;
            }
        }
        better_in_one
    }
}

/// Orders individuals for selection: `Less` means `a` is preferred over `b`.
/// Preference is determined first by rank (lower is better), then by
/// crowding distance (higher is better, to preserve diversity).
pub fn crowding_operator(a: &Individual, b: &Individual) -> Ordering {
    if a.rank < b.rank {
        Ordering::Less
    } else if b.rank < a.rank {
        Ordering::Greater
    } else {
        b.crowding_distance
            .partial_cmp(&a.crowding_distance)
            .unwrap_or(Ordering::Equal)
    }
}
