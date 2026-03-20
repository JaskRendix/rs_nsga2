# NSGA‑II Rust‑Core

A Rust implementation of the NSGA‑II multi‑objective evolutionary algorithm. The crate provides non‑dominated sorting, crowding distance, simulated binary crossover, polynomial mutation, and tournament selection. Objective evaluation runs in parallel through Rayon.

---

## Features

- Non‑dominated sorting and crowding distance
- Simulated binary crossover with bounds clamping
- Polynomial mutation with per‑gene probability
- Parallel objective evaluation via Rayon
- Support for any number of objectives and dimensions
- Custom problem interface through the `Problem` trait

---

## Usage

The crate exposes four main modules:

- `problem` — the `Problem` trait and built‑in problems
- `evolve` — the NSGA‑II engine
- `sort` — non‑dominated sorting and crowding distance
- `data` — core data structures

### Example: built‑in Schaffer problem
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    let problem = Schaffer;
    let engine = Evolution::new(problem, 100, 500);
    let front = engine.evolve();
    for ind in front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

### Example: custom problem
```rust
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct MyProblem;

impl Problem for MyProblem {
    fn num_variables(&self) -> usize {
        2
    }
    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 1.0), (0.0, 1.0)]
    }
    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0] + x[1], (x[0] - 1.0).powi(2) + (x[1] - 1.0).powi(2)]
    }
}

fn main() {
    let engine = Evolution::new(MyProblem, 200, 300);
    let front = engine.evolve();
    for ind in front {
        println!("{:?} -> {:?}", ind.features, ind.objectives);
    }
}
```

---

## Algorithm

Each generation applies binary tournament selection, SBX crossover, and polynomial mutation to produce offspring. Parent and offspring populations are merged, then reduced to the next generation through non‑dominated sorting and crowding distance ranking.

---

## Original authors (Python version)

- Pham Ngo Gia Bao
- Tram Loi Quan
- Quan Thanh Tho
- Akhil Garg

---

## Rust port

- Giorgio