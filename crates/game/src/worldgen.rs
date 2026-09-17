use engine_core::{ChunkPos, MIN_Y, SECTION_EDGE};
use world::Chunk;

use crate::blocks;

pub fn generate_chunk(seed: i64, pos: ChunkPos) -> Chunk {
    let mut chunk = Chunk::empty(pos);
    let ox = pos.x * SECTION_EDGE;
    let oz = pos.z * SECTION_EDGE;
    for lz in 0..16 {
        for lx in 0..16 {
            let wx = ox + lx;
            let wz = oz + lz;
            let h = surface_y(seed, wx, wz);
            for y in MIN_Y..h {
                let id = if y == h - 1 {
                    blocks::GRASS
                } else if y >= h - 4 {
                    blocks::DIRT
                } else {
                    blocks::STONE
                };
                chunk.set(lx as u32, y, lz as u32, id);
            }
            if tree_here(seed, wx, wz) && h > MIN_Y + 4 && h + 5 < engine_core::MAX_Y {
                let top = chunk.get(lx as u32, h - 1, lz as u32);
                if top == blocks::GRASS {
                    chunk.set(lx as u32, h - 1, lz as u32, blocks::DIRT);
                    let th = 4 + (hash(seed, wx, wz) % 3) as i32;
                    for dy in 0..th {
                        chunk.set(lx as u32, h + dy, lz as u32, blocks::LOG);
                    }
                }
            }
        }
    }
    chunk
}

pub fn surface_y(seed: i64, x: i32, z: i32) -> i32 {
    let n = fbm(seed, x as f32, z as f32);
    let h = 40.0 + n * 30.0;
    (h as i32).clamp(MIN_Y + 8, engine_core::MAX_Y - 16)
}

fn tree_here(seed: i64, x: i32, z: i32) -> bool {
    hash(seed.wrapping_add(91), x, z) % 47 == 0
}

fn hash(seed: i64, x: i32, z: i32) -> u32 {
    let mut n = seed as u32
        ^ (x as u32).wrapping_mul(374761393)
        ^ (z as u32).wrapping_mul(668265263);
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n ^ (n >> 16)
}

fn value_noise(seed: i64, x: f32, z: f32, scale: f32) -> f32 {
    let nx = x / scale;
    let nz = z / scale;
    let x0 = nx.floor() as i32;
    let z0 = nz.floor() as i32;
    let fx = nx - x0 as f32;
    let fz = nz - z0 as f32;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let n00 = hash01(seed, x0, z0);
    let n10 = hash01(seed, x0 + 1, z0);
    let n01 = hash01(seed, x0, z0 + 1);
    let n11 = hash01(seed, x0 + 1, z0 + 1);
    let a = n00 + (n10 - n00) * sx;
    let b = n01 + (n11 - n01) * sx;
    a + (b - a) * sz
}

fn hash01(seed: i64, x: i32, z: i32) -> f32 {
    (hash(seed, x, z) as f32) / (u32::MAX as f32)
}

fn fbm(seed: i64, x: f32, z: f32) -> f32 {
    let mut v = 0.0;
    let mut a = 1.0;
    let mut s = 48.0;
    let mut n = 0.0;
    for oct in 0..4 {
        v += (value_noise(seed.wrapping_add(oct as i64 * 19), x, z, s) * 2.0 - 1.0) * a;
        n += a;
        a *= 0.5;
        s *= 0.5;
    }
    v / n
}

pub fn spawn_pos(seed: i64) -> engine_core::Vec3 {
    let mut x = 0.5f32;
    let mut z = 0.5f32;
    let y = surface_y(seed, 0, 0) as f32 + 1.0;
    // stand on grass, not inside a tree
    if tree_here(seed, 0, 0) {
        x = 8.5;
        z = 8.5;
    }
    let y2 = surface_y(seed, x as i32, z as i32) as f32 + 1.0;
    engine_core::Vec3::new(x, y2.max(y), z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_is_deterministic() {
        let a = generate_chunk(123, ChunkPos::new(0, 0));
        let b = generate_chunk(123, ChunkPos::new(0, 0));
        assert_eq!(a.get(4, surface_y(123, 4, 4) - 1, 4), b.get(4, surface_y(123, 4, 4) - 1, 4));
        let c = generate_chunk(124, ChunkPos::new(0, 0));
        assert_ne!(a.snapshot().blocks, c.snapshot().blocks);
    }

    #[test]
    fn hills_vary() {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for z in 0..64 {
            for x in 0..64 {
                let y = surface_y(1, x, z);
                min = min.min(y);
                max = max.max(y);
            }
        }
        assert!(max - min >= 8, "expected hills, span {}", max - min);
    }
}
