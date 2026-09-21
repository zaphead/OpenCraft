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
            self.adv_open = false;
        }
        if self.pause_adv.as_ref().is_some_and(|b| b.hit(c)) {
            self.adv_open = !self.adv_open;
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
        while let Ok((pos, mesh, us)) = self.mesh_rx.try_recv() {
            // Drop meshes for chunks that unloaded while meshing: uploading
            // them would resurrect dead terrain for a frame.
            if self.replica.has_chunk(pos) {
                self.pending_upload.push((pos, mesh));
            }
            self.meshes_done += 1;
            self.mesh_ema_ms += (us as f64 / 1000.0 - self.mesh_ema_ms) * 0.1;
            self.in_flight.remove(&pos);
            // An edit landed mid-mesh: the upload above is already stale, so
            // immediately re-queue one fresh job from the current replica.
            if self.dirty.remove(&pos) {
                self.queue_chunk(pos);
            }
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
            ServerPlay::KeepTick { tick } => self.server_tick = tick,
            ServerPlay::Chunk { pos, sections } => {
                replica::apply_chunk(&mut self.replica, pos, &sections);
                self.queue_chunk(pos);
            }
            ServerPlay::UnloadChunk { pos } => {
                self.replica.remove_chunk(pos);
                self.meshed_step.remove(&pos);
                self.pending_remove.push(pos);
            }
            ServerPlay::BlockUpdate { pos, id } => {
                replica::apply_block(&mut self.replica, pos, id);
                self.queue_chunk(pos.chunk());
            }
            ServerPlay::PlayerCorrection {
                pos,
                vel,
                on_ground,
                health,
                fall_distance,
                ride,
                rooted,
                fx,
                adv,
                ..
            } => {
                self.health = health;
                self.ride = ride;
                self.rooted = rooted != 0;
                self.swift = fx & 4 != 0;
                self.glow = fx & 2 != 0;
                self.adv = adv;
                if let Some(p) = &mut self.predict {
                    p.correct(pos, vel, on_ground, health, fall_distance);
                }
                let _ = fx;
            }
            ServerPlay::EntitySpawn {
                id,
                pos,
                item,
                kind,
                hp,
                state,
                variant,
            } => {
                if kind == 1 {
                    self.items.insert(id, (pos, item));
                } else {
                    self.mobs.insert(id, super::MobSeen { kind, pos, yaw: 0.0, hp, state, variant });
                }
            }
            ServerPlay::EntityPos { id, pos, yaw, hp, state, .. } => {
                if let Some(e) = self.items.get_mut(&id) {
                    e.0 = pos;
                }
                if let Some(m) = self.mobs.get_mut(&id) {
                    m.pos = pos;
                    m.yaw = yaw;
                    m.hp = hp;
                    m.state = state;
                }
            }
            ServerPlay::EntityDespawn { id } => {
                self.items.remove(&id);
                self.mobs.remove(&id);
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
                    ParticleKind::Hearts => 6,
                    ParticleKind::Steam => 4,
                    ParticleKind::Spore => 10,
                    ParticleKind::Leaf => 5,
                    ParticleKind::Sting => 4,
                    ParticleKind::Wisp => 3,
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
                    SoundKind::Call | SoundKind::Hum => &self.sounds.pickup,
                };
                let cat = match kind {
                    SoundKind::Hurt | SoundKind::Step | SoundKind::Call => Category::Players,
                    SoundKind::Hum => Category::Music,
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
