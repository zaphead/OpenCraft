use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = (26, -10);
    if !near(chunk, x, z, 2) { return; }
    let y = ground(seed, x, z);
    for dy in 0..5 {
        put(chunk, x, y + dy, z, blocks::STONE);
        put(chunk, x + 1, y + dy, z, blocks::COBBLE);
    }
    put(chunk, x, y + 5, z, blocks::CRYSTAL);
}
