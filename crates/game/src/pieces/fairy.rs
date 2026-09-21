use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 1, Biome::Oak);
    if !near(chunk, x, z, 6) { return; }
    let y = ground(seed, x, z);
    for i in 0..12 {
        let a = i as f32 * 0.52;
        let dx = (a.cos() * 4.0) as i32;
        let dz = (a.sin() * 4.0) as i32;
        let gy = ground(seed, x + dx, z + dz);
        put(chunk, x + dx, gy, z + dz, blocks::FLOWER);
    }
    put(chunk, x, y, z, blocks::KING_CAP);
    put(chunk, x, y + 1, z, blocks::KING_CAP);
}
