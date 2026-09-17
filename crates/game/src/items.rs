use world::Stack;

pub struct ItemId;

impl ItemId {
    pub const STICK: u16 = 16;
    pub const WOOD_PICK: u16 = 17;
    pub const WOOD_AXE: u16 = 18;
    pub const WOOD_SHOVEL: u16 = 19;
    pub const STONE_PICK: u16 = 20;
    pub const STONE_AXE: u16 = 21;
    pub const STONE_SHOVEL: u16 = 22;
}

pub fn is_block_item(id: u16) -> bool {
    id >= 1 && id <= 8
}

pub fn item_max(id: u16) -> u8 {
    if id >= ItemId::WOOD_PICK {
        1
    } else {
        64
    }
}

pub fn as_block(id: u16) -> Option<u16> {
    if is_block_item(id) {
        Some(id)
    } else {
        None
    }
}

pub fn merge(into: &mut Stack, add: (u16, u8)) -> Option<(u16, u8)> {
    match *into {
        None => {
            *into = Some(add);
            None
        }
        Some((id, n)) if id == add.0 => {
            let max = item_max(id);
            let sum = n as u16 + add.1 as u16;
            if sum <= max as u16 {
                *into = Some((id, sum as u8));
                None
            } else {
                *into = Some((id, max));
                Some((id, (sum - max as u16) as u8))
            }
        }
        Some(_) => Some(add),
    }
}
