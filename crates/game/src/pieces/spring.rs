use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 11, Biome::Frost);
    if !near(chunk, x, z, 4) { return; }
    let y = ground(seed, x, z);
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx * dx + dz * dz > 4 { continue; }
            put(chunk, x + dx, y - 1, z + dz, blocks::STONE);
            put(chunk, x + dx, y, z + dz, blocks::SPRING);
        }
    }
}
