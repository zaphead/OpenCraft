use crate::blocks;
use crate::items::ItemId;

/// Shaped/shapeless recipes for the core slice. `w` is 2 or 3.
pub fn craft_result(grid: &[Option<(u16, u8)>], w: usize) -> Option<(u16, u8)> {
    let n = w * w;
    if grid.len() < n {
        return None;
    }
    let mut cells = vec![0u16; n];
    for i in 0..n {
        cells[i] = grid[i].map(|(id, c)| if c > 0 { id } else { 0 }).unwrap_or(0);
    }
    if let Some(r) = shapeless(&cells) {
        return Some(r);
    }
    shaped(&cells, w)
}

fn count(cells: &[u16], id: u16) -> usize {
    cells.iter().filter(|c| **c == id).count()
}

fn only(cells: &[u16], ids: &[u16]) -> bool {
    cells.iter().all(|c| *c == 0 || ids.contains(c))
}

fn shapeless(cells: &[u16]) -> Option<(u16, u8)> {
    let filled: Vec<u16> = cells.iter().copied().filter(|c| *c != 0).collect();
    if filled.is_empty() {
        return None;
    }
    if filled.iter().all(|c| *c == blocks::LOG) && only(cells, &[blocks::LOG]) {
        return Some((blocks::PLANKS, (4 * filled.len()).min(64) as u8));
    }
    if filled.len() == 2 && count(cells, blocks::PLANKS) == 2 {
        return Some((ItemId::STICK, 4));
    }
    None
}

fn shaped(cells: &[u16], w: usize) -> Option<(u16, u8)> {
    if w == 2 {
        // 2x2 planks = table
        if cells.iter().all(|c| *c == blocks::PLANKS) {
            return Some((blocks::CRAFTING_TABLE, 1));
        }
    }
    if w == 3 {
        // chest: planks around empty center
        let chest = [
            blocks::PLANKS,
            blocks::PLANKS,
            blocks::PLANKS,
            blocks::PLANKS,
            0,
            blocks::PLANKS,
            blocks::PLANKS,
            blocks::PLANKS,
            blocks::PLANKS,
        ];
        if cells == chest {
            return Some((blocks::CHEST, 1));
        }
        if let Some(r) = tool_recipe(cells, blocks::PLANKS, ItemId::WOOD_PICK, ItemId::WOOD_AXE, ItemId::WOOD_SHOVEL) {
            return Some(r);
        }
        if let Some(r) = tool_recipe(cells, blocks::COBBLE, ItemId::STONE_PICK, ItemId::STONE_AXE, ItemId::STONE_SHOVEL) {
            return Some(r);
        }
        // 2x2 planks in any position of 3x3 = table
        if let Some(r) = find_2x2_planks(cells) {
            return Some(r);
        }
        // two planks stacked anywhere -> sticks (also shapeless already)
    }
    // 2x2 table inside 2x2 already handled
    None
}

fn find_2x2_planks(cells: &[u16]) -> Option<(u16, u8)> {
    for oy in 0..2 {
        for ox in 0..2 {
            let i = |x: usize, y: usize| cells[y * 3 + x];
            if i(ox as usize, oy as usize) == blocks::PLANKS
                && i(ox as usize + 1, oy as usize) == blocks::PLANKS
                && i(ox as usize, oy as usize + 1) == blocks::PLANKS
                && i(ox as usize + 1, oy as usize + 1) == blocks::PLANKS
                && cells.iter().filter(|c| **c != 0).count() == 4
            {
                return Some((blocks::CRAFTING_TABLE, 1));
            }
        }
    }
    None
}

fn tool_recipe(
    cells: &[u16],
    head: u16,
    pick: u16,
    axe: u16,
    shovel: u16,
) -> Option<(u16, u8)> {
    let stick = ItemId::STICK;
    // pick: ### / .|. / .|.
    let pick_p = [head, head, head, 0, stick, 0, 0, stick, 0];
    if *cells == pick_p {
        return Some((pick, 1));
    }
    // axe: ##. / #|. / .|.
    let axe_l = [head, head, 0, head, stick, 0, 0, stick, 0];
    let axe_r = [0, head, head, 0, stick, head, 0, stick, 0];
    if *cells == axe_l || *cells == axe_r {
        return Some((axe, 1));
    }
    // shovel: .#. / .|. / .|.
    let sh = [0, head, 0, 0, stick, 0, 0, stick, 0];
    if *cells == sh {
        return Some((shovel, 1));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_to_planks() {
        let g = [Some((blocks::LOG, 1)), None, None, None];
        assert_eq!(craft_result(&g, 2), Some((blocks::PLANKS, 4)));
    }

    #[test]
    fn planks_to_table() {
        let g = [
            Some((blocks::PLANKS, 1)),
            Some((blocks::PLANKS, 1)),
            Some((blocks::PLANKS, 1)),
            Some((blocks::PLANKS, 1)),
        ];
        assert_eq!(craft_result(&g, 2), Some((blocks::CRAFTING_TABLE, 1)));
    }

    #[test]
    fn wooden_pick_recipe() {
        let g = [
            Some((blocks::PLANKS, 1)),
            Some((blocks::PLANKS, 1)),
            Some((blocks::PLANKS, 1)),
            None,
            Some((ItemId::STICK, 1)),
            None,
            None,
            Some((ItemId::STICK, 1)),
            None,
        ];
        assert_eq!(craft_result(&g, 3), Some((ItemId::WOOD_PICK, 1)));
    }
}
