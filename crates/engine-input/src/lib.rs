//! Our input snapshot. No winit types.

use engine_core::Vec2;

#[derive(Clone, Copy, Debug, Default)]
pub struct Snapshot {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sneak: bool,
    pub sprint: bool,
    pub inventory: bool,
    pub pause: bool,
    pub debug: bool,
    pub perspective: bool,
    pub attack: bool,
    pub use_item: bool,
    pub pick: bool,
    pub zoom: bool,
    pub screenshot: bool,
    pub hotbar: Option<u8>,
    pub scroll: i32,
    pub look_delta: Vec2,
    pub cursor: Vec2,
    pub cursor_in_window: bool,
    pub left_click: bool,
    pub right_click: bool,
    pub pick_click: bool,
    pub shift: bool,
}

impl Snapshot {
    pub fn move_axes(self) -> (f32, f32) {
        let mut f = 0.0;
        let mut s = 0.0;
        if self.forward {
            f += 1.0;
        }
        if self.back {
            f -= 1.0;
        }
        if self.left {
            s += 1.0;
        }
        if self.right {
            s -= 1.0;
        }
        (f, s)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Edges {
    pub inventory: bool,
    pub pause: bool,
    pub debug: bool,
    pub perspective: bool,
    pub left_press: bool,
    pub right_press: bool,
    pub left_release: bool,
    pub right_release: bool,
    pub pick_press: bool,
    pub screenshot_press: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Tracker {
    prev: Snapshot,
}

impl Tracker {
    pub fn edges(&mut self, now: Snapshot) -> Edges {
        let e = Edges {
            inventory: now.inventory && !self.prev.inventory,
            pause: now.pause && !self.prev.pause,
            debug: now.debug && !self.prev.debug,
            perspective: now.perspective && !self.prev.perspective,
            left_press: now.left_click && !self.prev.left_click,
            right_press: now.right_click && !self.prev.right_click,
            left_release: !now.left_click && self.prev.left_click,
            right_release: !now.right_click && self.prev.right_click,
            pick_press: now.pick_click && !self.prev.pick_click,
            screenshot_press: now.screenshot && !self.prev.screenshot,
        };
        self.prev = now;
        e
    }
}
