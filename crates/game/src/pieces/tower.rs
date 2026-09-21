use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 4, Biome::Moor);
    if !near(chunk, x, z, 4) { return; }
    let y = ground(seed, x, z);
    for dy in 0..12 {
        for dx in -2i32..=2 {
            for dz in -2i32..=2 {
                let rim = dx.abs() == 2 || dz.abs() == 2;
                if rim || dy == 0 || dy == 11 {
                    put(chunk, x + dx, y + dy, z + dz, blocks::COBBLE);
                }
            }
        }
    }
    put(chunk, x, y + 1, z + 2, blocks::AIR);
    put(chunk, x, y + 2, z + 2, blocks::AIR);
}
