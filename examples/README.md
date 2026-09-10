**`schaffer_optimization.rs`**

* **What it does:** Runs a quick, basic bi-objective optimization using the classic **Schaffer N.1** test problem. Perfect for verifying your setup and seeing the default API syntax in action.
* **Run command:** `cargo run --example schaffer_optimization`

**`zdt1_optimization.rs`**

* **What it does:** Tackles a higher-dimensional continuous optimization problem using **ZDT1** (30 decision variables) and computes the strict 2D hypervolume history along the way.
* **Run command:** `cargo run --example zdt1_optimization`

**`custom_problem.rs`**

* **What it does:** Walks through building and solving a completely **custom problem** from scratch by implementing the `Problem` trait for your own objective functions and variable ranges.
* **Run command:** `cargo run --example custom_problem`

**`constrained_problem.rs`**

* **What it does:** Demonstrates **constraint handling** in action by enforcing a strict geometric boundary condition ($x_0 + x_1 \ge 3.0$) and filtering out infeasible solutions automatically during sorting.
* **Run command:** `cargo run --example constrained_problem`

**`early_stopping.rs`**

* **What it does:** Shows how to configure **convergence-based early stopping**, using a rolling hypervolume window to automatically halt the evolution loop once optimization progress plateaus.
* **Run command:** `cargo run --example early_stopping`

* **`metrics_tracking.rs`**

* **What it does:** Demonstrates how to pass a known true Pareto front using `.with_true_front()` and evaluate optimization accuracy dynamically via **IGD** and **GD** history vectors.
* **Run command:** `cargo run --example metrics_tracking`
