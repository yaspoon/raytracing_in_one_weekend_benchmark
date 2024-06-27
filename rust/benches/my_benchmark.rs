use criterion::{criterion_group, criterion_main, Criterion};
use raytracer_in_one_weekend_benchmark::raytrace;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("raytrace", |b| b.iter(|| raytrace()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
