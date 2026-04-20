use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rs_nsga2::evolve::Evolution;
use rs_nsga2::metrics::{hypervolume_2d_auto, hypervolume_2d_strict, igd};
use rs_nsga2::problem::Problem;
use rs_nsga2::sort::Nsga2Sorter;

//
// Benchmark Problem
//

struct BenchProblem {
    ranges: [(f64, f64); 2],
}

impl BenchProblem {
    fn new() -> Self {
        Self {
            ranges: [(0.0, 1.0), (0.0, 1.0)],
        }
    }
}

impl Problem for BenchProblem {
    fn num_variables(&self) -> usize {
        2
    }

    fn num_objectives(&self) -> usize {
        2
    }

    fn variable_ranges(&self) -> &[(f64, f64)] {
        &self.ranges
    }

    fn calculate_objectives(&self, x: &[f64]) -> Vec<f64> {
        vec![x[0], x[1]]
    }
}

//
// 1. IGD‑only microbench
//
fn bench_igd_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("igd_only");
    group.sample_size(10);

    let true_front: Vec<Vec<f64>> = (0..200)
        .map(|i| {
            let x = i as f64 / 200.0;
            vec![x, 1.0 - x]
        })
        .collect();

    for &n in &[50, 100, 200, 500] {
        let obtained: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let x = i as f64 / n as f64;
                vec![x, 1.0 - x]
            })
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| igd(&true_front, &obtained));
        });
    }

    group.finish();
}

//
// 2. strict vs auto hypervolume
//
fn bench_hv_strict_vs_auto(c: &mut Criterion) {
    let mut group = c.benchmark_group("hypervolume_strict_vs_auto");
    group.sample_size(10);

    let reference = vec![2.0, 2.0];

    for &n in &[50, 100, 200, 500] {
        let front: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let x = i as f64 / n as f64;
                vec![x, 1.0 - x]
            })
            .collect();

        group.bench_with_input(BenchmarkId::new("strict", n), &n, |b, _| {
            b.iter(|| hypervolume_2d_strict(&front, &reference))
        });

        group.bench_with_input(BenchmarkId::new("auto", n), &n, |b, _| {
            b.iter(|| hypervolume_2d_auto(&front, &reference))
        });
    }

    group.finish();
}

//
// 3. sorting‑only benchmark
//
fn bench_sorting_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting_only");
    group.sample_size(10);

    for &n in &[50, 100, 200, 500] {
        let mut population = (0..n)
            .map(|i| {
                let x = i as f64 / n as f64;
                let mut ind = rs_nsga2::data::Individual::new(vec![x, 1.0 - x]);
                ind.objectives = vec![x, 1.0 - x];
                ind
            })
            .collect::<Vec<_>>();

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| {
                Nsga2Sorter::fast_nondominated_sort(&mut population);
            });
        });
    }

    group.finish();
}

//
// 4. evolve_with_igd (your original benchmark)
//
fn bench_evolve_with_igd(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolve_with_igd");
    group.sample_size(10);

    let true_front: Vec<Vec<f64>> = (0..100)
        .map(|i| {
            let x = i as f64 / 100.0;
            vec![x, 1.0 - x]
        })
        .collect();

    for &n in &[50, 100, 200, 500] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || Evolution::new(BenchProblem::new(), n, 20).with_true_front(true_front.clone()),
                |evo: Evolution<BenchProblem>| evo.evolve(),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_igd_only,
    bench_hv_strict_vs_auto,
    bench_sorting_only,
    bench_evolve_with_igd
);

criterion_main!(benches);
