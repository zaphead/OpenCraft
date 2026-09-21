use world::Chunk;

use crate::biome::Biome;
use crate::blocks;
use crate::stamp::{ground, near, put};
use crate::worldgen::hash;

pub fn stamp(chunk: &mut Chunk, _seed: i64) {
    let colors = [
        blocks::RAIN_R, blocks::RAIN_O, blocks::RAIN_Y, blocks::RAIN_G,
        blocks::RAIN_B, blocks::RAIN_I, blocks::RAIN_V,
    ];
    let y = 104;
    let z = 14;
    for i in 0..16 {
        let id = colors[i % colors.len()];
        put(chunk, 44 + i as i32, y, z, id);
        put(chunk, 44 + i as i32, y, z + 1, id);
    }
}
