use crate::random::jrand::JRand;
use crate::random::mcversion::{MCVersion, V1_13};
use crate::random::mth::MASK_48;
use crate::random::seeds::{PositionSeed, RegionSeed};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ChunkRand {
    jrand: JRand,
}

impl Deref for ChunkRand {
    type Target = JRand;

    fn deref(&self) -> &Self::Target {
        &self.jrand
    }
}

impl DerefMut for ChunkRand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.jrand
    }
}

impl Default for ChunkRand {
    fn default() -> Self {
        Self {
            jrand: JRand::new_scramble(0, false),
        }
    }
}

impl From<JRand> for ChunkRand {
    fn from(jrand: JRand) -> Self {
        Self { jrand }
    }
}

impl ChunkRand {
    pub const fn new(seed: i64) -> Self {
        Self {
            jrand: JRand::new(seed),
        }
    }

    pub const fn new_scramble(seed: i64, scramble: bool) -> Self {
        Self {
            jrand: JRand::new_scramble(seed, scramble),
        }
    }

    pub fn set_terrain_seed(&mut self, chunk_x: i32, chunk_z: i32) -> i64 {
        let seed = chunk_x as i64 * 341873128712 + chunk_z as i64 * 132897987541;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_population_seed(
        &mut self,
        world_seed: i64,
        x: i32,
        z: i32,
        version: MCVersion,
    ) -> i64 {
        self.set_seed(world_seed, true);
        let a: i64;
        let b: i64;

        if version.is_older_than(&V1_13) {
            a = (self.get_next_long() / 2) * 2 + 1;
            b = self.get_next_long() / 2 * 2 + 1;
        } else {
            let f = self.get_next_long();
            a = f | 1;
            let g = self.get_next_long();
            b = g | 1;
        }

        let f = x as i64 * a;
        let g = z as i64 * b;

        let seed = (f + g) ^ world_seed;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_decorator_seed_with_index(
        &mut self,
        population_seed: i64,
        index: i32,
        step: i32,
        version: MCVersion,
    ) -> i64 {
        self.set_decorator_seed(population_seed, index + 10000 * step, version)
    }

    pub fn set_decorator_seed(
        &mut self,
        population_seed: i64,
        salt: i32,
        version: MCVersion,
    ) -> i64 {
        if version.is_older_than(&V1_13) {
            panic!("Unsupported version: {}", version)
        }

        let seed = population_seed + salt as i64;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_decorator_seed_block(
        &mut self,
        world_seed: i64,
        block_x: i32,
        block_z: i32,
        index: i32,
        step: i32,
        version: MCVersion,
    ) -> i64 {
        let population_seed = self.set_population_seed(world_seed, block_x, block_z, version);
        self.set_decorator_seed_with_index(population_seed, index, step, version)
    }

    pub fn set_decorator_seed_block_salt(
        &mut self,
        world_seed: i64,
        block_x: i32,
        block_z: i32,
        salt: i32,
        version: MCVersion,
    ) -> i64 {
        let population_seed = self.set_population_seed(world_seed, block_x, block_z, version);
        self.set_decorator_seed(population_seed, salt, version)
    }

    pub fn set_carver_seed(
        &mut self,
        world_seed: i64,
        chunk_x: i32,
        chunk_z: i32,
        _version: MCVersion,
    ) -> i64 {
        self.set_seed(world_seed, true);
        let a = self.get_next_long();
        let b = self.get_next_long();
        let seed = (chunk_x as i64 * a) ^ (chunk_z as i64 * b) ^ world_seed;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_region_seed(
        &mut self,
        world_seed: i64,
        region_x: i32,
        region_z: i32,
        salt: i32,
        _version: MCVersion,
    ) -> i64 {
        let seed = region_x as i64 * RegionSeed::A
            + region_z as i64 * RegionSeed::B
            + world_seed
            + salt as i64;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_weak_seed(
        &mut self,
        world_seed: i64,
        chunk_x: i32,
        chunk_z: i32,
        _version: MCVersion,
    ) -> i64 {
        let sx = chunk_x >> 4;
        let sz = chunk_z >> 4;
        let seed = (sx ^ sz << 4) as i64 ^ world_seed;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_slime_seed_scramble(
        &mut self,
        world_seed: i64,
        chunk_x: i32,
        chunk_z: i32,
        scrambler: i64,
        _version: MCVersion,
    ) -> i64 {
        let seed = (world_seed
            + (chunk_x * chunk_x * 4987142) as i64
            + (chunk_x * 5947611) as i64
            + (chunk_z * chunk_z) as i64 * 4392871
            + (chunk_z * 389711) as i64)
            ^ scrambler;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_slime_seed(
        &mut self,
        world_seed: i64,
        chunk_x: i32,
        chunk_z: i32,
        _version: MCVersion,
    ) -> i64 {
        self.set_slime_seed_scramble(world_seed, chunk_x, chunk_z, 987234911, _version)
    }

    pub fn set_position_seed(&mut self, x: i32, y: i32, z: i32, _version: MCVersion) -> i64 {
        let seed = PositionSeed::get_position_seed(x, y, z);
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_base_stone_seed(&mut self, world_seed: i64, x: i32, y: i32, z: i32) -> i64 {
        self.set_seed(world_seed, true);
        let a = self.get_next_long();
        let b = self.get_next_long();
        let _c = self.get_next_long();
        let seed = (x as i64 * a) ^ (y as i64 * b) ^ (z as i64 * z as i64) ^ world_seed;
        self.set_seed(seed, true);
        seed
    }

    pub fn get_random<T: Clone>(&mut self, list: &[T]) -> T {
        list[self.get_next_int_bound(list.len() as i32) as usize].clone()
    }

    pub fn get_int(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            min
        } else {
            self.get_next_int_bound(max - min + 1) + min
        }
    }
}

mod tests {
    use crate::random::chunkrand::ChunkRand;
    use crate::random::jrand::JRand;
    use crate::random::mcversion::MCVersion;

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
            chunk_rand.set_population_seed(5235023985902385, 234, 256, *MCVersion::latest()),
            72830530974919
        );
    }

    #[test]
    fn test_decorator_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_decorator_seed_block_salt(571437852345, 10, 25, 5, *MCVersion::latest()),
            234602471937219
        );
    }

    #[test]
    fn test_carver_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_carver_seed(5714378552345, 55, 6, *MCVersion::latest()),
            57433805167245
        );
    }

    #[test]
    fn test_region_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_region_seed(5714378552345, 55, 6, 53415, *MCVersion::latest()),
            25314788610166
        );
    }

    #[test]
    fn test_weak_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_weak_seed(7432895798243759, 34, 3, *MCVersion::latest()),
            114546403766701
        );
    }

    #[test]
    fn test_slime_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_slime_seed_scramble(7432895798243759, 34, 3, 5555, *MCVersion::latest()),
            114548116864026
        )
    }

    #[test]
    fn test_position_seed() {
        let rand = JRand::new(4506419895);
        let mut chunk_rand: ChunkRand = rand.into();
        assert_eq!(
            chunk_rand.set_position_seed(555, 54, 1345, *MCVersion::latest()),
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
}
