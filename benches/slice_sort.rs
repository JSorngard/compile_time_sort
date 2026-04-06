use compile_time_sort::{sort_f32_slice, sort_i32_slice, sort_u8_slice};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn bench_sort_i32(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([42; 32]);

    let data: Vec<i32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("std::sort_unstable_i32", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| slice.sort_unstable(),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("sort_i32_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| sort_i32_slice(&mut slice),
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
            |mut slice| slice.sort_unstable_by(|a, b| a.total_cmp(b)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("sort_f32_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| sort_f32_slice(&mut slice),
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
            |mut slice| slice.sort_unstable(),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("sort_u8_slice", |b| {
        b.iter_batched(
            || data.clone(),
            |mut slice| sort_u8_slice(&mut slice),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_sort_i32, bench_sort_f32, bench_sort_u8);
criterion_main!(benches);
