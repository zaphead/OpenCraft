use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 8, Biome::Mesa);
    if !near(chunk, x, z, 8) { return; }
    let y = ground(seed, x, z);
    for dx in -4i32..=4 {
        for dz in -4i32..=4 {
            if dx.abs() + dz.abs() > 6 { continue; }
            for dy in 0..10 {
                let id = if (dy / 2) % 2 == 0 { blocks::CLAY } else { blocks::SAND };
                put(chunk, x + dx, y + dy, z + dz, id);
            }
            put(chunk, x + dx, y + 10, z + dz, blocks::SAND);
        }
    }
}
