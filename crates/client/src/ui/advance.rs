use engine_core::Vec2;

use super::prim::{Button, Panel, Text};
use super::UiFrame;

pub struct AdvancementScreen;

const LINES: &[(&str, u16, bool)] = &[
    ("Wake in a meadow", 1, false),
    ("Fell a tree", 2, true),
    ("Take a hide", 4, true),
    ("Ride", 8, false),
    ("Etch a tool", 16, false),
    ("Drink a potion", 32, false),
    ("See a blood moon", 64, false),
    ("Strike the Wyrm", 128, false),
];

impl AdvancementScreen {
    pub fn draw(ui: &mut UiFrame, bits: u16, cursor: Vec2) -> Button {
        super::darken(ui);
        let s = ui.scale;
        let pw = 220.0 * s;
        let ph = 168.0 * s;
        let origin = Vec2::new((ui.size.x - pw) * 0.5, (ui.size.y - ph) * 0.5);
        Panel::draw(ui, origin, origin + Vec2::new(pw, ph), [0.12, 0.12, 0.12, 0.96], 0.08);
        Text::draw(ui, origin + Vec2::new(16.0 * s, 8.0 * s), "Advancements", [1.0, 1.0, 1.0, 1.0], 0.02);
        for (i, (label, bit, child)) in LINES.iter().enumerate() {
            let mark = if bits & bit != 0 { "+" } else { "-" };
            let x = if *child { 28.0 } else { 16.0 };
            let line = format!("{mark} {label}");
            Text::draw(
                ui,
                origin + Vec2::new(x * s, (28.0 + i as f32 * 12.0) * s),
                &line,
                [0.92, 0.92, 0.82, 1.0],
                0.02,
            );
        }
        let min = origin + Vec2::new(20.0 * s, 140.0 * s);
        let max = min + Vec2::new(140.0 * s, 18.0 * s);
        let hover = cursor.x >= min.x && cursor.x <= max.x && cursor.y >= min.y && cursor.y <= max.y;
        Button::draw(ui, min, max, "Back", false, hover)
    }
}
