use world::Chunk;

use crate::biome::{biome_at, Biome};
use crate::blocks;
use crate::stamp::{get, ground, put};
use crate::worldgen::hash;

pub fn grow(chunk: &mut Chunk, seed: i64) {
    let (ax, az) = crate::biome::anchor(seed, 7, Biome::Dune);
    if !crate::stamp::near(chunk, ax, az, 6) { return; }
    let y = ground(seed, ax, az);
    for dx in -4i32..=4 {
        for dz in -4i32..=4 {
            if dx * dx + dz * dz > 16 { continue; }
            let yy = ground(seed, ax + dx, az + dz);
            put(chunk, ax + dx, yy - 1, az + dz, blocks::SAND);
            if dx * dx + dz * dz < 9 {
                put(chunk, ax + dx, yy, az + dz, blocks::WATER);
            }
        }
    }
    let _ = y;
}
