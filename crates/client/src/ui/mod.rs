mod advance;
mod container;
mod font;
mod hud;
mod pause;
mod prim;

pub use advance::AdvancementScreen;
pub use container::ContainerScreen;
pub use hud::HudLayer;
pub use pause::PauseMenu;
pub use prim::{Button, Panel};

use engine_core::Vec2;
use engine_render::UiQuad;

pub struct UiFrame {
    pub quads: Vec<UiQuad>,
    pub scale: f32,
    pub size: Vec2,
}

impl UiFrame {
    pub fn new(w: f32, h: f32) -> Self {
        let scale = ((h / 240.0).floor() as i32).clamp(2, 4) as f32;
        Self {
            quads: Vec::new(),
            scale,
            size: Vec2::new(w, h),
        }
    }
}

pub fn darken(ui: &mut UiFrame) {
    ui.quads.push(UiQuad {
        min: Vec2::ZERO,
        max: ui.size,
        uv_min: Vec2::ZERO,
        uv_max: Vec2::new(1.0 / 256.0, 1.0 / 256.0),
        color: [0.0, 0.0, 0.0, 0.45],
        z: 0.2,
    });
}
