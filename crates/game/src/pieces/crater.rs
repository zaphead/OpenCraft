use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 10, Biome::Ash);
    if !near(chunk, x, z, 8) { return; }
    let y = ground(seed, x, z);
    for dx in -5i32..=5 {
        for dz in -5i32..=5 {
            let d = dx * dx + dz * dz;
            if d > 25 { continue; }
            let dig = 4 - d / 8;
            for dy in 0..dig {
                put(chunk, x + dx, y - dy, z + dz, if d < 4 { blocks::LAVA } else { blocks::AIR });
            }
        }
    }
}
