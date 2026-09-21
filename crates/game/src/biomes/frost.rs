use world::Chunk;

use crate::biome::{biome_at, Biome};
use crate::blocks;
use crate::stamp::{get, ground, put};
use crate::worldgen::hash;

pub fn grow(chunk: &mut Chunk, seed: i64) {
    let ox = chunk.pos.x * 16;
    let oz = chunk.pos.z * 16;
    for lz in 0..16 {
        for lx in 0..16 {
            let (x, z) = (ox + lx, oz + lz);
            if biome_at(seed, x, z) != Biome::Frost { continue; }
            let y = ground(seed, x, z);
            if hash(seed, x, z) % 6 == 0 {
                put(chunk, x, y, z, blocks::TUFT);
            }
        }
    }
}
