use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 18, Biome::Moor);
    if !near(chunk, x, z, 6) { return; }
    let y = ground(seed, x, z);
    for i in 0..4 {
        let gy = ground(seed, x + i * 2, z);
        put(chunk, x + i * 2, gy, z, blocks::COBBLE);
        put(chunk, x + i * 2, gy + 1, z, blocks::STONE);
    }
    for i in 0..5 {
        let a = i as f32;
        let dx = (a * 1.3) as i32 - 2;
        let gy = ground(seed, x + dx, z + 4);
        put(chunk, x + dx, gy, z + 4, blocks::STONE);
        put(chunk, x + dx, gy + 1, z + 4, blocks::STONE);
        put(chunk, x + dx, gy + 2, z + 4, blocks::STONE);
    }
}
