use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 5, Biome::Oak);
    if !near(chunk, x, z, 5) { return; }
    let y = ground(seed, x, z);
    for i in 0..8 {
        let a = i as f32 * 0.78;
        let dx = (a.cos() * 3.0) as i32;
        let dz = (a.sin() * 3.0) as i32;
        let gy = ground(seed, x + dx, z + dz);
        put(chunk, x + dx, gy, z + dz, blocks::COBBLE);
    }
    put(chunk, x, y, z, blocks::CHEST);
}
