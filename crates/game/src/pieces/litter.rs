use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 14, Biome::Oak);
    if !near(chunk, x, z, 8) { return; }
    let y = ground(seed, x, z);
    for dx in 0..4 {
        put(chunk, x + dx, y, z, blocks::LOG);
    }
    let (bx, bz) = crate::biome::anchor(seed, 15, Biome::Meadow);
    if near(chunk, bx, bz, 3) {
        let by = ground(seed, bx, bz);
        put(chunk, bx, by, bz, blocks::COBBLE);
        put(chunk, bx + 1, by, bz, blocks::COBBLE);
        put(chunk, bx, by + 1, bz, blocks::STONE);
    }
    let (rx, rz) = crate::biome::anchor(seed, 16, Biome::Dune);
    if near(chunk, rx, rz, 4) {
        let ry = ground(seed, rx, rz);
        for dx in -2i32..=2 {
            put(chunk, rx + dx, ry, rz, blocks::STONE);
            if dx.abs() == 2 { put(chunk, rx + dx, ry + 1, rz, blocks::STONE); }
        }
    }
}
