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
        _ => T_MISSING,
    }
}

pub fn item_tile(item: u16) -> u32 {
    match item {
        1..=8 => block_tile(item, 4),
        16 => T_STICK,
        17 => T_WOOD_PICK,
        18 => T_WOOD_AXE,
        19 => T_WOOD_SHOVEL,
        20 => T_STONE_PICK,
        21 => T_STONE_AXE,
        22 => T_STONE_SHOVEL,
        _ => T_MISSING,
    }
}

pub fn procedural_atlas() -> Vec<u8> {
    let mut data = vec![0u8; (ATLAS * ATLAS * 4) as usize];
    fill_tile(&mut data, T_MISSING, checker([255, 0, 255, 255], [0, 0, 0, 255]));
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
    data
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

fn checker(a: [u8; 4], b: [u8; 4]) -> impl Fn(u32, u32) -> [u8; 4] {
    move |x, y| if (x / 8 + y / 8) % 2 == 0 { a } else { b }
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
