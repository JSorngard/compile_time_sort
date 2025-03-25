use compile_time_sort::{
    sort_bool_slice, sort_i128_slice, sort_i32_slice, sort_u8_slice, sort_usize_slice,
};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const MAX_LEN: usize = 1_000;

fn bench_std_sort(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("std_on_i32", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<i32>>()
            },
            |data| black_box(data.sort_unstable()),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
fn bench_sort_bool_slice(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("bool", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
            },
            |data| black_box(sort_bool_slice(data)),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
fn bench_sort_u8_slice(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("u8", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
            },
            |data| black_box(sort_u8_slice(data)),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
fn bench_sort_i32_slice(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("i32", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
            },
            |data| black_box(sort_i32_slice(data)),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
fn bench_sort_usize_slice(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("usize", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
            },
            |data| black_box(sort_usize_slice(data)),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
fn bench_sort_i128_slice(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    c.bench_function("i128", |b| {
        b.iter_batched_ref(
            || {
                (0..rng.gen_range(0..MAX_LEN))
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
            },
            |data| black_box(sort_i128_slice(data)),
            BatchSize::SmallInput,
        )
    });
}

#[rustversion::since(1.83.0)]
criterion_group!(
    benches,
    bench_sort_bool_slice,
    bench_sort_u8_slice,
    bench_sort_i32_slice,
    bench_sort_usize_slice,
    bench_sort_i128_slice
);

criterion_group!(control, bench_std_sort);

#[rustversion::since(1.83.0)]
criterion_main!(control, benches);

#[rustversion::not(since(1.83.0))]
criterion_main!(control);
