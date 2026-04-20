use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Individual {
    pub features: Vec<f64>,
    pub objectives: Vec<f64>,
    pub rank: usize,
    pub crowding_distance: f64,
    pub feasible: bool,
    pub constraint_violations: Vec<f64>,
}

impl Individual {
    /// Constructor used throughout operators and evolution
    pub fn new(features: Vec<f64>) -> Self {
        Self {
            features,
            objectives: Vec::new(),
            rank: 0,
            crowding_distance: 0.0,
            feasible: true,
            constraint_violations: Vec::new(),
        }
    }

    /// Sum of positive constraint violations
    pub fn total_violation(&self) -> f64 {
        self.constraint_violations.iter().map(|&v| v.max(0.0)).sum()
    }

    /// Classic NSGA-II dominance check (boolean)
    pub fn dominates(&self, other: &Individual) -> bool {
        // Feasibility rules
        match (self.feasible, other.feasible) {
            (true, false) => return true,
            (false, true) => return false,
            (false, false) => {
                return self.total_violation() < other.total_violation();
            }
            (true, true) => {}
        }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomRelation {
    IDominatesJ,
    JDominatesI,
    None,
}

impl Individual {
    /// Tri-state dominance relation used by the optimized sorter
    pub fn dominance_relation(&self, other: &Individual) -> DomRelation {
        // Feasibility first
        match (self.feasible, other.feasible) {
            (true, false) => return DomRelation::IDominatesJ,
            (false, true) => return DomRelation::JDominatesI,
            (false, false) => {
                let a = self.total_violation();
                let b = other.total_violation();
                return if a < b {
                    DomRelation::IDominatesJ
                } else if b < a {
                    DomRelation::JDominatesI
                } else {
                    DomRelation::None
                };
            }
            (true, true) => {}
        }

        // Objective dominance
        let mut better = false;
        let mut worse = false;

        for (a, b) in self.objectives.iter().zip(&other.objectives) {
            if a < b {
                better = true;
            } else if a > b {
                worse = true;
            }
        }

        match (better, worse) {
            (true, false) => DomRelation::IDominatesJ,
            (false, true) => DomRelation::JDominatesI,
            _ => DomRelation::None,
        }
    }
}

/// Crowding operator used during selection
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
