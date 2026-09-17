use engine_core::Vec2;
use game::{layout, WindowKind};
use protocol::{InventorySnap, OpenKind};

use super::prim::{Panel, Slot, Text};
use super::UiFrame;
use crate::atlas::{item_tile, tile_uv};

pub struct ContainerScreen;

pub struct SlotGrid;

impl SlotGrid {
    pub fn draw(
        ui: &mut UiFrame,
        inv: &InventorySnap,
        origin: Vec2,
        cols: u16,
        rows: u16,
        start: u16,
        stride: f32,
        hits: &mut Vec<(u16, Vec2, Vec2)>,
    ) {
        for r in 0..rows {
            for c in 0..cols {
                let slot = start + r * cols + c;
                draw_one(ui, inv, origin + Vec2::new(c as f32 * stride, r as f32 * stride), slot, stride, hits);
            }
        }
    }
}

impl ContainerScreen {
    pub fn draw(ui: &mut UiFrame, kind: OpenKind, inv: &InventorySnap, cursor: Vec2) -> Vec<(u16, Vec2, Vec2)> {
        super::darken(ui);
        let s = ui.scale;
        let (pw, ph) = match kind {
            OpenKind::Inventory => (176.0 * s, 166.0 * s),
            OpenKind::CraftingTable => (176.0 * s, 166.0 * s),
            OpenKind::Chest => (176.0 * s, 168.0 * s),
        };
        let origin = Vec2::new((ui.size.x - pw) * 0.5, (ui.size.y - ph) * 0.5);
        Panel::draw(ui, origin, origin + Vec2::new(pw, ph), [0.78, 0.78, 0.78, 1.0], 0.1);
        Panel::draw(
            ui,
            origin + Vec2::splat(3.0),
            origin + Vec2::new(pw, ph) - Vec2::splat(3.0),
            [0.55, 0.55, 0.55, 1.0],
            0.09,
        );

        let mut hits = Vec::new();
        let stride = 18.0 * s;
        let kind_w = match kind {
            OpenKind::Inventory => WindowKind::Inventory,
            OpenKind::CraftingTable => WindowKind::CraftingTable,
            OpenKind::Chest => WindowKind::Chest,
        };
        let map = layout(kind_w);
        match kind {
            OpenKind::Inventory => {
                Text::draw(ui, origin + Vec2::new(8.0 * s, 6.0 * s), "Crafting", [0.15, 0.15, 0.15, 1.0], 0.02);
                SlotGrid::draw(ui, inv, origin + Vec2::new(88.0 * s, 18.0 * s), 2, 2, map.craft, stride, &mut hits);
                draw_one(ui, inv, origin + Vec2::new(144.0 * s, 28.0 * s), map.result, stride, &mut hits);
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 84.0 * s), 9, 3, map.main, stride, &mut hits);
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 142.0 * s), 9, 1, map.hotbar, stride, &mut hits);
            }
            OpenKind::CraftingTable => {
                Text::draw(ui, origin + Vec2::new(8.0 * s, 6.0 * s), "Crafting Table", [0.15, 0.15, 0.15, 1.0], 0.02);
                SlotGrid::draw(ui, inv, origin + Vec2::new(30.0 * s, 16.0 * s), 3, 3, map.craft, stride, &mut hits);
                draw_one(ui, inv, origin + Vec2::new(124.0 * s, 34.0 * s), map.result, stride, &mut hits);
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 84.0 * s), 9, 3, map.main, stride, &mut hits);
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 142.0 * s), 9, 1, map.hotbar, stride, &mut hits);
            }
            OpenKind::Chest => {
                Text::draw(ui, origin + Vec2::new(8.0 * s, 6.0 * s), "Chest", [0.15, 0.15, 0.15, 1.0], 0.02);
                if let Some(chest_base) = map.chest {
                    SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 18.0 * s), 9, 3, chest_base, stride, &mut hits);
                }
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 84.0 * s), 9, 3, map.main, stride, &mut hits);
                SlotGrid::draw(ui, inv, origin + Vec2::new(8.0 * s, 142.0 * s), 9, 1, map.hotbar, stride, &mut hits);
            }
        }
        if let Some((id, n)) = inv.cursor {
            let (uv0, uv1) = tile_uv(item_tile(id));
            Slot::draw(ui, cursor - Vec2::splat(8.0 * s), false, Some((id, n)), Some((uv0, uv1)));
        }
        hits
    }

    pub fn hit(hits: &[(u16, Vec2, Vec2)], p: Vec2) -> Option<u16> {
        for (slot, min, max) in hits {
            if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                return Some(*slot);
            }
        }
        None
    }
}

fn draw_one(
    ui: &mut UiFrame,
    inv: &InventorySnap,
    pos: Vec2,
    slot: u16,
    stride: f32,
    hits: &mut Vec<(u16, Vec2, Vec2)>,
) {
    let st = inv.slots.get(slot as usize).copied().flatten();
    let uv = st.map(|(id, _)| tile_uv(item_tile(id)));
    Slot::draw(ui, pos, false, st, uv);
    hits.push((slot, pos, pos + Vec2::splat(stride)));
}
