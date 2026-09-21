use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 6, Biome::Redwood);
    if !near(chunk, x, z, 6) { return; }
    let y = ground(seed, x, z);
    for dy in 0..8 {
        put(chunk, x - 3, y + dy, z, blocks::STONE);
        put(chunk, x + 3, y + dy, z, blocks::STONE);
    }
    for dx in -3i32..=3 {
        put(chunk, x + dx, y + 8, z, blocks::STONE);
        put(chunk, x + dx, y + 9, z, blocks::STONE);
    }
}
