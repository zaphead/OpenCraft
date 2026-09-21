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
    pub const HIDE: u16 = 64;
    pub const BEEF: u16 = 65;
    pub const HONEYCOMB: u16 = 66;
    pub const GLOWCAP: u16 = 67;
    pub const NETHER_ASH: u16 = 68;
    pub const GOLD: u16 = 69;
    pub const STAFF: u16 = 70;
    pub const KEY: u16 = 71;
    pub const WYRM_SCALE: u16 = 72;
    pub const LANTERN: u16 = 73;
    pub const POTION_FIRE: u16 = 74;
    pub const POTION_GLOW: u16 = 75;
    pub const POTION_SWIFT: u16 = 76;
    pub const POTION_HEAL: u16 = 77;
    pub const ETCH_WOOD_PICK: u16 = 78;
    pub const ETCH_WOOD_AXE: u16 = 79;
    pub const ETCH_WOOD_SHOVEL: u16 = 80;
    pub const ETCH_STONE_PICK: u16 = 81;
    pub const ETCH_STONE_AXE: u16 = 82;
    pub const ETCH_STONE_SHOVEL: u16 = 83;
}

pub fn is_block_item(id: u16) -> bool {
    (1..=15).contains(&id) || (23..=60).contains(&id)
}

pub fn item_max(id: u16) -> u8 {
    if (ItemId::WOOD_PICK..=ItemId::STONE_SHOVEL).contains(&id)
        || (ItemId::STAFF..=ItemId::LANTERN).contains(&id)
        || (ItemId::POTION_FIRE..=ItemId::ETCH_STONE_SHOVEL).contains(&id)
    {
        1
    } else {
        64
    }
}

pub fn food_hearts(id: u16) -> u8 {
    if id == ItemId::BEEF || id == ItemId::HONEYCOMB {
        2
    } else {
        0
    }
}

pub fn is_potion(id: u16) -> bool {
    (ItemId::POTION_FIRE..=ItemId::POTION_HEAL).contains(&id)
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
