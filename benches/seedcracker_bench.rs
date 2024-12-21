use criterion::{BatchSize, black_box, Criterion, criterion_group, criterion_main};
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
    c.bench_function("rand_int", |b| {
        let mut rand = ChunkRand::default();
        b.iter(|| {
            rand.get_next_int();
        })
    })
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
