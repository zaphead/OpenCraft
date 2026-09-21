use engine_core::Vec2;

pub const TILE: u32 = 16;
pub const TILES: u32 = 16;
pub const ATLAS: u32 = TILE * TILES;

pub const T_MISSING: u32 = 0;
pub const T_GRASS_TOP: u32 = 1;
pub const T_GRASS_SIDE: u32 = 2;
pub const T_DIRT: u32 = 3;
pub const T_STONE: u32 = 4;
pub const T_COBBLE: u32 = 5;
pub const T_LOG: u32 = 6;
pub const T_LOG_TOP: u32 = 7;
pub const T_PLANKS: u32 = 8;
pub const T_TABLE_TOP: u32 = 9;
pub const T_TABLE_FRONT: u32 = 10;
pub const T_TABLE_SIDE: u32 = 11;
pub const T_CHEST: u32 = 12;
pub const T_STICK: u32 = 13;
pub const T_WOOD_PICK: u32 = 14;
pub const T_WOOD_AXE: u32 = 15;
pub const T_WOOD_SHOVEL: u32 = 16;
pub const T_STONE_PICK: u32 = 17;
pub const T_STONE_AXE: u32 = 18;
pub const T_STONE_SHOVEL: u32 = 19;
pub const T_LEAVES: u32 = 20;

pub fn tile_uv(tile: u32) -> (Vec2, Vec2) {
    let x = (tile % TILES) as f32;
    let y = (tile / TILES) as f32;
    let s = 1.0 / TILES as f32;
    (
        Vec2::new(x * s + 0.5 / ATLAS as f32, y * s + 0.5 / ATLAS as f32),
        Vec2::new((x + 1.0) * s - 0.5 / ATLAS as f32, (y + 1.0) * s - 0.5 / ATLAS as f32),
    )
}

pub fn block_tile(block: u16, face: u8) -> u32 {
    match block {
        1 => match face {
            3 => T_GRASS_TOP,
            2 => T_DIRT,
            _ => T_GRASS_SIDE,
        },
        2 => T_DIRT,
        3 => T_STONE,
        4 => T_COBBLE,
        5 => match face {
            2 | 3 => T_LOG_TOP,
            _ => T_LOG,
        },
        6 => T_PLANKS,
        7 => match face {
            3 => T_TABLE_TOP,
            4 | 5 => T_TABLE_FRONT,
            _ => T_TABLE_SIDE,
        },
        8 => T_CHEST,
        9 => T_LEAVES,
        _ => 30 + block as u32,
    }
}

pub fn item_tile(item: u16) -> u32 {
    match item {
        1..=9 => block_tile(item, 4),
        16 => T_STICK,
        17 => T_WOOD_PICK,
        18 => T_WOOD_AXE,
        19 => T_WOOD_SHOVEL,
        20 => T_STONE_PICK,
        21 => T_STONE_AXE,
        22 => T_STONE_SHOVEL,
        64 => 70,
        65 => 71,
        66 => 72,
        67 => 73,
        68 => 74,
        69 => 75,
        70 => 76,
        71 => 77,
        72 => 78,
        73 => 79,
        74..=77 => 80,
        78..=83 => T_WOOD_PICK,
        _ => {
            if (1..=60).contains(&item) {
                block_tile(item, 4)
            } else {
                T_MISSING
            }
        }
    }
}

pub fn procedural_atlas() -> Vec<u8> {
    let mut data = vec![0u8; (ATLAS * ATLAS * 4) as usize];
    // Missing art falls back to plain rock-grey noise: playable, never magenta.
    fill_tile(&mut data, T_MISSING, speckle([122, 116, 108, 255], [96, 90, 84, 255]));
    fill_tile(&mut data, T_GRASS_TOP, solid([74, 166, 58, 255]));
    fill_tile(&mut data, T_GRASS_SIDE, split([74, 166, 58, 255], [121, 85, 58, 255]));
    fill_tile(&mut data, T_DIRT, solid([121, 85, 58, 255]));
    fill_tile(&mut data, T_STONE, solid([125, 125, 125, 255]));
    fill_tile(&mut data, T_COBBLE, speckle([110, 110, 110, 255], [90, 90, 90, 255]));
    fill_tile(&mut data, T_LOG, solid([102, 81, 50, 255]));
    fill_tile(&mut data, T_LOG_TOP, ring([102, 81, 50, 255], [70, 55, 30, 255]));
    fill_tile(&mut data, T_PLANKS, solid([186, 150, 94, 255]));
    fill_tile(&mut data, T_TABLE_TOP, solid([140, 90, 40, 255]));
    fill_tile(&mut data, T_TABLE_FRONT, solid([120, 70, 30, 255]));
    fill_tile(&mut data, T_TABLE_SIDE, solid([130, 80, 35, 255]));
    fill_tile(&mut data, T_CHEST, solid([150, 90, 30, 255]));
    fill_tile(&mut data, T_STICK, tool([120, 80, 40, 255]));
    fill_tile(&mut data, T_WOOD_PICK, tool([160, 120, 70, 255]));
    fill_tile(&mut data, T_WOOD_AXE, tool([160, 120, 70, 255]));
    fill_tile(&mut data, T_WOOD_SHOVEL, tool([160, 120, 70, 255]));
    fill_tile(&mut data, T_STONE_PICK, tool([140, 140, 140, 255]));
    fill_tile(&mut data, T_STONE_AXE, tool([140, 140, 140, 255]));
    fill_tile(&mut data, T_STONE_SHOVEL, tool([140, 140, 140, 255]));
    fill_tile(&mut data, T_LEAVES, speckle([46, 122, 40, 255], [28, 84, 26, 255]));
    for id in 10u16..=64 {
        fill_tile(&mut data, 30 + id as u32, solid(block_rgba(id)));
    }
    for (tile, c) in [
        (70u32, [140, 96, 48, 255]),
        (71, [160, 64, 48, 255]),
        (72, [210, 170, 40, 255]),
        (73, [180, 80, 200, 255]),
        (74, [70, 70, 70, 255]),
        (75, [212, 175, 55, 255]),
        (76, [90, 140, 220, 255]),
        (77, [180, 150, 60, 255]),
        (78, [120, 200, 160, 255]),
        (79, [240, 220, 120, 255]),
        (80, [120, 200, 255, 255]),
    ] {
        fill_tile(&mut data, tile, solid(c));
    }
    // Reserved white anchor for engine-render ground shadows (see ANCHOR_UV):
    // tile 255 is never blitted by the pack loader, so this pixel survives.
    let anchor = ((255 * ATLAS + 255) * 4) as usize;
    data[anchor..anchor + 4].copy_from_slice(&[255, 255, 255, 255]);
    data
}

fn block_rgba(id: u16) -> [u8; 4] {
    match id {
        10 => [48, 110, 210, 255],
        11 => [220, 80, 20, 255],
        12 => [90, 190, 200, 255],
        13 => [214, 196, 130, 255],
        14 => [176, 92, 64, 255],
        15 => [50, 48, 52, 255],
        23 => [190, 230, 240, 255],
        24 => [220, 220, 210, 255],
        25 => [150, 190, 90, 255],
        26 => [90, 70, 40, 255],
        27 => [40, 90, 40, 255],
        28 => [120, 60, 40, 255],
        29 => [160, 80, 30, 255],
        30 => [150, 110, 60, 255],
        31 => [70, 130, 50, 255],
        32 => [80, 100, 50, 255],
        33 => [60, 110, 60, 255],
        34 => [210, 200, 180, 255],
        35 => [150, 40, 50, 255],
        36 => [180, 40, 160, 255],
        37 => [230, 120, 140, 255],
        38 => [230, 80, 110, 255],
        39 => [70, 160, 50, 255],
        40 => [60, 140, 70, 255],
        41 => [40, 140, 70, 255],
        42 => [50, 80, 30, 255],
        43 => [180, 200, 255, 255],
        44 => [240, 200, 40, 255],
        45 => [80, 220, 140, 255],
        46 => [140, 220, 255, 255],
        47 => [200, 160, 40, 255],
        48 => [90, 60, 40, 255],
        49 => [70, 70, 90, 255],
        50 => [40, 140, 40, 255],
        51 => [220, 40, 40, 255],
        52 => [230, 130, 30, 255],
        53 => [230, 210, 40, 255],
        54 => [40, 180, 60, 255],
        55 => [40, 80, 220, 255],
        56 => [90, 40, 180, 255],
        57 => [140, 40, 180, 255],
        58 => [40, 40, 40, 255],
        59 => [180, 200, 220, 255],
        60 => [40, 100, 40, 255],
        _ => [120, 110, 100, 255],
    }
}

fn fill_tile(data: &mut [u8], tile: u32, px: impl Fn(u32, u32) -> [u8; 4]) {
    let tx = (tile % TILES) * TILE;
    let ty = (tile / TILES) * TILE;
    for y in 0..TILE {
        for x in 0..TILE {
            let c = px(x, y);
            let i = (((ty + y) * ATLAS + tx + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&c);
        }
    }
}

fn solid(c: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| {
        let n = ((x.wrapping_mul(3) + y.wrapping_mul(7)) % 5) as i16 - 2;
        [
            c[0].saturating_add_signed(n as i8),
            c[1].saturating_add_signed(n as i8),
            c[2].saturating_add_signed(n as i8),
            255,
        ]
    }
}

fn split(top: [u8; 4], bot: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| if y < 4 { solid(top)(x, y) } else { solid(bot)(x, y) }
}

fn speckle(a: [u8; 4], b: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| if (x * 13 + y * 7) % 5 == 0 { b } else { a }
}

fn ring(a: [u8; 4], b: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| {
        let dx = x as i32 - 8;
        let dy = y as i32 - 8;
        if dx * dx + dy * dy < 12 { b } else { a }
    }
}

fn tool(c: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| {
        if (x as i32 - y as i32).abs() < 2 || (x > 4 && y < 6 && x < 12) {
            c
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub fn blit_tile(atlas: &mut [u8], tile: u32, rgba: &[u8], w: u32, h: u32) {
    // Tile 255 hosts the ANCHOR_UV white pixel; no pack art may claim it.
    debug_assert_ne!(tile, 255, "tile 255 is the reserved white anchor");
    let tx = (tile % TILES) * TILE;
    let ty = (tile / TILES) * TILE;
    for y in 0..TILE {
        for x in 0..TILE {
            let sx = x * w / TILE;
            let sy = y * h / TILE;
            let si = ((sy * w + sx) * 4) as usize;
            if si + 3 >= rgba.len() {
                continue;
            }
            let i = (((ty + y) * ATLAS + tx + x) * 4) as usize;
            atlas[i..i + 4].copy_from_slice(&rgba[si..si + 4]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face_tiles_read_right() {
        // Grass-top on top, dirt under, side grain on the sides; logs read
        // as logs from every side; leaves and unknowns never go magenta.
        assert_eq!(block_tile(1, 3), T_GRASS_TOP);
        assert_eq!(block_tile(1, 2), T_DIRT);
        assert_eq!(block_tile(1, 4), T_GRASS_SIDE);
        assert_eq!(block_tile(5, 3), T_LOG_TOP);
        assert_eq!(block_tile(5, 0), T_LOG);
        assert_eq!(block_tile(9, 4), T_LEAVES);
        assert_eq!(block_tile(200, 4), T_MISSING);
        // Missing art is grey noise now: sample the tile, reject magenta.
        let atlas = procedural_atlas();
        let tx = (T_MISSING % TILES) * TILE;
        let ty = (T_MISSING / TILES) * TILE;
        for y in 0..TILE {
            for x in 0..TILE {
                let i = (((ty + y) * ATLAS + tx + x) * 4) as usize;
                let (r, g, b) = (atlas[i], atlas[i + 1], atlas[i + 2]);
                assert!(!(r > 200 && g < 80 && b > 200), "magenta at ({x}, {y})");
            }
        }
    }
}
