# NSGA‑II Rust Core

A Rust implementation of the NSGA‑II multi‑objective evolutionary algorithm.  
The crate provides the core components needed to build deterministic or stochastic evolutionary workflows.

---

## Features

- non‑dominated sorting (parallel dominance matrix, tri‑state dominance)
- crowding distance (NaN‑safe, stable ordering)
- simulated binary crossover (SBX)
- polynomial mutation
- tournament selection
- constraint handling
- strict and auto hypervolume
- IGD (Inverted Generational Distance)
- per‑generation Pareto snapshots
- early stopping
- reproducible runs through optional RNG seeding

Objective evaluation and dominance checks run in parallel through Rayon.  
Algorithm parameters use a builder‑style API.

---

## Modules

- `problem` — `Problem` trait and built‑in problems  
- `evolve` — NSGA‑II engine and `RunResult`  
- `sort` — non‑dominated sorting and crowding distance  
- `data` — individuals, dominance logic, feasibility rules, crowding operator  
- `metrics` — strict HV, auto HV, IGD  

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
        vec![2.0 - x[0] - x[1]] // x0 + x1 >= 2
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
}
```

---

## Sorting

### Non‑dominated sorting
- tri‑state dominance (`IDominatesJ`, `JDominatesI`, `None`)
- parallel dominance matrix
- sequential front extraction
- feasibility rules:
  - feasible dominates infeasible
  - among infeasible: lower total violation dominates

### Crowding distance
- stable, NaN‑safe sorting per objective
- infinite distance for boundary individuals
- finite‑difference distance for interior individuals

---

## Metrics

### Strict hypervolume

```rust
use rs_nsga2::metrics::hypervolume_2d_strict;
```

Strict HV requires the reference point to dominate the front.

### Auto hypervolume

```rust
use rs_nsga2::metrics::hypervolume_2d_auto;
```

Auto HV expands the reference point when needed.

### IGD

```rust
use rs_nsga2::metrics::igd;
```

Computes the average distance from the true front to the obtained front.

---

## Algorithm

Each generation:

1. tournament selection  
2. SBX crossover  
3. polynomial mutation  
4. parallel objective evaluation  
5. merge parents and offspring  
6. non‑dominated sorting  
7. crowding‑distance truncation  

Feasible solutions dominate infeasible ones.  
Among infeasible solutions, lower total violation dominates.

---

## RunResult

| Field | Description |
|---|---|
| `pareto_front` | final Pareto front |
| `history` | per‑generation Pareto snapshots |
| `hypervolume_history` | HV per generation (`NaN` if no reference point) |
| `igd_history` | IGD per generation (`NaN` if no true front) |
| `generations_completed` | number of generations executed |

---

## Benchmarks

The crate includes:

- full NSGA‑II loop  
- sorting only  
- strict vs auto hypervolume  
- IGD microbench  

Run all:

```
cargo bench
```

Run a specific benchmark:

```
cargo bench --bench igd
cargo bench --bench sorting
cargo bench --bench evolution
```

Reports are written to `target/criterion/`.

---

## Original authors (Python version)

- Pham Ngo Gia Bao  
- Tram Loi Quan  
- Quan Thanh Tho  
- Akhil Garg  

## Rust port

- Giorgio
