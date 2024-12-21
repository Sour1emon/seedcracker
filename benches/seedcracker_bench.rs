use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::Rng;

use seedcracker::check_seed;
use seedcracker::random::chunkrand::ChunkRand;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("check_seed", |b| {
        b.iter_batched(
            || (rng.gen::<u64>(), rng.gen::<i16>(), rng.gen::<i16>()),
            |(seed, chunk_x, chunk_z)| check_seed(black_box(seed), chunk_x as i32, chunk_z as i32),
            BatchSize::SmallInput,
        )
    });
    let mut group = c.benchmark_group("Next Int Bound");
    group.bench_function("next_seed", |b| {
        let mut rand = ChunkRand::default();
        b.iter(|| {
            black_box(rand.next_seed());
        });
    });
    group.bench_function("next_seed_fast", |b| {
        let mut rand = ChunkRand::default();
        b.iter(|| {
            black_box(rand.next_seed_fast());
        });
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
