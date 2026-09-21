use engine_core::Vec2;

use super::prim::{Button, Panel, Text};
use super::UiFrame;

pub struct PauseMenu;

impl PauseMenu {
    pub fn draw(
        ui: &mut UiFrame,
        seed: i64,
        confirm: bool,
        cursor: Vec2,
    ) -> (Button, Button, Button) {
        super::darken(ui);
        let s = ui.scale;
        let pw = 180.0 * s;
        let ph = if confirm { 90.0 * s } else { 120.0 * s };
        let origin = Vec2::new((ui.size.x - pw) * 0.5, (ui.size.y - ph) * 0.5);
        Panel::draw(ui, origin, origin + Vec2::new(pw, ph), [0.12, 0.12, 0.12, 0.96], 0.08);
        if confirm {
            Text::draw(
                ui,
                origin + Vec2::new(12.0 * s, 12.0 * s),
                "World is not saved",
                [1.0, 1.0, 1.0, 1.0],
                0.02,
            );
            let q = btn(ui, origin + Vec2::new(16.0 * s, 40.0 * s), 70.0 * s, "Quit", true, cursor);
            let c = btn(ui, origin + Vec2::new(94.0 * s, 40.0 * s), 70.0 * s, "Cancel", false, cursor);
            return (c, q, q);
        }
        Text::draw(
            ui,
            origin + Vec2::new(16.0 * s, 10.0 * s),
            "Game paused",
            [1.0, 1.0, 1.0, 1.0],
            0.02,
        );
        Text::draw(
            ui,
            origin + Vec2::new(16.0 * s, 24.0 * s),
            &format!("Seed: {seed}"),
            [0.85, 0.85, 0.7, 1.0],
            0.02,
        );
        let back = btn(ui, origin + Vec2::new(20.0 * s, 44.0 * s), 140.0 * s, "Back to Game", false, cursor);
        let adv = btn(ui, origin + Vec2::new(20.0 * s, 66.0 * s), 140.0 * s, "Advancements", false, cursor);
        let quit = btn(ui, origin + Vec2::new(20.0 * s, 88.0 * s), 140.0 * s, "Quit", true, cursor);
        (back, adv, quit)
    }
}

fn btn(ui: &mut UiFrame, min: Vec2, w: f32, label: &str, dest: bool, cursor: Vec2) -> Button {
    let max = min + Vec2::new(w, 18.0 * ui.scale);
    let hover = cursor.x >= min.x && cursor.x <= max.x && cursor.y >= min.y && cursor.y <= max.y;
    Button::draw(ui, min, max, label, dest, hover)
}
