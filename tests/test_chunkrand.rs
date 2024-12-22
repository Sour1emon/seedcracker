use seedcracker::random::chunkrand::ChunkRand;
use seedcracker::random::jrand::JRand;

#[test]
fn test_terrain_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(chunk_rand.set_terrain_seed(54, 34), 22979680526842);
}

#[test]
fn test_population_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_population_seed(5235023985902385, 234, 256),
        72830530974919
    );
}

#[test]
fn test_decorator_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_decorator_seed_block_salt(571437852345, 10, 25, 5),
        234602471937219
    );
}

#[test]
fn test_carver_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_carver_seed(5714378552345, 55, 6),
        57433805167245
    );
}

#[test]
fn test_region_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_region_seed(5714378552345, 55, 6, 53415),
        25314788610166
    );
}

#[test]
fn test_weak_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_weak_seed(7432895798243759, 34, 3),
        114546403766701
    );
}

#[test]
fn test_slime_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_slime_seed_scramble(7432895798243759, 34, 3, 5555),
        114548116864026
    )
}

#[test]
fn test_position_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_position_seed(555, 54, 1345),
        27536523298125
    )
}

#[test]
fn test_base_stone_seed() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(
        chunk_rand.set_base_stone_seed(4327857727348957239, 399, 45, 399),
        515723718367701839
    )
}

#[test]
fn test_random_item() {
    let rand = JRand::new(4506419895);
    let mut chunk_rand: ChunkRand = rand.into();
    assert_eq!(chunk_rand.get_random(&[1, 2, 3, 4, 5]), 1)
}
