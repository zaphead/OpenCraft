use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = (36, 14);
    if !near(chunk, x, z, 18) { return; }
    let y = ground(seed, x, z);
    for dy in 0..(100 - y) {
        put(chunk, x, y + dy, z, blocks::BEAN);
        if dy % 3 == 0 { put(chunk, x + 1, y + dy, z, blocks::BEAN); }
    }
    let top = 104;
    for dx in -6i32..=8 {
        for dz in -4i32..=4 {
            put(chunk, x + dx, top, z + dz, blocks::GRASS);
            put(chunk, x + dx, top - 1, z + dz, blocks::DIRT);
        }
    }
    // Waterfall of still water off the east lip.
    for i in 0..8 {
        put(chunk, x + 9, top - i, z, blocks::WATER);
    }
    let _ = hash(seed, x, z);
}
