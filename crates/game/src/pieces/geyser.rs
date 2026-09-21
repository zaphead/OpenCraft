use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 12, Biome::Ash);
    if !near(chunk, x, z, 2) { return; }
    let y = ground(seed, x, z);
    put(chunk, x, y, z, blocks::GEYSER);
    put(chunk, x, y - 1, z, blocks::LAVA);
}
