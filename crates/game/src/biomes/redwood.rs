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
            if biome_at(seed, x, z) != Biome::Redwood { continue; }
            let y = ground(seed, x, z);
            let h = hash(seed, x, z);
            if h % 29 == 0 && get(chunk, x, y - 1, z) == blocks::GRASS {
                for dy in 0..18 {
                    for dx in -2i32..=2 {
                        for dz in -2i32..=2 {
                            let rim = dx.abs() == 2 || dz.abs() == 2;
                            if rim {
                                put(chunk, x + dx, y + dy, z + dz, blocks::REDWOOD_LOG);
                            }
                        }
                    }
                }
                for dx in -3i32..=3 {
                    for dz in -3i32..=3 {
                        if dx.abs() + dz.abs() > 5 { continue; }
                        put(chunk, x + dx, y + 17, z + dz, blocks::REDWOOD_LEAVES);
                        put(chunk, x + dx, y + 18, z + dz, blocks::REDWOOD_LEAVES);
                    }
                }
            } else if h % 4 == 0 && get(chunk, x, y - 1, z) == blocks::GRASS {
                put(chunk, x, y, z, blocks::TUFT);
            }
        }
    }
}
