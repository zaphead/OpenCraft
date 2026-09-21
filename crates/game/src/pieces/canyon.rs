use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 9, Biome::Mesa);
    if !near(chunk, x, z, 10) { return; }
    let y = ground(seed, x, z);
    for dx in -8i32..=8 {
        for dz in -1i32..=1 {
            for dy in 0..12 {
                put(chunk, x + dx, y - dy, z + dz, blocks::AIR);
            }
        }
    }
}
