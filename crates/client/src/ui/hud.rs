use engine_core::Vec2;
use protocol::InventorySnap;

use super::prim::{Crosshair, Panel, Slot, Text};
use super::UiFrame;
use crate::atlas::{item_tile, tile_uv};

pub struct HudLayer;

impl HudLayer {
    pub fn draw(
        ui: &mut UiFrame,
        inv: &InventorySnap,
        health: u8,
        debug: Option<engine_core::Vec3>,
        show_hotbar: bool,
        show_crosshair: bool,
        boss: Option<(&str, u8, u8)>,
    ) {
        if show_crosshair {
            Crosshair::draw(ui);
        }
        if show_hotbar {
            if let Some((name, hp, max)) = boss {
                BossBar::draw(ui, name, hp, max);
            }
            HeartRow::draw(ui, health);
            Hotbar::draw(ui, inv);
        }
        if let Some(p) = debug {
            DebugMeter::draw(ui, p);
        }
    }
}

pub struct BossBar;

impl BossBar {
    pub fn draw(ui: &mut UiFrame, name: &str, hp: u8, max: u8) {
        let s = ui.scale;
        let w = 140.0 * s;
        let origin = Vec2::new((ui.size.x - w) * 0.5, ui.size.y - 48.0 * s);
        Panel::draw(ui, origin, origin + Vec2::new(w, 10.0 * s), [0.1, 0.1, 0.1, 0.9], 0.04);
        let fill = w * (hp as f32 / max.max(1) as f32);
        Panel::draw(ui, origin, origin + Vec2::new(fill.max(2.0), 10.0 * s), [0.55, 0.12, 0.12, 1.0], 0.03);
        Text::draw(ui, origin + Vec2::new(4.0 * s, -10.0 * s), name, [1.0, 0.9, 0.85, 1.0], 0.02);
    }
}

pub struct HeartRow;

impl HeartRow {
    pub fn draw(ui: &mut UiFrame, health: u8) {
        let s = ui.scale;
        let slot = 18.0 * s;
        let hot_w = 9.0 * slot;
        let origin = Vec2::new((ui.size.x - hot_w) * 0.5, ui.size.y - 22.0 * s - 10.0 * s);
        for i in 0..10 {
            let full = health > i * 2 + 1;
            let half = health == i * 2 + 1;
            let color = if full {
                [0.86, 0.16, 0.16, 1.0]
            } else if half {
                [0.86, 0.16, 0.16, 0.55]
            } else {
                [0.12, 0.12, 0.12, 1.0]
            };
            let x = origin.x + i as f32 * 9.0 * s;
            heart(ui, Vec2::new(x, origin.y), color);
        }
    }
}

fn heart(ui: &mut UiFrame, pos: Vec2, color: [f32; 4]) {
    let s = ui.scale;
    Panel::draw(ui, pos + Vec2::new(s, 0.0), pos + Vec2::new(4.0 * s, 3.0 * s), color, 0.03);
    Panel::draw(ui, pos + Vec2::new(5.0 * s, 0.0), pos + Vec2::new(8.0 * s, 3.0 * s), color, 0.03);
    Panel::draw(ui, pos + Vec2::new(0.0, 2.0 * s), pos + Vec2::new(9.0 * s, 6.0 * s), color, 0.03);
    Panel::draw(ui, pos + Vec2::new(1.0 * s, 6.0 * s), pos + Vec2::new(8.0 * s, 8.0 * s), color, 0.03);
    Panel::draw(ui, pos + Vec2::new(3.0 * s, 8.0 * s), pos + Vec2::new(6.0 * s, 9.0 * s), color, 0.03);
}

pub struct Hotbar;

impl Hotbar {
    pub fn draw(ui: &mut UiFrame, inv: &InventorySnap) {
        let s = ui.scale;
        let slot = 18.0 * s;
        let w = 9.0 * slot;
        let origin = Vec2::new((ui.size.x - w) * 0.5, ui.size.y - 22.0 * s);
        for i in 0..9 {
            let st = inv.slots.get(i).copied().flatten();
            let uv = st.map(|(id, _)| tile_uv(item_tile(id)));
            Slot::draw(
                ui,
                origin + Vec2::new(i as f32 * slot, 0.0),
                i as u8 == inv.selected,
                st,
                uv,
            );
        }
    }
}

pub struct DebugMeter;

impl DebugMeter {
    pub fn draw(ui: &mut UiFrame, p: engine_core::Vec3) {
        let t = format!("{:.1} {:.1} {:.1}", p.x, p.y, p.z);
        Text::draw(ui, Vec2::new(8.0, 8.0), &t, [1.0, 1.0, 1.0, 1.0], 0.0);
    }
}
