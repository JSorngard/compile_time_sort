use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn bench_sort_i32(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<i32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_i32", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable()),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_f32(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<f32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_f32", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable_by(|a, b| a.total_cmp(b))),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_u8(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_u8", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable()),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_bool(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<bool> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_bool", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable()),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_u128(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<u128> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_u128", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable()),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_f64(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_f64", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(slice.sort_unstable_by(|a, b| a.total_cmp(b))),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    bench_sort_i32,
    bench_sort_f32,
    bench_sort_f64,
    bench_sort_u8,
    bench_sort_bool,
    bench_sort_u128
);
criterion_main!(benches);
