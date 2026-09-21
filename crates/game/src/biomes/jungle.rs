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
            if biome_at(seed, x, z) != Biome::Jungle { continue; }
            let y = ground(seed, x, z);
            if get(chunk, x, y - 1, z) != blocks::GRASS { continue; }
            let h = hash(seed, x, z);
            if h % 17 == 0 {
                put(chunk, x, y - 1, z, blocks::DIRT);
                for dy in 0..10 { put(chunk, x, y + dy, z, blocks::LOG); }
                for dx in -2i32..=2 {
                    for dz in -2i32..=2 {
                        put(chunk, x + dx, y + 9, z + dz, blocks::LEAVES);
                        put(chunk, x + dx, y + 10, z + dz, blocks::LEAVES);
                    }
                }
            } else if h % 3 != 2 {
                let n = 1 + (h % 3) as i32;
                for dy in 0..n { put(chunk, x, y + dy, z, blocks::FLOWER); }
            }
        }
    }
}
