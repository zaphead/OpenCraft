use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 2, Biome::Swamp);
    if !near(chunk, x, z, 6) { return; }
    let y = ground(seed, x, z).max(12);
    for dx in -3i32..=3 {
        for dz in -3i32..=3 {
            for dy in 0..5 {
                let edge = dx.abs() == 3 || dz.abs() == 3 || dy == 0 || dy == 4;
                let id = if edge { blocks::STONE } else { blocks::AIR };
                put(chunk, x + dx, y + dy, z + dz, id);
            }
        }
    }
    put(chunk, x, y + 1, z, blocks::CHEST);
    put(chunk, x + 2, y + 1, z, blocks::SPAWNER);
    put(chunk, x - 2, y + 1, z, blocks::VAULT);
}
