pub const AIR: u16 = 0;
pub const GRASS: u16 = 1;
pub const DIRT: u16 = 2;
pub const STONE: u16 = 3;
pub const COBBLE: u16 = 4;
pub const LOG: u16 = 5;
pub const PLANKS: u16 = 6;
pub const CRAFTING_TABLE: u16 = 7;
pub const CHEST: u16 = 8;
pub const LEAVES: u16 = 9;
pub const WATER: u16 = 10;
pub const LAVA: u16 = 11;
pub const SPRING: u16 = 12;
pub const SAND: u16 = 13;
pub const CLAY: u16 = 14;
pub const BASALT: u16 = 15;
// 16–22 are the existing stick and tool items. Blocks resume at 23.
pub const PACKED_ICE: u16 = 23;
pub const BIRCH_LOG: u16 = 24;
pub const BIRCH_LEAVES: u16 = 25;
pub const PINE_LOG: u16 = 26;
pub const PINE_LEAVES: u16 = 27;
pub const REDWOOD_LOG: u16 = 28;
pub const REDWOOD_LEAVES: u16 = 29;
pub const BAOBAB_LOG: u16 = 30;
pub const BAOBAB_LEAVES: u16 = 31;
pub const WILLOW_LOG: u16 = 32;
pub const WILLOW_LEAVES: u16 = 33;
pub const MUSH_STEM: u16 = 34;
pub const MUSH_CAP: u16 = 35;
pub const KING_CAP: u16 = 36;
pub const CORAL: u16 = 37;
pub const FLOWER: u16 = 38;
pub const TUFT: u16 = 39;
pub const REED: u16 = 40;
pub const LILY: u16 = 41;
pub const THORN: u16 = 42;
pub const STARBLOOM: u16 = 43;
pub const SUNPETAL: u16 = 44;
pub const GLOWVINE: u16 = 45;
pub const CRYSTAL: u16 = 46;
pub const HIVE: u16 = 47;
pub const BREW: u16 = 48;
pub const VAULT: u16 = 49;
pub const BEAN: u16 = 50;
pub const RAIN_R: u16 = 51;
pub const RAIN_O: u16 = 52;
pub const RAIN_Y: u16 = 53;
pub const RAIN_G: u16 = 54;
pub const RAIN_B: u16 = 55;
pub const RAIN_I: u16 = 56;
pub const RAIN_V: u16 = 57;
pub const SPAWNER: u16 = 58;
pub const GEYSER: u16 = 59;
pub const VINE: u16 = 60;

pub fn is_solid(id: u16) -> bool {
    world::solid_id(id)
}

pub fn is_fluid(id: u16) -> bool {
    matches!(id, WATER | LAVA | SPRING)
}

pub fn is_flora(id: u16) -> bool {
    matches!(
        id,
        FLOWER | TUFT | REED | LILY | STARBLOOM | SUNPETAL | GLOWVINE | LEAVES | BIRCH_LEAVES | PINE_LEAVES
            | REDWOOD_LEAVES | BAOBAB_LEAVES | WILLOW_LEAVES | MUSH_CAP | KING_CAP
    )
}

pub fn is_leaf(id: u16) -> bool {
    matches!(
        id,
        LEAVES | BIRCH_LEAVES | PINE_LEAVES | REDWOOD_LEAVES | BAOBAB_LEAVES | WILLOW_LEAVES
    )
}

/// Crossed plant, not a cube. Thorn stays a solid cube so a maze has walls.
pub fn is_cross(id: u16) -> bool {
    matches!(id, FLOWER | TUFT | REED | STARBLOOM | SUNPETAL | GLOWVINE | VINE)
}

pub fn is_log(id: u16) -> bool {
    matches!(id, LOG | BIRCH_LOG | PINE_LOG | REDWOOD_LOG | BAOBAB_LOG | WILLOW_LOG | MUSH_STEM)
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
