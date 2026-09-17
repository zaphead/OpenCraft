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
        x if x == ItemId::WOOD_AXE => (ToolKind::Axe, 2),
        x if x == ItemId::STONE_AXE => (ToolKind::Axe, 4),
        x if x == ItemId::WOOD_PICK => (ToolKind::Pick, 2),
        x if x == ItemId::STONE_PICK => (ToolKind::Pick, 4),
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
        blocks::LOG | blocks::PLANKS | blocks::CRAFTING_TABLE | blocks::CHEST => match kind {
            ToolKind::Axe => 12 / tier.max(1),
            ToolKind::None | ToolKind::Shovel | ToolKind::Pick => 20,
        },
        blocks::STONE | blocks::COBBLE => match kind {
            ToolKind::Pick => 30 / tier.max(1),
            ToolKind::None | ToolKind::Shovel | ToolKind::Axe => 150,
        },
        // Leaves are soft foliage: fast by hand or any tool, never a tool gate.
        blocks::LEAVES => 4,
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
        blocks::LOG => Some((blocks::LOG, 1)),
        blocks::PLANKS => Some((blocks::PLANKS, 1)),
        blocks::CRAFTING_TABLE => Some((blocks::CRAFTING_TABLE, 1)),
        blocks::CHEST => Some((blocks::CHEST, 1)),
        // Leaves vanish on break: no drop, no decay, no recipe.
        blocks::LEAVES => None,
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
