use std::time::Instant;

use engine_phys::hit_voxel;
use protocol::ClientPlay;

use super::App;
use crate::ui::ContainerScreen;
use game::blocks;

impl App {
    pub(super) fn tick_predict(&mut self) {
        if self.paused || self.overlay.is_some() {
            return;
        }
        let Some(p) = &mut self.predict else { return };
        let (fwd, strafe) = self.snap.move_axes();
        let input = engine_phys::MoveInput {
            forward: fwd,
            strafe,
            jump: self.snap.jump,
            sneak: self.snap.sneak,
            sprint: self.snap.sprint,
            yaw: p.body.yaw,
            pitch: p.body.pitch,
        };
        p.step(input, &self.replica);
        let pkt = ClientPlay::TickInput {
            tick: p.local_tick,
            yaw: p.body.yaw,
            pitch: p.body.pitch,
            forward: fwd,
            strafe,
            jump: self.snap.jump,
            sneak: self.snap.sneak,
            sprint: self.snap.sprint,
        };
        self.last_tick = Instant::now();
        self.send(pkt);
    }

    pub(super) fn look(&mut self, dx: f64, dy: f64) {
        if self.paused || self.overlay.is_some() {
            return;
        }
        if let Some(p) = &mut self.predict {
            p.body.yaw += dx as f32 * self.look_sens;
            p.body.pitch += dy as f32 * self.look_sens;
            p.body.pitch = p.body.pitch.clamp(-1.535, 1.535);
        }
    }

    pub(super) fn play_clicks(&mut self, e: engine_input::Edges) {
        if self.overlay.is_some() || self.paused {
            return;
        }
        let Some(p) = &self.predict else { return };
        let hit = hit_voxel(p.body.eye(), p.body.look_dir(), 5.0, |b| game::is_solid(self.replica.block(b)));
        if e.left_press {
            if let Some(h) = &hit {
                self.send(ClientPlay::StartDig { pos: h.pos });
                self.digging = Some((h.pos, Instant::now(), self.replica.block(h.pos)));
            }
        }
        if e.left_release {
            self.send(ClientPlay::CancelDig);
            self.digging = None;
        }
        if self.snap.attack {
            if let Some(h) = &hit {
                if self.digging.map(|d| d.0) != Some(h.pos) {
                    self.send(ClientPlay::StartDig { pos: h.pos });
                    self.digging = Some((h.pos, Instant::now(), self.replica.block(h.pos)));
                }
            }
        }
        if e.right_press {
            if let Some(h) = &hit {
                let id = self.replica.block(h.pos);
                if !self.snap.sneak && (id == blocks::CRAFTING_TABLE || id == blocks::CHEST) {
                    self.send(ClientPlay::UseBlock { pos: h.pos });
                } else {
                    self.send(ClientPlay::Place {
                        against: h.pos,
                        face: h.face,
                        hotbar: self.inv.selected,
                    });
                }
            }
        }
    }

    pub(super) fn overlay_clicks(&mut self, e: engine_input::Edges) {
        if self.overlay.is_none() {
            return;
        }
        if e.left_press || e.right_press {
            if let Some(slot) = ContainerScreen::hit(&self.slot_hits, self.snap.cursor) {
                self.send(ClientPlay::ClickSlot {
                    slot,
                    button: if e.right_press { 1 } else { 0 },
                    shift: self.snap.shift,
                });
            }
        }
    }

}
