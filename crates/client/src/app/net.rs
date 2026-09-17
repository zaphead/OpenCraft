use std::time::Instant;

use engine_audio::Category;
use protocol::{ClientPlay, ParticleKind, ServerPlay, SoundKind};
use winit::window::CursorGrabMode;

use super::App;
use crate::atlas::block_tile;
use crate::particles;
use crate::predict::Predict;
use crate::replica;

impl App {
    pub(super) fn grab(&self, on: bool) {
        let Some(w) = &self.window else { return };
        if on {
            w.set_cursor_grab(CursorGrabMode::Locked).ok();
            w.set_cursor_visible(false);
        } else {
            w.set_cursor_grab(CursorGrabMode::None).ok();
            w.set_cursor_visible(true);
        }
    }

    pub(super) fn send(&self, p: ClientPlay) {
        if let Some(s) = &self.server {
            s.to_sim.send(p).ok();
        }
    }

    pub(super) fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
        if let Some(s) = &self.server {
            s.set_paused(paused);
        }
        self.grab(!paused);
    }

    /// Pause-menu clicks. Returns true when the user confirmed quit.
    pub(super) fn pause_clicks(&mut self) -> bool {
        let c = self.snap.cursor;
        if self.confirm_quit {
            if self.pause_quit.as_ref().is_some_and(|q| q.hit(c)) {
                return true;
            }
            if self.pause_back.as_ref().is_some_and(|b| b.hit(c)) {
                self.confirm_quit = false;
            }
            return false;
        }
        if self.pause_back.as_ref().is_some_and(|b| b.hit(c)) {
            self.set_paused(false);
        }
        if self.pause_quit.as_ref().is_some_and(|q| q.hit(c)) {
            self.confirm_quit = true;
        }
        false
    }

    pub(super) fn pump(&mut self) {
        let mut msgs = Vec::new();
        if let Some(s) = &self.server {
            while let Ok(m) = s.from_sim.try_recv() {
                msgs.push(m);
            }
        }
        for m in msgs {
            self.handle_server(m);
        }
        while let Ok(job) = self.mesh_rx.try_recv() {
            self.pending_upload.push(job);
        }
    }

    fn handle_server(&mut self, m: ServerPlay) {
        match m {
            ServerPlay::JoinGame {
                seed,
                pos,
                health,
                inventory,
                spawn,
                ..
            } => {
                self.seed = seed;
                self.spawn = spawn;
                self.predict = Some(Predict::new(pos));
                self.health = health;
                self.inv = inventory;
                self.last_tick = Instant::now();
            }
            ServerPlay::KeepTick { .. } => {}
            ServerPlay::Chunk { pos, sections } => {
                replica::apply_chunk(&mut self.replica, pos, &sections);
                if let Some(snap) = self.replica.snapshot(pos) {
                    self.mesh_tx.send(snap).ok();
                }
            }
            ServerPlay::UnloadChunk { pos } => {
                self.replica.remove_chunk(pos);
                self.pending_remove.push(pos);
            }
            ServerPlay::BlockUpdate { pos, id } => {
                replica::apply_block(&mut self.replica, pos, id);
                if let Some(snap) = self.replica.snapshot(pos.chunk()) {
                    self.mesh_tx.send(snap).ok();
                }
            }
            ServerPlay::PlayerCorrection {
                pos,
                vel,
                on_ground,
                health,
                fall_distance,
                ..
            } => {
                self.health = health;
                if let Some(p) = &mut self.predict {
                    p.correct(pos, vel, on_ground, health, fall_distance);
                }
            }
            ServerPlay::EntitySpawn { id, pos, item, kind } => {
                if kind == 1 {
                    self.items.insert(id, (pos, item));
                }
            }
            ServerPlay::EntityPos { id, pos, .. } => {
                if let Some(e) = self.items.get_mut(&id) {
                    e.0 = pos;
                }
            }
            ServerPlay::EntityDespawn { id } => {
                self.items.remove(&id);
            }
            ServerPlay::Inventory(inv) => self.inv = inv,
            ServerPlay::OpenWindow { kind, .. } => {
                self.overlay = Some(kind);
                self.grab(false);
            }
            ServerPlay::CloseWindow => {
                self.overlay = None;
                if !self.paused {
                    self.grab(true);
                }
            }
            ServerPlay::Particles { kind, pos, block } => {
                let tile = block_tile(block, 4);
                let n = match kind {
                    ParticleKind::Break => 16,
                    ParticleKind::Place => 8,
                };
                particles::burst(&mut self.particles, pos, tile, n, 0.08);
            }
            ServerPlay::Sound { kind, pos, .. } => {
                let ids = match kind {
                    SoundKind::Step => &self.sounds.step,
                    SoundKind::Break => &self.sounds.break_b,
                    SoundKind::Place => &self.sounds.place,
                    SoundKind::Hurt => &self.sounds.hurt,
                    SoundKind::Pickup => &self.sounds.pickup,
                };
                let cat = match kind {
                    SoundKind::Hurt | SoundKind::Step => Category::Players,
                    SoundKind::Break | SoundKind::Place | SoundKind::Pickup => Category::Master,
                };
                if let Some(id) = ids.first() {
                    self.mixer.play(*id, pos, cat);
                }
            }
            ServerPlay::Hurt { health } => self.health = health,
            ServerPlay::Respawned { pos, health, inventory } => {
                self.health = health;
                self.inv = inventory;
                self.predict = Some(Predict::new(pos));
            }
        }
    }

}
