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
            if biome_at(seed, x, z) != Biome::Savanna { continue; }
            let y = ground(seed, x, z);
            if get(chunk, x, y - 1, z) != blocks::GRASS { continue; }
            let h = hash(seed, x, z);
            if h % 21 == 0 {
                for dx in 0..2 {
                    for dz in 0..2 {
                        for dy in 0..7 {
                            put(chunk, x + dx, y + dy, z + dz, blocks::BAOBAB_LOG);
                        }
                    }
                }
                for dx in -3i32..=4 {
                    for dz in -3i32..=4 {
                        if dx.abs() + dz.abs() > 6 { continue; }
                        put(chunk, x + dx, y + 7, z + dz, blocks::BAOBAB_LEAVES);
                        put(chunk, x + dx, y + 8, z + dz, blocks::BAOBAB_LEAVES);
                    }
                }
            } else if h % 4 == 0 {
                put(chunk, x, y, z, blocks::TUFT);
            }
        }
    }
}
