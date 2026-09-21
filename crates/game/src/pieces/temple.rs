use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, seed: i64) {
    let (x, z) = crate::biome::anchor(seed, 3, Biome::Dune);
    if !near(chunk, x, z, 7) { return; }
    let y = ground(seed, x, z);
    for dx in -4i32..=4 {
        for dz in -4i32..=4 {
            for dy in 0..6 {
                let wall = dx.abs() == 4 || dz.abs() == 4 || dy == 0 || dy == 5;
                let door = dy < 3 && dx == 0 && dz == 4;
                let id = if door { blocks::AIR } else if wall { blocks::SAND } else { blocks::AIR };
                put(chunk, x + dx, y + dy, z + dz, if wall && !door { blocks::STONE } else { id });
            }
        }
    }
    put(chunk, x, y + 1, z, blocks::CHEST);
    put(chunk, x + 2, y + 1, z + 2, blocks::SPAWNER);
    put(chunk, x, y + 1, z - 2, blocks::CHEST);
}
