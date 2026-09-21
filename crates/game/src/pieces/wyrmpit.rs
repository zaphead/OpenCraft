use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::cell_center(Biome::Ash);
    if !near(chunk, x, z, 8) { return; }
    let y = ground(seed, x, z);
    for dx in -5i32..=5 {
        for dz in -5i32..=5 {
            if dx.abs() == 5 || dz.abs() == 5 {
                for dy in 0..4 {
                    put(chunk, x + dx, y + dy, z + dz, blocks::BASALT);
                }
            } else {
                put(chunk, x + dx, y - 1, z + dz, blocks::LAVA);
                put(chunk, x + dx, y, z + dz, blocks::LAVA);
            }
        }
    }
}
