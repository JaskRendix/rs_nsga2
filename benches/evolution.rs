use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Problem;

struct BenchProblem;

impl Problem for BenchProblem {
    fn num_variables(&self) -> usize {
        2
    }
    fn num_objectives(&self) -> usize {
        2
    }
    fn variable_ranges(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 1.0), (0.0, 1.0)]
    }
    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0], x[1]]
    }
}

fn bench_evolve(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolve");

    // fewer iterations since each run is expensive
    group.sample_size(10);

    for &n in &[50, 100, 200, 500] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || Evolution::new(BenchProblem, n, 20),
                |evo| evo.evolve(),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_evolve_with_hypervolume(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolve_with_hypervolume");

    group.sample_size(10);

    for &n in &[50, 100, 200, 500] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || Evolution::new(BenchProblem, n, 20).with_reference_point(vec![2.0, 2.0]),
                |evo| evo.evolve(),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_evolve, bench_evolve_with_hypervolume);
criterion_main!(benches);
