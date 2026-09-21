//! Block writes that stay inside one chunk. Structures call this; they do not
//! reach into neighbor chunks.

use world::Chunk;

use crate::worldgen::surface_y;

pub fn put(chunk: &mut Chunk, wx: i32, y: i32, wz: i32, id: u16) {
    if y < engine_core::MIN_Y || y >= engine_core::MAX_Y {
        return;
    }
    let lx = wx - chunk.pos.x * 16;
    let lz = wz - chunk.pos.z * 16;
    if (0..16).contains(&lx) && (0..16).contains(&lz) {
        chunk.set(lx as u32, y, lz as u32, id);
    }
}

pub fn get(chunk: &Chunk, wx: i32, y: i32, wz: i32) -> u16 {
    let lx = wx - chunk.pos.x * 16;
    let lz = wz - chunk.pos.z * 16;
    if !(0..16).contains(&lx) || !(0..16).contains(&lz) {
        return 0;
    }
    chunk.get(lx as u32, y, lz as u32)
}

pub fn near(chunk: &Chunk, wx: i32, wz: i32, r: i32) -> bool {
    let ox = chunk.pos.x * 16;
    let oz = chunk.pos.z * 16;
    let x0 = ox - r;
    let z0 = oz - r;
    let x1 = ox + 16 + r;
    let z1 = oz + 16 + r;
    wx >= x0 && wx < x1 && wz >= z0 && wz < z1
}

pub fn ground(seed: i64, x: i32, z: i32) -> i32 {
    surface_y(seed, x, z)
}
