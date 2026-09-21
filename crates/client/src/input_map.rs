use engine_core::Vec2;
use engine_input::Snapshot;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};

pub fn apply_key(snap: &mut Snapshot, event: &KeyEvent) {
    let PhysicalKey::Code(code) = event.physical_key else {
        return;
    };
    let down = event.state == ElementState::Pressed;
    match code {
        KeyCode::KeyW => snap.forward = down,
        KeyCode::KeyS => snap.back = down,
        KeyCode::KeyA => snap.left = down,
        KeyCode::KeyD => snap.right = down,
        KeyCode::Space => snap.jump = down,
        KeyCode::ShiftLeft | KeyCode::ShiftRight => {
            snap.sneak = down;
            snap.shift = down;
        }
        KeyCode::ControlLeft | KeyCode::ControlRight => snap.sprint = down,
        KeyCode::KeyE => snap.inventory = down,
        KeyCode::Escape => snap.pause = down,
        KeyCode::F2 => snap.debug = down,
        KeyCode::F5 => snap.perspective = down,
        KeyCode::KeyC => snap.zoom = down,
        KeyCode::F12 => snap.screenshot = down,
        KeyCode::Digit1 => {
            if down {
                snap.hotbar = Some(0);
            }
        }
        KeyCode::Digit2 => {
            if down {
                snap.hotbar = Some(1);
            }
        }
        KeyCode::Digit3 => {
            if down {
                snap.hotbar = Some(2);
            }
        }
        KeyCode::Digit4 => {
            if down {
                snap.hotbar = Some(3);
            }
        }
        KeyCode::Digit5 => {
            if down {
                snap.hotbar = Some(4);
            }
        }
        KeyCode::Digit6 => {
            if down {
                snap.hotbar = Some(5);
            }
        }
        KeyCode::Digit7 => {
            if down {
                snap.hotbar = Some(6);
            }
        }
        KeyCode::Digit8 => {
            if down {
                snap.hotbar = Some(7);
            }
        }
        KeyCode::Digit9 => {
            if down {
                snap.hotbar = Some(8);
            }
        }
        _ => {}
    }
}

pub fn apply_mouse_button(snap: &mut Snapshot, button: MouseButton, state: ElementState) {
    let down = state == ElementState::Pressed;
    match button {
        MouseButton::Left => {
            snap.attack = down;
            snap.left_click = down;
        }
        MouseButton::Right => {
            snap.use_item = down;
            snap.right_click = down;
        }
        MouseButton::Middle => {
            snap.pick = down;
            snap.pick_click = down;
        }
        _ => {}
    }
}

pub fn apply_scroll(snap: &mut Snapshot, delta: MouseScrollDelta) {
    snap.scroll += match delta {
        MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
        MouseScrollDelta::PixelDelta(p) => p.y.signum() as i32,
    };
}

pub fn apply_cursor(snap: &mut Snapshot, pos: PhysicalPosition<f64>) {
    snap.cursor = Vec2::new(pos.x as f32, pos.y as f32);
    snap.cursor_in_window = true;
}
