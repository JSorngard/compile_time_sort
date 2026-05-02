use compile_time_sort::{
    sort_bool_slice, sort_f32_slice, sort_f64_slice, sort_i32_slice, sort_u128_slice, sort_u8_slice,
};
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn bench_sort_i32(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<i32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_i32_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_i32_slice(&mut slice)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_f32(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<f32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_f32_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_f32_slice(&mut slice)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_u8(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_u8_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_u8_slice(&mut slice)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_bool(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<bool> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_bool_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_bool_slice(&mut slice)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_u128(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<u128> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_u128_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_u128_slice(&mut slice)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sort_f64(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("sort_f64_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| black_box(sort_f64_slice(&mut slice)),
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
