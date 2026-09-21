use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 13, Biome::Moor);
    if !near(chunk, x, z, 12) { return; }
    let y = ground(seed, x, z);
    for i in -8..=8 {
        let gy = ground(seed, x + i, z - 8);
        put(chunk, x + i, gy, z - 8, blocks::THORN);
        put(chunk, x + i, gy + 1, z - 8, blocks::THORN);
        let gy2 = ground(seed, x - 8, z + i);
        put(chunk, x - 8, gy2, z + i, blocks::THORN);
    }
    // A small maze just south of the wall.
    for dz in 0..7 {
        for dx in 0..7 {
            let wall = dz % 2 == 0 && dx != 3 || dx == 0 || dx == 6;
            if wall {
                let gy = ground(seed, x + dx, z + dz);
                put(chunk, x + dx, gy, z + dz, blocks::THORN);
                put(chunk, x + dx, gy + 1, z + dz, blocks::THORN);
            }
        }
    }
}
