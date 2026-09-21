use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = (12, 48);
    if !near(chunk, x, z, 8) { return; }
    for dx in -5i32..=5 {
        for dz in -5i32..=5 {
            if dx * dx + dz * dz > 22 { continue; }
            for y in 8..16 {
                put(chunk, x + dx, y, z + dz, blocks::AIR);
            }
            put(chunk, x + dx, 7, z + dz, blocks::STONE);
            if hash(seed, x + dx, z + dz) % 3 == 0 {
                put(chunk, x + dx, 8, z + dz, blocks::CRYSTAL);
            }
        }
    }
}
