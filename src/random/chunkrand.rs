use std::ops::{Deref, DerefMut};

use crate::MC_VERSION;
use crate::random::jrand::JRand;
use crate::random::mcversion::V1_13;
use crate::random::mth::MASK_48;
use crate::random::seeds::{PositionSeed, RegionSeed};

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

    pub fn set_population_seed(&mut self, world_seed: i64, x: i32, z: i32) -> i64 {
        self.set_seed(world_seed, true);
        let a: i64;
        let b: i64;

        if MC_VERSION.is_older_than(&V1_13) {
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
    ) -> i64 {
        self.set_decorator_seed(population_seed, index + 10000 * step)
    }

    pub fn set_decorator_seed(&mut self, population_seed: i64, salt: i32) -> i64 {
        if MC_VERSION.is_older_than(&V1_13) {
            panic!("Unsupported version: {}", MC_VERSION)
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
    ) -> i64 {
        let population_seed = self.set_population_seed(world_seed, block_x, block_z);
        self.set_decorator_seed_with_index(population_seed, index, step)
    }

    pub fn set_decorator_seed_block_salt(
        &mut self,
        world_seed: i64,
        block_x: i32,
        block_z: i32,
        salt: i32,
    ) -> i64 {
        let population_seed = self.set_population_seed(world_seed, block_x, block_z);
        self.set_decorator_seed(population_seed, salt)
    }

    pub fn set_carver_seed(&mut self, world_seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
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
    ) -> i64 {
        let seed = region_x as i64 * RegionSeed::A
            + region_z as i64 * RegionSeed::B
            + world_seed
            + salt as i64;
        self.set_seed(seed, true);
        seed & MASK_48
    }

    pub fn set_weak_seed(&mut self, world_seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
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

    pub fn set_slime_seed(&mut self, world_seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
        self.set_slime_seed_scramble(world_seed, chunk_x, chunk_z, 987234911)
    }

    pub fn set_position_seed(&mut self, x: i32, y: i32, z: i32) -> i64 {
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
