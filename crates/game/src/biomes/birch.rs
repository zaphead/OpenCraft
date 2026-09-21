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
            if biome_at(seed, x, z) != Biome::Birch { continue; }
            let y = ground(seed, x, z);
            if get(chunk, x, y - 1, z) != blocks::GRASS { continue; }
            let h = hash(seed, x, z);
            if h % 13 == 0 {
                put(chunk, x, y - 1, z, blocks::DIRT);
                for dy in 0..6 { put(chunk, x, y + dy, z, blocks::BIRCH_LOG); }
                for dx in -2i32..=2 {
                    for dz in -2i32..=2 {
                        for dy in 4..7 {
                            if dx == 0 && dz == 0 { continue; }
                            put(chunk, x + dx, y + dy, z + dz, blocks::BIRCH_LEAVES);
                        }
                    }
                }
            } else if h % 4 == 0 {
                put(chunk, x, y, z, blocks::FLOWER);
            } else if h % 3 == 0 {
                put(chunk, x, y, z, blocks::TUFT);
            }
        }
    }
}
