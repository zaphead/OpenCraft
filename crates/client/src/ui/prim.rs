use engine_core::Vec2;
use engine_render::UiQuad;

use super::font;
use super::UiFrame;

fn white_uv() -> (Vec2, Vec2) {
    let s = 0.5 / 256.0;
    (Vec2::ZERO, Vec2::new(s, s))
}

pub struct Panel;

impl Panel {
    pub fn draw(ui: &mut UiFrame, min: Vec2, max: Vec2, color: [f32; 4], z: f32) {
        let (uv0, uv1) = white_uv();
        ui.quads.push(UiQuad {
            min,
            max,
            uv_min: uv0,
            uv_max: uv1,
            color,
            z,
        });
    }
}

pub struct Text;

impl Text {
    pub fn draw(ui: &mut UiFrame, origin: Vec2, s: &str, color: [f32; 4], z: f32) {
        let px = ui.scale;
        let mut x = origin.x;
        for ch in s.chars() {
            let g = font::glyph(ch);
            for (row, bits) in g.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        let min = Vec2::new(x + col as f32 * px, origin.y + row as f32 * px);
                        Panel::draw(ui, min, min + Vec2::splat(px), color, z);
                    }
                }
            }
            x += 6.0 * px;
        }
    }

    pub fn width(ui: &UiFrame, s: &str) -> f32 {
        s.chars().count() as f32 * 6.0 * ui.scale
    }
}

pub struct Button {
    pub min: Vec2,
    pub max: Vec2,
}

impl Button {
    pub fn draw(ui: &mut UiFrame, min: Vec2, max: Vec2, label: &str, destructive: bool, hover: bool) -> Self {
        let mut c = if destructive {
            [0.55, 0.18, 0.16, 1.0]
        } else {
            [0.22, 0.48, 0.22, 1.0]
        };
        if hover {
            c[0] = (c[0] + 0.1f32).min(1.0);
            c[1] = (c[1] + 0.1f32).min(1.0);
            c[2] = (c[2] + 0.1f32).min(1.0);
        }
        Panel::draw(ui, min, max, [0.05, 0.05, 0.05, 1.0], 0.05);
        Panel::draw(ui, min + Vec2::splat(2.0), max - Vec2::splat(2.0), c, 0.04);
        let tw = Text::width(ui, label);
        let x = (min.x + max.x - tw) * 0.5;
        let y = (min.y + max.y - 7.0 * ui.scale) * 0.5;
        Text::draw(ui, Vec2::new(x, y), label, [1.0, 1.0, 1.0, 1.0], 0.01);
        Self { min, max }
    }

    pub fn hit(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

pub struct Slot;

impl Slot {
    pub fn draw(
        ui: &mut UiFrame,
        pos: Vec2,
        selected: bool,
        item: Option<(u16, u8)>,
        item_uv: Option<(Vec2, Vec2)>,
    ) {
        let s = 18.0 * ui.scale;
        let inner = 16.0 * ui.scale;
        let border = if selected {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.15, 0.15, 0.15, 1.0]
        };
        Panel::draw(ui, pos, pos + Vec2::splat(s), border, 0.06);
        Panel::draw(
            ui,
            pos + Vec2::splat(ui.scale),
            pos + Vec2::splat(s - ui.scale),
            [0.35, 0.35, 0.35, 1.0],
            0.055,
        );
        if let (Some(_), Some((uv0, uv1))) = (item, item_uv) {
            let pad = Vec2::splat((s - inner) * 0.5);
            ui.quads.push(UiQuad {
                min: pos + pad,
                max: pos + pad + Vec2::splat(inner),
                uv_min: uv0,
                uv_max: uv1,
                color: [1.0, 1.0, 1.0, 1.0],
                z: 0.04,
            });
        }
        if let Some((_, n)) = item {
            if n > 1 {
                Text::draw(
                    ui,
                    pos + Vec2::new(s - 6.0 * ui.scale, s - 8.0 * ui.scale),
                    &n.to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    0.01,
                );
            }
        }
    }
}

pub struct Crosshair;

impl Crosshair {
    pub fn draw(ui: &mut UiFrame) {
        let c = ui.size * 0.5;
        let t = ui.scale;
        let len = 6.0 * ui.scale;
        Panel::draw(
            ui,
            Vec2::new(c.x - len, c.y - t),
            Vec2::new(c.x + len, c.y + t),
            [1.0, 1.0, 1.0, 0.9],
            0.0,
        );
        Panel::draw(
            ui,
            Vec2::new(c.x - t, c.y - len),
            Vec2::new(c.x + t, c.y + len),
            [1.0, 1.0, 1.0, 0.9],
            0.0,
        );
    }
}
