use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 17, Biome::Glow);
    if !near(chunk, x, z, 6) { return; }
    let y = ground(seed, x, z);
    for dx in -3i32..=3 {
        for dz in -3i32..=3 {
            if dx * dx + dz * dz > 8 { continue; }
            for dy in 0..8 {
                put(chunk, x + dx, y - dy, z + dz, blocks::AIR);
            }
            put(chunk, x + dx, y - 8, z + dz, blocks::WATER);
        }
    }
    for dy in 1..7 {
        put(chunk, x + 2, y - dy, z, blocks::GLOWVINE);
        put(chunk, x - 2, y - dy, z + 1, blocks::GLOWVINE);
    }
}
