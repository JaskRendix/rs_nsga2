## **schaffer_optimization.rs**
**Purpose:** Runs a simple bi‑objective optimization using the classic **Schaffer N.1** benchmark. Useful for validating your setup and learning the default API syntax.  
**Run:** `cargo run --example schaffer_optimization`

---

## **zdt1_optimization.rs**
**Purpose:** Solves the higher‑dimensional **ZDT1** problem (30 variables) and records strict 2D hypervolume history throughout the run.  
**Run:** `cargo run --example zdt1_optimization`

---

## **custom_problem.rs**
**Purpose:** Shows how to build a fully **custom optimization problem** by implementing the `Problem` trait, defining your own objectives, and specifying variable bounds.  
**Run:** `cargo run --example custom_problem`

---

## **constrained_problem.rs**
**Purpose:** Demonstrates constraint handling with a geometric feasibility rule (`x₀ + x₁ ≥ 3.0`). Infeasible solutions are automatically filtered during sorting.  
**Run:** `cargo run --example constrained_problem`

---

## **early_stopping.rs**
**Purpose:** Implements convergence‑based **early stopping** using a rolling hypervolume window to halt evolution once progress plateaus.  
**Run:** `cargo run --example early_stopping`

---

## **metrics_tracking.rs**
**Purpose:** Tracks optimization accuracy by providing a known true Pareto front via `.with_true_front()` and computing **IGD** and **GD** metrics over time.  
**Run:** `cargo run --example metrics_tracking`
