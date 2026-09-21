//! Biome sample. Spawn is always a sunpetal meadow. Every other biome sits
//! inside a few minutes' walk of the origin, for every seed.

use crate::blocks;

pub const CELL: i32 = 72;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Meadow,
    Oak,
    Birch,
    Pine,
    Redwood,
    Dune,
    Mesa,
    Savanna,
    Jungle,
    Swamp,
    Mushroom,
    Coral,
    Ash,
    Frost,
    Glow,
    Moor,
}

impl Biome {
    pub fn all() -> &'static [Biome] {
        &[
            Self::Meadow,
            Self::Oak,
            Self::Birch,
            Self::Pine,
            Self::Redwood,
            Self::Dune,
            Self::Mesa,
            Self::Savanna,
            Self::Jungle,
            Self::Swamp,
            Self::Mushroom,
            Self::Coral,
            Self::Ash,
            Self::Frost,
            Self::Glow,
            Self::Moor,
        ]
    }
}

/// Fixed compass so seed 1234 (and every other seed) can walk each biome.
/// Cell (0, 0) and the spawn disk stay meadow.
fn cell_biome(cx: i32, cz: i32) -> Biome {
    match (cx, cz) {
        (0, 0) => Biome::Meadow,
        (0, 1) => Biome::Oak,
        (1, 0) => Biome::Birch,
        (2, 0) => Biome::Pine,
        (2, 1) => Biome::Redwood,
        (-2, 0) => Biome::Dune,
        (0, -1) => Biome::Mesa,
        (-1, 0) => Biome::Savanna,
        (1, 2) => Biome::Jungle,
        (0, 2) => Biome::Swamp,
        (2, 2) => Biome::Mushroom,
        (-1, -2) => Biome::Coral,
        (-2, 1) => Biome::Ash,
        (-2, -1) => Biome::Frost,
        (-1, 2) => Biome::Glow,
        (1, -1) => Biome::Moor,
        _ => {
            let h = (cx.wrapping_mul(13) ^ cz.wrapping_mul(29)).unsigned_abs() as usize;
            Biome::all()[h % Biome::all().len()]
        }
    }
}

pub fn biome_at(_seed: i64, x: i32, z: i32) -> Biome {
    if x * x + z * z < 40 * 40 {
        return Biome::Meadow;
    }
    cell_biome(x.div_euclid(CELL), z.div_euclid(CELL))
}

pub fn cell_of(biome: Biome) -> (i32, i32) {
    match biome {
        Biome::Meadow => (0, 0),
        Biome::Oak => (0, 1),
        Biome::Birch => (1, 0),
        Biome::Pine => (2, 0),
        Biome::Redwood => (2, 1),
        Biome::Dune => (-2, 0),
        Biome::Mesa => (0, -1),
        Biome::Savanna => (-1, 0),
        Biome::Jungle => (1, 2),
        Biome::Swamp => (0, 2),
        Biome::Mushroom => (2, 2),
        Biome::Coral => (-1, -2),
        Biome::Ash => (-2, 1),
        Biome::Frost => (-2, -1),
        Biome::Glow => (-1, 2),
        Biome::Moor => (1, -1),
    }
}

pub fn cell_center(biome: Biome) -> (i32, i32) {
    let (cx, cz) = cell_of(biome);
    (cx * CELL + CELL / 2, cz * CELL + CELL / 2)
}

/// Feature anchor, jittered inside its biome cell by the seed.
pub fn anchor(seed: i64, id: i32, biome: Biome) -> (i32, i32) {
    let (cx, cz) = cell_center(biome);
    let h = hash(seed, id, cx);
    let dx = (h % 17) as i32 - 8;
    let dz = (hash(seed, cz, id) % 17) as i32 - 8;
    (cx + dx, cz + dz)
}

pub fn surface_block(biome: Biome, y: i32) -> u16 {
    match biome {
        Biome::Dune => blocks::SAND,
        Biome::Mesa => {
            if (y / 3) % 2 == 0 {
                blocks::CLAY
            } else {
                blocks::SAND
            }
        }
        Biome::Ash => blocks::BASALT,
        Biome::Frost => blocks::PACKED_ICE,
        Biome::Coral => blocks::SAND,
        Biome::Mushroom => blocks::DIRT,
        _ => blocks::GRASS,
    }
}

pub fn filler_block(biome: Biome) -> u16 {
    match biome {
        Biome::Dune | Biome::Mesa | Biome::Coral => blocks::SAND,
        Biome::Ash => blocks::BASALT,
        Biome::Frost => blocks::PACKED_ICE,
        _ => blocks::DIRT,
    }
}

pub fn height_bias(biome: Biome) -> (f32, f32) {
    match biome {
        Biome::Meadow => (48.0, 10.0),
        Biome::Oak => (46.0, 8.0),
        Biome::Birch => (50.0, 8.0),
        Biome::Pine => (58.0, 12.0),
        Biome::Redwood => (68.0, 8.0),
        Biome::Dune => (44.0, 5.0),
        Biome::Mesa => (62.0, 14.0),
        Biome::Savanna => (50.0, 3.0),
        Biome::Jungle => (46.0, 7.0),
        Biome::Swamp => (36.0, 2.0),
        Biome::Mushroom => (44.0, 3.0),
        Biome::Coral => (30.0, 2.0),
        Biome::Ash => (48.0, 5.0),
        Biome::Frost => (60.0, 8.0),
        Biome::Glow => (46.0, 6.0),
        Biome::Moor => (44.0, 4.0),
    }
}

pub fn sea_level(biome: Biome) -> i32 {
    match biome {
        Biome::Swamp | Biome::Coral | Biome::Jungle => 38,
        _ => 0,
    }
}

fn hash(seed: i64, x: i32, z: i32) -> u32 {
    let mut n = seed as u32 ^ (x as u32).wrapping_mul(374761393) ^ (z as u32).wrapping_mul(668265263);
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n ^ (n >> 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_disk_is_meadow() {
        assert_eq!(biome_at(1234, 0, 0), Biome::Meadow);
        assert_eq!(biome_at(999, 8, -8), Biome::Meadow);
    }

    #[test]
    fn seed_1234_biomes_are_a_short_walk() {
        for b in Biome::all() {
            let (x, z) = cell_center(*b);
            assert_eq!(biome_at(1234, x, z), *b, "{b:?} center landed in the wrong biome");
            let d = ((x * x + z * z) as f32).sqrt();
            assert!(d < 360.0, "{b:?} is {d} blocks out");
        }
    }
}
