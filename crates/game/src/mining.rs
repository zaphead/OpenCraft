use crate::blocks;
use crate::items::ItemId;

pub enum ToolKind {
    None,
    Shovel,
    Axe,
    Pick,
}

pub fn tool_for(item: u16) -> (ToolKind, u32) {
    match item {
        x if x == ItemId::WOOD_SHOVEL => (ToolKind::Shovel, 2),
        x if x == ItemId::STONE_SHOVEL => (ToolKind::Shovel, 4),
        x if x == ItemId::ETCH_WOOD_SHOVEL => (ToolKind::Shovel, 4),
        x if x == ItemId::ETCH_STONE_SHOVEL => (ToolKind::Shovel, 6),
        x if x == ItemId::WOOD_AXE => (ToolKind::Axe, 2),
        x if x == ItemId::STONE_AXE => (ToolKind::Axe, 4),
        x if x == ItemId::ETCH_WOOD_AXE => (ToolKind::Axe, 4),
        x if x == ItemId::ETCH_STONE_AXE => (ToolKind::Axe, 6),
        x if x == ItemId::WOOD_PICK => (ToolKind::Pick, 2),
        x if x == ItemId::STONE_PICK => (ToolKind::Pick, 4),
        x if x == ItemId::ETCH_WOOD_PICK => (ToolKind::Pick, 4),
        x if x == ItemId::ETCH_STONE_PICK => (ToolKind::Pick, 6),
        _ => (ToolKind::None, 1),
    }
}

/// Ticks to break. 0 means instant (never for this slice).
pub fn break_ticks(block: u16, held: Option<u16>) -> u32 {
    let (kind, tier) = match held {
        Some(i) => tool_for(i),
        None => (ToolKind::None, 1),
    };
    match block {
        blocks::GRASS | blocks::DIRT => match kind {
            ToolKind::Shovel => 8 / tier.max(1),
            ToolKind::None | ToolKind::Axe | ToolKind::Pick => 15,
        },
        blocks::LOG | blocks::PLANKS | blocks::CRAFTING_TABLE | blocks::CHEST
        | blocks::BIRCH_LOG | blocks::PINE_LOG | blocks::REDWOOD_LOG | blocks::BAOBAB_LOG
        | blocks::WILLOW_LOG | blocks::MUSH_STEM | blocks::HIVE | blocks::BREW => match kind {
            ToolKind::Axe => 12 / tier.max(1),
            ToolKind::None | ToolKind::Shovel | ToolKind::Pick => 20,
        },
        blocks::STONE | blocks::COBBLE | blocks::BASALT | blocks::CRYSTAL => match kind {
            ToolKind::Pick => 30 / tier.max(1),
            ToolKind::None | ToolKind::Shovel | ToolKind::Axe => 150,
        },
        blocks::LEAVES | blocks::BIRCH_LEAVES | blocks::PINE_LEAVES | blocks::REDWOOD_LEAVES
        | blocks::BAOBAB_LEAVES | blocks::WILLOW_LEAVES | blocks::FLOWER | blocks::TUFT
        | blocks::REED | blocks::LILY | blocks::STARBLOOM | blocks::SUNPETAL | blocks::GLOWVINE
        | blocks::VINE | blocks::THORN => 4,
        _ => 20,
    }
    .max(1)
}

pub fn harvest_drop(block: u16, held: Option<u16>) -> Option<(u16, u8)> {
    let (kind, _) = match held {
        Some(i) => tool_for(i),
        None => (ToolKind::None, 1),
    };
    match block {
        blocks::GRASS => Some((blocks::DIRT, 1)),
        blocks::DIRT => Some((blocks::DIRT, 1)),
        blocks::STONE => match kind {
            ToolKind::Pick => Some((blocks::COBBLE, 1)),
            ToolKind::None | ToolKind::Shovel | ToolKind::Axe => None,
        },
        blocks::COBBLE => match kind {
            ToolKind::Pick => Some((blocks::COBBLE, 1)),
            ToolKind::None | ToolKind::Shovel | ToolKind::Axe => None,
        },
        blocks::LEAVES | blocks::BIRCH_LEAVES | blocks::PINE_LEAVES | blocks::REDWOOD_LEAVES
        | blocks::BAOBAB_LEAVES | blocks::WILLOW_LEAVES | blocks::VINE => None,
        blocks::LOG => Some((blocks::LOG, 1)),
        blocks::BIRCH_LOG => Some((blocks::BIRCH_LOG, 1)),
        blocks::PINE_LOG => Some((blocks::PINE_LOG, 1)),
        blocks::REDWOOD_LOG => Some((blocks::REDWOOD_LOG, 1)),
        blocks::BAOBAB_LOG => Some((blocks::BAOBAB_LOG, 1)),
        blocks::WILLOW_LOG => Some((blocks::WILLOW_LOG, 1)),
        blocks::BASALT => Some((ItemId::NETHER_ASH, 1)),
        blocks::HIVE => Some((ItemId::HONEYCOMB, 1)),
        blocks::MUSH_CAP | blocks::KING_CAP => Some((ItemId::GLOWCAP, 1)),
        blocks::STARBLOOM => Some((blocks::STARBLOOM, 1)),
        blocks::CRYSTAL => Some((blocks::CRYSTAL, 1)),
        blocks::GLOWVINE => Some((blocks::GLOWVINE, 1)),
        blocks::FLOWER => Some((blocks::FLOWER, 1)),
        blocks::SUNPETAL => Some((blocks::SUNPETAL, 1)),
        blocks::SAND => Some((blocks::SAND, 1)),
        blocks::CLAY => Some((blocks::CLAY, 1)),
        blocks::PLANKS => Some((blocks::PLANKS, 1)),
        blocks::CRAFTING_TABLE => Some((blocks::CRAFTING_TABLE, 1)),
        blocks::CHEST => Some((blocks::CHEST, 1)),
        blocks::BREW => Some((blocks::BREW, 1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_needs_pick() {
        assert!(harvest_drop(blocks::STONE, None).is_none());
        assert_eq!(
            harvest_drop(blocks::STONE, Some(ItemId::WOOD_PICK)),
            Some((blocks::COBBLE, 1))
        );
    }

    #[test]
    fn leaves_vanish_fast() {
        assert_eq!(harvest_drop(blocks::LEAVES, None), None);
        assert_eq!(harvest_drop(blocks::LEAVES, Some(ItemId::WOOD_AXE)), None);
        assert!(break_ticks(blocks::LEAVES, None) <= 6);
    }
}
