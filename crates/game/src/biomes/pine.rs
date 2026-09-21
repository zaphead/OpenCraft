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
            if biome_at(seed, x, z) != Biome::Pine { continue; }
            let y = ground(seed, x, z);
            if get(chunk, x, y - 1, z) != blocks::GRASS { continue; }
            let h = hash(seed, x, z);
            if h % 15 == 0 {
                put(chunk, x, y - 1, z, blocks::DIRT);
                for dy in 0..8 { put(chunk, x, y + dy, z, blocks::PINE_LOG); }
                for dy in 3..9 {
                    let r = if dy > 6 { 1 } else { 2 };
                    for dx in -r..=r {
                        for dz in -r..=r {
                            if dx == 0 && dz == 0 { continue; }
                            put(chunk, x + dx, y + dy, z + dz, blocks::PINE_LEAVES);
                        }
                    }
                }
            } else if h % 5 == 0 {
                put(chunk, x, y, z, blocks::TUFT);
            }
        }
    }
}
