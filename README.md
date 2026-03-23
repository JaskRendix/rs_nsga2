# NSGA‑II Rust‑Core

A Rust implementation of the NSGA‑II multi‑objective evolutionary algorithm. The crate provides
non‑dominated sorting, crowding distance, simulated binary crossover, polynomial mutation, and
tournament selection. Objective evaluation and dominance comparisons run in parallel through Rayon.
Constraint handling is supported via the `Problem` trait, where infeasible solutions are
automatically ranked below feasible ones based on total violation. Per‑generation Pareto front
snapshots, hypervolume indicator tracking, and convergence‑based early stopping are available
through `RunResult`. Algorithm parameters are configured through a builder‑style API.

---

## Modules

- `problem` — the `Problem` trait and built‑in problems
- `evolve` — the NSGA‑II engine and `RunResult`
- `sort` — non‑dominated sorting and crowding distance
- `data` — core data structures
- `metrics` — hypervolume indicator

---

## Usage

### Built‑in Schaffer problem
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    let result = Evolution::new(Schaffer, 100, 500).evolve();

    for ind in &result.pareto_front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

### Custom problem
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct MyProblem;

impl Problem for MyProblem {
    fn num_variables(&self) -> usize { 2 }
    fn num_objectives(&self) -> usize { 2 }
    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 1.0), (0.0, 1.0)]
    }
    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0] + x[1], (x[0] - 1.0).powi(2) + (x[1] - 1.0).powi(2)]
    }
}

fn main() {
    let result = Evolution::new(MyProblem, 200, 300).evolve();

    for ind in &result.pareto_front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

### Constrained problem
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct ConstrainedProblem;

impl Problem for ConstrainedProblem {
    fn num_variables(&self) -> usize { 2 }
    fn num_objectives(&self) -> usize { 2 }
    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 5.0), (0.0, 5.0)]
    }
    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0], x[1]]
    }
    fn constraint_violations(&self, x: &[f64]) -> Vec<f64> {
        // x[0] + x[1] >= 2.0, encoded as 2.0 - x[0] - x[1] <= 0
        vec![2.0 - x[0] - x[1]]
    }
}

fn main() {
    let result = Evolution::new(ConstrainedProblem, 100, 200).evolve();

    for ind in &result.pareto_front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

### Hypervolume tracking and early stopping
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    let result = Evolution::new(Schaffer, 100, 500)
        .with_reference_point(vec![10.0, 10.0])
        .with_convergence_threshold(10, 0.001)
        .evolve();

    println!("Generations completed: {}", result.generations_completed);

    for (gen, hv) in result.hypervolume_history.iter().enumerate() {
        println!("Generation {}: hypervolume = {:.4}", gen + 1, hv);
    }

    for ind in &result.pareto_front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

### Hypervolume of an arbitrary front
```rust
use rs_nsga2::metrics::hypervolume_2d;

fn main() {
    let front = vec![
        vec![0.1, 3.9],
        vec![1.0, 1.0],
        vec![2.5, 0.5],
        vec![3.8, 0.1],
    ];
    let reference = vec![5.0, 5.0];
    let hv = hypervolume_2d(&front, &reference);
    println!("Hypervolume: {:.4}", hv);
}
```

### Custom algorithm parameters
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    let result = Evolution::new(Schaffer, 100, 300)
        .with_crossover_param(15.0)
        .with_mutation_param(10.0)
        .evolve();

    for ind in &result.pareto_front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

---

## Algorithm

Each generation applies binary tournament selection, SBX crossover, and polynomial mutation to
produce offspring. Parent and offspring populations are merged, then reduced to the next generation
through non‑dominated sorting and crowding distance ranking. Feasibility is tracked per individual
and incorporated into dominance comparisons — feasible solutions always dominate infeasible ones,
and among infeasible solutions the one with lower total constraint violation is preferred.

---

## RunResult

`evolve()` returns a `RunResult` containing:

| Field | Description |
|---|---|
| `pareto_front` | Final Pareto‑optimal solutions |
| `history` | Per‑generation Pareto front snapshots |
| `hypervolume_history` | Hypervolume indicator per generation (`NaN` if no reference point set) |
| `generations_completed` | Number of generations actually run (may be less than `num_generations` if early stopping fired) |

---

## Benchmarks

To measure sorting and evolution performance across population sizes:
```
cargo bench
```

To isolate the impact of parallelisation in `fast_nondominated_sort`:
```
RAYON_NUM_THREADS=1 cargo bench --bench sorting
cargo bench --bench sorting
```

HTML reports are written to `target/criterion/`.

---

## Original authors (Python version)

- Pham Ngo Gia Bao
- Tram Loi Quan
- Quan Thanh Tho
- Akhil Garg

---

## Rust port

- Giorgio