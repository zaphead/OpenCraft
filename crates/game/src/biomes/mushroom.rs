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
            if biome_at(seed, x, z) != Biome::Mushroom { continue; }
            let y = ground(seed, x, z);
            let h = hash(seed, x, z);
            if h % 18 == 0 {
                for dy in 0..5 { put(chunk, x, y + dy, z, blocks::MUSH_STEM); }
                for dx in -2i32..=2 {
                    for dz in -2i32..=2 {
                        put(chunk, x + dx, y + 5, z + dz, blocks::MUSH_CAP);
                    }
                }
            }
        }
    }
}
