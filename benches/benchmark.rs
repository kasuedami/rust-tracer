use criterion::{criterion_group, criterion_main, Criterion};

mod fov;
mod many_spheres;

fn benchmark_fov(c: &mut Criterion) {
    let world = fov::create_world();
    let fov_start = 5;
    let fov_step = 5;
    let fov_end = 100;

    for fov in (fov_start..fov_end).step_by(fov_step) {
        c.bench_function(&format!("fov{fov}"), |b| {
            b.iter(|| fov::create_camera(fov as f64, &world))
        });
    }
}

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
    targets = benchmark_fov, benchmark_many_spheres
}

criterion_group! {
    name = many_spheres;
    config = bench_settings();
    targets = benchmark_many_spheres
}

criterion_main!(benches);
