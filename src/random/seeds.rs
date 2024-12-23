use std::sync::Mutex;

use crate::random::chunkrand::ChunkRand;

pub struct RegionSeed;

impl RegionSeed {
    pub const A: i64 = 341873128712;
    pub const B: i64 = 132897987541;

    pub const fn get_increment(d_region_x: i32, d_region_z: i32) -> i64 {
        Self::A * d_region_x as i64 + Self::B * d_region_z as i64
    }

    pub const fn convert_to_world_seed(region_seed: i64, region_x: i32, region_z: i32, salt: i32) -> i64 {
        Self::translate(region_seed, region_x, region_z) - salt as i64
    }

    pub const fn translate(region_seed: i64, d_region_x: i32, d_region_z: i32) -> i64 {
        region_seed - Self::get_increment(d_region_x, d_region_z)
    }

    pub const fn get_neighbor(region_seed: i64, d_region_x: i32, d_region_z: i32) -> i64 {
        region_seed + Self::get_increment(d_region_x, d_region_z)
    }

    pub const fn are_neighbors(
        region_seed1: i64,
        region_seed2: i64,
        d_region_x: i32,
        d_region_z: i32,
    ) -> bool {
        Self::are_neighbors_with_salt(region_seed1, region_seed2, 0, 0, d_region_x, d_region_z)
    }

    pub const fn are_neighbors_with_salt(
        region_seed1: i64,
        region_seed2: i64,
        salt1: i32,
        salt2: i32,
        d_region_x: i32,
        d_region_z: i32,
    ) -> bool {
        Self::get_increment(d_region_x, d_region_z)
            == (region_seed2 - salt2 as i64) - (region_seed1 - salt1 as i64)
    }
}

pub struct PositionSeed;

static INTERNAL: Mutex<ChunkRand> = Mutex::new(ChunkRand::new_scramble(0, false));

impl PositionSeed {
    pub const fn get_position_seed(x: i32, y: i32, z: i32) -> i64 {
        let mut i = (x * 3129871) as i64 ^ (z as i64 * 116129781) ^ y as i64;
        i = i * i * 42317861 + i * 11;
        i >> 16
    }

    pub fn get_base_stone_seed(world_seed: i64, x: i32, y: i32, z: i32) -> i64 {
        let mut chunkrand = INTERNAL.lock().unwrap();
        chunkrand.set_base_stone_seed(world_seed, x, y, z)
    }
}
