use engine_core::{ChunkPos, MIN_Y, SECTION_EDGE};
use world::Chunk;

use crate::biome::{biome_at, filler_block, height_bias, sea_level, surface_block, Biome};
use crate::blocks;

pub fn generate_chunk(seed: i64, pos: ChunkPos) -> Chunk {
    let mut chunk = Chunk::empty(pos);
    let ox = pos.x * SECTION_EDGE;
    let oz = pos.z * SECTION_EDGE;
    for lz in 0..16 {
        for lx in 0..16 {
            let wx = ox + lx;
            let wz = oz + lz;
            let biome = biome_at(seed, wx, wz);
            let h = surface_y(seed, wx, wz);
            for y in MIN_Y..h {
                let id = if y == h - 1 {
                    surface_block(biome, y)
                } else if y >= h - 4 {
                    filler_block(biome)
                } else if biome == Biome::Frost && y > h - 14 {
                    blocks::PACKED_ICE
                } else if biome == Biome::Ash && y > h - 10 {
                    blocks::BASALT
                } else {
                    blocks::STONE
                };
                chunk.set(lx as u32, y, lz as u32, id);
            }
            let sea = sea_level(biome);
            if sea > h {
                for y in h..sea {
                    chunk.set(lx as u32, y, lz as u32, blocks::WATER);
                }
            }
            let river = (wz as f32 - 36.0 - (wx as f32 * 0.04).sin() * 8.0).abs();
            if river < 2.5 && (-40..220).contains(&wx) && h < 34 {
                for y in h..34 {
                    chunk.set(lx as u32, y, lz as u32, blocks::WATER);
                }
            }
            let pond = (wx - 18) * (wx - 18) + (wz - 22) * (wz - 22);
            if pond < 16 && biome == Biome::Meadow {
                let bed = (h - 2).max(MIN_Y);
                for y in bed..h {
                    chunk.set(lx as u32, y, lz as u32, blocks::WATER);
                }
            }
            if tree_here(seed, wx, wz)
                && matches!(biome, Biome::Meadow | Biome::Oak)
                && h > MIN_Y + 4
                && h + 5 < engine_core::MAX_Y
            {
                let top = chunk.get(lx as u32, h - 1, lz as u32);
                if top == blocks::GRASS {
                    chunk.set(lx as u32, h - 1, lz as u32, blocks::DIRT);
                    let th = 4 + (hash(seed, wx, wz) % 3) as i32;
                    for dy in 0..th {
                        chunk.set(lx as u32, h + dy, lz as u32, blocks::LOG);
                    }
                    canopy(&mut chunk, seed, wx, h + th, wz);
                }
            }
        }
    }
    // Halo pass: trees rooted up to 2 blocks outside this chunk still drape
    // canopy in here. tree_here/surface_y are pure world functions, and a
    // surface column top is grass by construction, so the home-chunk guards
    // apply as-is without reading neighbor chunks (the grass check would
    // always pass; height guards are repeated exactly). canopy() only ever
    // replaces air, so a halo tree can never eat this chunk's trunks.
    for dz in -2..18 {
        for dx in -2..18 {
            if (0..16).contains(&dx) && (0..16).contains(&dz) {
                continue;
            }
            let (wx, wz) = (ox + dx, oz + dz);
            if tree_here(seed, wx, wz) {
                let h = surface_y(seed, wx, wz);
                if h > MIN_Y + 4 && h + 5 < engine_core::MAX_Y {
                    let th = 4 + (hash(seed, wx, wz) % 3) as i32;
                    canopy(&mut chunk, seed, wx, h + th, wz);
                }
            }
        }
    }
    crate::biomes::grow_all(&mut chunk, seed);
    crate::pieces::stamp_all(&mut chunk, seed);
    chunk
}

pub fn surface_y(seed: i64, x: i32, z: i32) -> i32 {
    let biome = biome_at(seed, x, z);
    let (base, amp) = height_bias(biome);
    let n = fbm(seed, x as f32, z as f32);
    let mut h = base + n * amp;
    let river = (z as f32 - 36.0 - (x as f32 * 0.04).sin() * 8.0).abs();
    if river < 2.5 && (-40..220).contains(&x) {
        h = h.min(33.0);
    }
    (h as i32).clamp(MIN_Y + 8, engine_core::MAX_Y - 24)
}

fn tree_here(seed: i64, x: i32, z: i32) -> bool {
    hash(seed.wrapping_add(91), x, z) % 47 == 0
}

/// Leaf canopy around the trunk top. Writes into this chunk only: caller
/// guarantees the trunk fits, the canopy may clip at chunk borders the same
/// way terrain does. Only replaces air, so canopies never eat trunks or hills.
fn canopy(chunk: &mut Chunk, seed: i64, wx: i32, top_y: i32, wz: i32) {
    let ox = chunk.pos.x * SECTION_EDGE;
    let oz = chunk.pos.z * SECTION_EDGE;
    for dy in -2..=1 {
        let r = if dy <= -1 { 2 } else { 1 };
        for dx in -r..=r {
            for dz in -r..=r {
                if dx * dx + dz * dz > r * r + 1 {
                    continue;
                }
                // Ragged deterministic edge, same seed always grows the same tree.
                if dx * dx + dz * dz == r * r + 1 && hash(seed, wx + dx, wz + dz + dy) % 2 == 0 {
                    continue;
                }
                let (x, y, z) = (wx + dx, top_y + dy, wz + dz);
                let (lx, lz) = (x - ox, z - oz);
                if !(0..16).contains(&lx) || !(0..16).contains(&lz) {
                    continue;
                }
                if y < MIN_Y || y >= engine_core::MAX_Y {
                    continue;
                }
                if chunk.get(lx as u32, y, lz as u32) == blocks::AIR {
                    chunk.set(lx as u32, y, lz as u32, blocks::LEAVES);
                }
            }
        }
    }
}

pub(crate) fn hash(seed: i64, x: i32, z: i32) -> u32 {
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

    #[test]
    fn halo_drapes_across_borders() {
        // A tree rooted just west of chunk (0, 0) must still leaf into it.
        // No interior tree is allowed near the asserted cells, so any leaves
        // there prove the halo pass, not the home loop.
        let mut grown = None;
        for seed in 1..2000 {
            for wz in 2..14 {
                for wx in -2..0 {
                    if !tree_here(seed, wx, wz) {
                        continue;
                    }
                    let h = surface_y(seed, wx, wz);
                    if !(h > MIN_Y + 4 && h + 5 < engine_core::MAX_Y) {
                        continue;
                    }
                    let clear = (0..6).all(|ix| {
                        ((wz - 3)..(wz + 3)).all(|iz| !tree_here(seed, ix, iz))
                    });
                    if clear {
                        grown = Some((seed, wx, wz, h));
                        break;
                    }
                }
                if grown.is_some() {
                    break;
                }
            }
            if grown.is_some() {
                break;
            }
        }
        let (seed, wx, wz, h) = grown.expect("no isolated border tree in seeds 1..2000");
        let th = 4 + (hash(seed, wx, wz) % 3) as i32;
        let top = h + th;
        let c = generate_chunk(seed, ChunkPos::new(0, 0));
        let mut leaves = 0;
        for lx in 0..2 {
            for y in top - 2..=top + 1 {
                for lz in (wz - 2).max(0)..=(wz + 2).min(15) {
                    if c.get(lx as u32, y, lz as u32) == blocks::LEAVES {
                        leaves += 1;
                    }
                }
            }
        }
        assert!(leaves > 0, "halo left no leaves for seed {seed} tree ({wx}, {wz})");
    }

    #[test]
    fn trees_wear_canopies() {
        // Find a seed with a tree safely inside chunk (0, 0), then prove the
        // canopy exists, the trunk survived it, and the seed regrows it.
        let mut grown = None;
        for seed in 1..500 {
            let c = generate_chunk(seed, ChunkPos::new(0, 0));
            let mut leaves = 0;
            let mut logs = 0;
            for sec in c.sections.iter() {
                for id in sec.fill().iter() {
                    if *id == blocks::LEAVES {
                        leaves += 1;
                    } else if *id == blocks::LOG {
                        logs += 1;
                    }
                }
            }
            if leaves >= 8 && logs >= 4 {
                grown = Some(seed);
                break;
            }
        }
        let seed = grown.expect("no canopied tree in seeds 1..500");
        let again = generate_chunk(seed, ChunkPos::new(0, 0));
        let mut leaves = 0;
        for sec in again.sections.iter() {
            leaves += sec.fill().iter().filter(|id| **id == blocks::LEAVES).count();
        }
        assert!(leaves >= 8, "seed {seed} regrew {leaves} leaves");
    }
}
