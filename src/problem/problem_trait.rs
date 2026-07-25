pub trait Problem: Send + Sync {
    //
    // ─────────────────────────────────────────────
    // Core NSGA‑II requirements
    // ─────────────────────────────────────────────
    //

    fn num_variables(&self) -> usize;
    fn num_objectives(&self) -> usize;

    /// Variable bounds for each decision variable.
    fn variable_ranges(&self) -> &[(f64, f64)];

    /// Compute objective values for a given feature vector.
    fn calculate_objectives(&self, features: &[f64]) -> Vec<f64>;

    //
    // ─────────────────────────────────────────────
    // Constraints (optional)
    // ─────────────────────────────────────────────
    //

    /// Constraint violations (<= 0 means satisfied).
    fn constraint_violations(&self, _features: &[f64]) -> Vec<f64> {
        vec![]
    }

    /// Feasibility check based on constraint violations.
    fn is_feasible(&self, features: &[f64]) -> bool {
        self.constraint_violations(features)
            .iter()
            .all(|&v| v <= 0.0)
    }

    //
    // ─────────────────────────────────────────────
    // Performance‑oriented extensions
    // ─────────────────────────────────────────────
    //

    /// In‑place objective computation (avoids Vec allocation).
    /// Default implementation falls back to allocate‑based version.
    fn calculate_objectives_in_place(&self, features: &[f64], out: &mut [f64]) {
        let vals = self.calculate_objectives(features);
        out.copy_from_slice(&vals);
    }

    /// In‑place constraint computation.
    fn constraint_violations_in_place(&self, features: &[f64], out: &mut [f64]) {
        let vals = self.constraint_violations(features);
        out.copy_from_slice(&vals);
    }

    //
    // ─────────────────────────────────────────────
    // Expressiveness & ergonomics
    // ─────────────────────────────────────────────
    //

    /// Human‑readable name for logging, debugging, benchmarking.
    fn name(&self) -> &'static str {
        "UnnamedProblem"
    }

    /// Optional description for documentation or CLI tools.
    fn description(&self) -> &'static str {
        "No description provided."
    }

    //
    // ─────────────────────────────────────────────
    // Solution initialization & repair
    // ─────────────────────────────────────────────
    //

    /// Provide a problem‑specific initial solution (instead of random).
    fn initial_solution(&self) -> Option<Vec<f64>> {
        None
    }

    /// Repair invalid solutions (e.g., clamp, project, normalize).
    /// Default: no repair.
    fn repair_solution(&self, _features: &mut [f64]) {
        // Default: do nothing
    }

    //
    // ─────────────────────────────────────────────
    // Optional custom dominance (advanced)
    // ─────────────────────────────────────────────
    //

    /// Override Pareto dominance (e.g., epsilon‑dominance).
    /// Default: use standard NSGA‑II dominance.
    fn custom_dominance(
        &self,
        _a_obj: &[f64],
        _b_obj: &[f64],
        _a_cv: &[f64],
        _b_cv: &[f64],
    ) -> Option<std::cmp::Ordering> {
        None
    }
}
