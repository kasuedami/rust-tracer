use criterion::{criterion_group, criterion_main, Criterion};

mod many_spheres;

fn benchmark_many_spheres(c: &mut Criterion) {
    let world = many_spheres::create_world();
    c.bench_function("many_spheres", |b| {
        b.iter(|| many_spheres::create_camera(&world))
    });
}

fn bench_settings() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = bench_settings();
    targets = benchmark_many_spheres
}

criterion_group! {
    name = many_spheres;
    config = bench_settings();
    targets = benchmark_many_spheres
}

criterion_main!(benches);
