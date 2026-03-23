use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rs_nsga2::data::Individual;
use rs_nsga2::sort::Nsga2Sorter;

fn make_population(n: usize) -> Vec<Individual> {
    (0..n)
        .map(|i| {
            let x = i as f64 / n as f64;
            let mut ind = Individual::new(vec![x]);
            ind.objectives = vec![x, 1.0 - x];
            ind
        })
        .collect()
}

fn bench_fast_nondominated_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_nondominated_sort");

    for &n in &[100, 500, 1000, 2000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || make_population(n),
                |mut pop| Nsga2Sorter::fast_nondominated_sort(&mut pop),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_crowding_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("crowding_distance");

    for &n in &[100, 500, 1000, 2000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || make_population(n),
                |mut pop| Nsga2Sorter::calculate_crowding_distance(&mut pop),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_fast_nondominated_sort,
    bench_crowding_distance
);
criterion_main!(benches);
