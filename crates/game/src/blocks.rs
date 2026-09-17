pub const AIR: u16 = 0;
pub const GRASS: u16 = 1;
pub const DIRT: u16 = 2;
pub const STONE: u16 = 3;
pub const COBBLE: u16 = 4;
pub const LOG: u16 = 5;
pub const PLANKS: u16 = 6;
pub const CRAFTING_TABLE: u16 = 7;
pub const CHEST: u16 = 8;

pub fn is_solid(id: u16) -> bool {
    id != AIR
}

/// Face 0=-x 1=+x 2=-y 3=+y 4=-z 5=+z
pub fn face_offset(face: u8) -> (i32, i32, i32) {
    match face {
        0 => (-1, 0, 0),
        1 => (1, 0, 0),
        2 => (0, -1, 0),
        3 => (0, 1, 0),
        4 => (0, 0, -1),
        5 => (0, 0, 1),
        _ => (0, 0, 0),
    }
}
