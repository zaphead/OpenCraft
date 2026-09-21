//! Drops, brew, etch, and who spawns where. The server is the only caller.

use world::EntityKind;

use crate::biome::Biome;
use crate::blocks;
use crate::items::ItemId;
use crate::mind::{ASLEEP, SHUT};

pub const ADV_WAKE: u16 = 1 << 0;
pub const ADV_TREE: u16 = 1 << 1;
pub const ADV_HIDE: u16 = 1 << 2;
pub const ADV_RIDE: u16 = 1 << 3;
pub const ADV_ETCH: u16 = 1 << 4;
pub const ADV_DRINK: u16 = 1 << 5;
pub const ADV_MOON: u16 = 1 << 6;
pub const ADV_WYRM: u16 = 1 << 7;

pub fn max_hp(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Aurochs | EntityKind::Horse => 16,
        EntityKind::Boar | EntityKind::Deer | EntityKind::Husk | EntityKind::Goblin => 12,
        EntityKind::Wraith | EntityKind::Sentinel => 24,
        EntityKind::Wight => 14,
        EntityKind::Treant => 40,
        EntityKind::Wyrm => 60,
        EntityKind::Skywhale => 30,
        EntityKind::Hopper | EntityKind::Imp | EntityKind::Bee => 6,
        EntityKind::Mimic => 14,
        EntityKind::Sage => 20,
        _ => 8,
    }
}

pub fn drops(kind: EntityKind) -> &'static [(u16, u8)] {
    match kind {
        EntityKind::Aurochs => &[(ItemId::HIDE, 1), (ItemId::BEEF, 1)],
        EntityKind::Boar => &[(ItemId::BEEF, 1)],
        EntityKind::Wyrm => &[(ItemId::WYRM_SCALE, 1)],
        _ => &[],
    }
}

pub fn initial_state(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Mimic => SHUT,
        EntityKind::Sentinel => ASLEEP,
        _ => 0,
    }
}

pub fn initial_stamina(kind: EntityKind) -> u8 {
    if kind == EntityKind::Horse { 40 } else { 0 }
}

/// Locals a spawner emits, and the night/day hostiles of a biome.
pub fn spawner_kind(biome: Biome) -> EntityKind {
    match biome {
        Biome::Dune => EntityKind::Husk,
        Biome::Swamp => EntityKind::Wraith,
        Biome::Moor => EntityKind::Goblin,
        Biome::Ash => EntityKind::Imp,
        Biome::Frost => EntityKind::Wight,
        Biome::Mushroom => EntityKind::Hopper,
        _ => EntityKind::Goblin,
    }
}

pub fn residents(biome: Biome) -> &'static [EntityKind] {
    match biome {
        Biome::Meadow => &[EntityKind::Aurochs, EntityKind::Rabbit, EntityKind::Deer, EntityKind::Songbird, EntityKind::Horse],
        Biome::Oak => &[EntityKind::Fox, EntityKind::Boar, EntityKind::Songbird, EntityKind::Bee],
        Biome::Birch | Biome::Pine => &[EntityKind::Rabbit, EntityKind::Deer],
        Biome::Redwood => &[EntityKind::Goat, EntityKind::Griffin],
        Biome::Dune => &[EntityKind::Husk],
        Biome::Savanna => &[EntityKind::Griffin, EntityKind::Aurochs],
        Biome::Jungle => &[EntityKind::Parrot, EntityKind::Boar],
        Biome::Swamp => &[EntityKind::Frog, EntityKind::Wraith],
        Biome::Mushroom => &[EntityKind::Hopper],
        Biome::Coral => &[EntityKind::Fish],
        Biome::Ash => &[EntityKind::Imp],
        Biome::Frost => &[EntityKind::Rabbit, EntityKind::Wight],
        Biome::Glow => &[EntityKind::Glowbeetle],
        Biome::Moor => &[EntityKind::Crow, EntityKind::Goblin],
        Biome::Mesa => &[EntityKind::Goat],
    }
}

pub fn etched(id: u16) -> Option<u16> {
    Some(match id {
        x if x == ItemId::WOOD_PICK => ItemId::ETCH_WOOD_PICK,
        x if x == ItemId::WOOD_AXE => ItemId::ETCH_WOOD_AXE,
        x if x == ItemId::WOOD_SHOVEL => ItemId::ETCH_WOOD_SHOVEL,
        x if x == ItemId::STONE_PICK => ItemId::ETCH_STONE_PICK,
        x if x == ItemId::STONE_AXE => ItemId::ETCH_STONE_AXE,
        x if x == ItemId::STONE_SHOVEL => ItemId::ETCH_STONE_SHOVEL,
        _ => return None,
    })
}

/// Shapeless brew of up to three reagents.
pub fn brew(slots: &[Option<(u16, u8)>]) -> Option<(u16, u8)> {
    let mut ash = false;
    let mut cap = false;
    let mut bloom = false;
    let mut n = 0;
    for s in slots {
        let Some((id, c)) = *s else { continue };
        if c == 0 {
            continue;
        }
        n += 1;
        if id == ItemId::NETHER_ASH {
            ash = true;
        } else if id == ItemId::GLOWCAP {
            cap = true;
        } else if id == blocks::STARBLOOM {
            bloom = true;
        } else {
            return None;
        }
    }
    if n == 3 && ash && cap && bloom {
        return Some((ItemId::POTION_HEAL, 1));
    }
    if n == 2 && ash && cap && !bloom {
        return Some((ItemId::POTION_FIRE, 1));
    }
    if n == 2 && cap && bloom && !ash {
        return Some((ItemId::POTION_GLOW, 1));
    }
    if n == 2 && ash && bloom && !cap {
        return Some((ItemId::POTION_SWIFT, 1));
    }
    None
}

pub fn boss_name(kind: EntityKind) -> Option<&'static str> {
    match kind {
        EntityKind::Treant => Some("Treant elder"),
        EntityKind::Wyrm => Some("The Wyrm"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brew_three_ways_and_the_heal() {
        let ash = Some((ItemId::NETHER_ASH, 1));
        let cap = Some((ItemId::GLOWCAP, 1));
        let bloom = Some((blocks::STARBLOOM, 1));
        assert_eq!(brew(&[ash, cap, None]), Some((ItemId::POTION_FIRE, 1)));
        assert_eq!(brew(&[cap, bloom, None]), Some((ItemId::POTION_GLOW, 1)));
        assert_eq!(brew(&[ash, bloom, None]), Some((ItemId::POTION_SWIFT, 1)));
        assert_eq!(brew(&[ash, cap, bloom]), Some((ItemId::POTION_HEAL, 1)));
    }

    #[test]
    fn etch_maps_wood_and_stone() {
        assert_eq!(etched(ItemId::WOOD_PICK), Some(ItemId::ETCH_WOOD_PICK));
        assert_eq!(etched(ItemId::STONE_AXE), Some(ItemId::ETCH_STONE_AXE));
        assert_eq!(etched(ItemId::STICK), None);
    }
}
