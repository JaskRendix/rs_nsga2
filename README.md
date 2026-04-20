# NSGA‑II Rust‑Core

A high‑performance Rust implementation of the NSGA‑II multi‑objective evolutionary algorithm.  
The crate provides:

- fast non‑dominated sorting  
- crowding distance  
- simulated binary crossover (SBX)  
- polynomial mutation  
- tournament selection  
- constraint handling  
- strict and auto‑adjusting hypervolume  
- IGD (Inverted Generational Distance)  
- per‑generation Pareto snapshots  
- early stopping  
- reproducible runs via optional RNG seeding  

Objective evaluation and dominance checks run in parallel through Rayon.  
Algorithm parameters are configured through a builder‑style API.

---

## Modules

- `problem` — the `Problem` trait and built‑in problems  
- `evolve` — NSGA‑II engine and `RunResult`  
- `sort` — non‑dominated sorting and crowding distance  
- `data` — core data structures  
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

## Metrics

### Strict hypervolume

```rust
use rs_nsga2::metrics::hypervolume_2d_strict;
```

Strict HV requires the reference point to dominate the front.

### Auto‑adjusting hypervolume

```rust
use rs_nsga2::metrics::hypervolume_2d_auto;
```

Auto HV expands the reference point minimally to avoid panics.

### IGD

```rust
use rs_nsga2::metrics::igd;

fn main() {
    let true_front = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
    let obtained = vec![vec![0.1, 0.9], vec![0.8, 0.2]];
    let d = igd(&true_front, &obtained);
    println!("IGD: {}", d);
}
```

---

## Algorithm

Each generation:

1. binary tournament selection  
2. SBX crossover  
3. polynomial mutation  
4. parallel objective evaluation  
5. merge parents + offspring  
6. fast non‑dominated sort  
7. crowding‑distance truncation  

Feasible solutions dominate infeasible ones.  
Among infeasible solutions, lower total violation is preferred.

---

## RunResult

`evolve()` returns:

| Field | Description |
|---|---|
| `pareto_front` | Final Pareto‑optimal solutions |
| `history` | Per‑generation Pareto front snapshots |
| `hypervolume_history` | Strict HV per generation (`NaN` if no reference point) |
| `igd_history` | IGD per generation (`NaN` if no true front) |
| `generations_completed` | Actual number of generations run |

---

## Benchmarks

The crate includes:

- **evolution** (full NSGA‑II loop)  
- **sorting‑only** (fast non‑dominated sort)  
- **strict vs auto hypervolume**  
- **IGD‑only microbench**  

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

HTML reports are written to `target/criterion/`.

---

## Original authors (Python version)

- Pham Ngo Gia Bao  
- Tram Loi Quan  
- Quan Thanh Tho  
- Akhil Garg  

## Rust port

- Giorgio
