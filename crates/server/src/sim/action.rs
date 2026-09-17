use std::sync::mpsc::Sender;

use engine_core::{BlockPos, Vec3};
use engine_phys::{take_landed_fall, tick_body, MoveInput, VoxelSolid};
use game::{
    apply_fall_damage, break_ticks, click_slot, collect_into, drop_all, face_offset, harvest_drop,
    is_solid, Window, WindowKind, MAX_HEALTH,
};
use protocol::{ClientPlay, OpenKind, ParticleKind, ServerPlay, SoundKind};
use world::EntityKind;

use super::{emit, Sim};
use crate::session::{snap_inv, Player};

impl Sim {
    pub(super) fn handle(&mut self, pkt: &ClientPlay, out: &Sender<ServerPlay>) {
        match pkt {
            ClientPlay::TickInput {
                tick,
                yaw,
                pitch,
                forward,
                strafe,
                jump,
                sneak,
                sprint,
            } => {
                self.player.last_tick = *tick;
                self.player.input = MoveInput {
                    forward: *forward,
                    strafe: *strafe,
                    jump: *jump,
                    sneak: *sneak,
                    sprint: *sprint,
                    yaw: *yaw,
                    pitch: *pitch,
                };
                self.player.body.yaw = *yaw;
                self.player.body.pitch = *pitch;
            }
            ClientPlay::StartDig { pos } => {
                if self.player.window.is_some() {
                    return;
                }
                if !in_reach(&self.player, *pos) {
                    return;
                }
                self.player.digging = Some((*pos, 0));
            }
            ClientPlay::CancelDig => {
                self.player.digging = None;
            }
            ClientPlay::Place {
                against,
                face,
                hotbar,
            } => self.place(*against, *face, *hotbar, out),
            ClientPlay::UseBlock { pos } => self.use_block(*pos, out),
            ClientPlay::SelectHotbar { slot } => {
                self.player.selected = (*slot).min(8);
                if let Some(w) = &mut self.player.window {
                    w.selected = self.player.selected;
                }
            }
            ClientPlay::ClickSlot { slot, button, shift } => {
                if let Some(mut w) = self.player.window.take() {
                    click_slot(&mut w, *slot, *button, *shift);
                    sync_player_from_window(&mut self.player, &w);
                    persist_chest(self, &w);
                    self.player.window = Some(w);
                    emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
                }
            }
            ClientPlay::CloseWindow => self.close_window(out),
            ClientPlay::OpenInventory => {
                if self.player.window.is_some() {
                    self.close_window(out);
                    return;
                }
                let w = Window::player(self.player.hotbar, self.player.main, self.player.selected);
                self.player.window = Some(w);
                emit(out, ServerPlay::OpenWindow {
                    kind: OpenKind::Inventory,
                    pos: None,
                });
                emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
            }
        }
    }

    pub(super) fn simulate_player(&mut self, out: &Sender<ServerPlay>) {
        if self.player.window.is_none() {
            tick_body(&mut self.player.body, self.player.input, &self.world);
        }
        let landed = self.player.body.on_ground && !self.player.last_on_ground;
        self.player.last_on_ground = self.player.body.on_ground;
        if landed {
            let fall = take_landed_fall(&mut self.player.body);
            let before = self.player.health;
            self.player.health = apply_fall_damage(self.player.health, fall);
            if self.player.health < before {
                emit(out, ServerPlay::Hurt {
                    health: self.player.health,
                });
                emit(out, ServerPlay::Sound {
                    kind: SoundKind::Hurt,
                    pos: self.player.body.pos,
                    block: 0,
                });
            }
        }
        if self.player.body.on_ground {
            let speed = Vec3::new(self.player.body.vel.x, 0.0, self.player.body.vel.z).length();
            self.player.step_accum += speed;
            if self.player.step_accum > 2.2 {
                self.player.step_accum = 0.0;
                let feet = BlockPos::from_vec3(self.player.body.pos - Vec3::new(0.0, 0.1, 0.0));
                emit(out, ServerPlay::Sound {
                    kind: SoundKind::Step,
                    pos: self.player.body.pos,
                    block: self.world.block(feet),
                });
            }
        }
        if self.player.health == 0 {
            self.die(out);
        }
        if self.player.body.pos.y < -8.0 {
            self.player.health = 0;
            self.die(out);
        }
    }

    fn die(&mut self, out: &Sender<ServerPlay>) {
        let pos = self.player.body.pos + Vec3::new(0.0, 0.4, 0.0);
        let mut drops = drop_all(&mut self.player.hotbar, &mut self.player.main);
        if let Some(w) = self.player.window.take() {
            self.player.hotbar = w.hotbar;
            self.player.main = w.main;
            drops.extend(drop_all(&mut self.player.hotbar, &mut self.player.main));
            for s in w.craft.iter().flatten() {
                drops.push(*s);
            }
            if let Some(c) = w.cursor {
                drops.push(c);
            }
            emit(out, ServerPlay::CloseWindow);
        }
        for (id, n) in drops {
            let eid = self.world.spawn_item(pos, id, n);
            emit(out, ServerPlay::EntitySpawn {
                id: eid,
                kind: 1,
                pos,
                item: Some((id, n)),
            });
        }
        self.player.body = engine_phys::Body::at(self.player.spawn);
        self.player.health = MAX_HEALTH;
        self.player.digging = None;
        emit(out, ServerPlay::Respawned {
            pos: self.player.spawn,
            health: MAX_HEALTH,
            inventory: snap_inv(&self.player),
        });
    }

    pub(super) fn progress_dig(&mut self, out: &Sender<ServerPlay>) {
        let Some((pos, t)) = self.player.digging else {
            return;
        };
        if !in_reach(&self.player, pos) {
            self.player.digging = None;
            return;
        }
        let block = self.world.block(pos);
        if !is_solid(block) {
            self.player.digging = None;
            return;
        }
        let need = break_ticks(block, self.player.held());
        let t = t + 1;
        if t >= need {
            self.player.digging = None;
            self.break_block(pos, block, out);
        } else {
            self.player.digging = Some((pos, t));
        }
    }

    fn break_block(&mut self, pos: BlockPos, block: u16, out: &Sender<ServerPlay>) {
        if block == game::blocks::CHEST {
            if let Some(slots) = self.world.chests.remove(&pos) {
                for s in slots.into_iter().flatten() {
                    let eid = self.world.spawn_item(pos.center(), s.0, s.1);
                    emit(out, ServerPlay::EntitySpawn {
                        id: eid,
                        kind: 1,
                        pos: pos.center(),
                        item: Some(s),
                    });
                }
            }
        }
        self.world.set_block(pos, 0);
        emit(out, ServerPlay::BlockUpdate { pos, id: 0 });
        emit(out, ServerPlay::Particles {
            kind: ParticleKind::Break,
            pos: pos.center(),
            block,
        });
        emit(out, ServerPlay::Sound {
            kind: SoundKind::Break,
            pos: pos.center(),
            block,
        });
        if let Some((id, n)) = harvest_drop(block, self.player.held()) {
            let eid = self.world.spawn_item(pos.center(), id, n);
            emit(out, ServerPlay::EntitySpawn {
                id: eid,
                kind: 1,
                pos: pos.center(),
                item: Some((id, n)),
            });
        }
    }

    fn place(&mut self, against: BlockPos, face: u8, hotbar: u8, out: &Sender<ServerPlay>) {
        if self.player.window.is_some() {
            return;
        }
        self.player.selected = hotbar.min(8);
        let dest = {
            let (dx, dy, dz) = face_offset(face);
            against.offset(dx, dy, dz)
        };
        if !in_reach(&self.player, dest) && !in_reach(&self.player, against) {
            return;
        }
        if is_solid(self.world.block(dest)) {
            return;
        }
        let Some((item, n)) = self.player.hotbar[self.player.selected as usize] else {
            return;
        };
        let Some(block) = game::as_block(item) else {
            return;
        };
        let feet = self.player.body.aabb();
        let place_box = engine_phys::Aabb {
            min: Vec3::new(dest.x as f32, dest.y as f32, dest.z as f32),
            max: Vec3::new(dest.x as f32 + 1.0, dest.y as f32 + 1.0, dest.z as f32 + 1.0),
        };
        if feet.intersects(place_box) {
            return;
        }
        self.world.set_block(dest, block);
        if block == game::blocks::CHEST {
            self.world.chests.insert(dest, [None; 27]);
        }
        let left = n.saturating_sub(1);
        self.player.hotbar[self.player.selected as usize] = if left == 0 { None } else { Some((item, left)) };
        emit(out, ServerPlay::BlockUpdate { pos: dest, id: block });
        emit(out, ServerPlay::Particles {
            kind: ParticleKind::Place,
            pos: dest.center(),
            block,
        });
        emit(out, ServerPlay::Sound {
            kind: SoundKind::Place,
            pos: dest.center(),
            block,
        });
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    fn use_block(&mut self, pos: BlockPos, out: &Sender<ServerPlay>) {
        if self.player.body.sneaking {
            return;
        }
        let id = self.world.block(pos);
        if id == game::blocks::CRAFTING_TABLE {
            let mut w = Window::player(self.player.hotbar, self.player.main, self.player.selected);
            w.kind = WindowKind::CraftingTable;
            self.player.window = Some(w);
            emit(out, ServerPlay::OpenWindow {
                kind: OpenKind::CraftingTable,
                pos: Some(pos),
            });
            emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
        } else if id == game::blocks::CHEST {
            let mut w = Window::player(self.player.hotbar, self.player.main, self.player.selected);
            w.kind = WindowKind::Chest;
            w.chest = *self.world.chests.entry(pos).or_insert([None; 27]);
            self.player.chest_pos = Some(pos);
            self.player.window = Some(w);
            emit(out, ServerPlay::OpenWindow {
                kind: OpenKind::Chest,
                pos: Some(pos),
            });
            emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
        }
    }

    fn close_window(&mut self, out: &Sender<ServerPlay>) {
        if let Some(w) = self.player.window.take() {
            sync_player_from_window(&mut self.player, &w);
            persist_chest(self, &w);
            for s in w.craft.iter().flatten() {
                if let Some(rest) = collect_into(&mut self.player.hotbar, &mut self.player.main, *s) {
                    let eid = self.world.spawn_item(self.player.body.pos + Vec3::Y, rest.0, rest.1);
                    emit(out, ServerPlay::EntitySpawn {
                        id: eid,
                        kind: 1,
                        pos: self.player.body.pos + Vec3::Y,
                        item: Some(rest),
                    });
                }
            }
            if let Some(c) = w.cursor {
                if let Some(rest) = collect_into(&mut self.player.hotbar, &mut self.player.main, c) {
                    let eid = self.world.spawn_item(self.player.body.pos + Vec3::Y, rest.0, rest.1);
                    emit(out, ServerPlay::EntitySpawn {
                        id: eid,
                        kind: 1,
                        pos: self.player.body.pos + Vec3::Y,
                        item: Some(rest),
                    });
                }
            }
        }
        self.player.chest_pos = None;
        emit(out, ServerPlay::CloseWindow);
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    pub(super) fn tick_items(&mut self, out: &Sender<ServerPlay>) {
        let ids: Vec<_> = self.world.entities.iter_alive().collect();
        for (id, i) in ids {
            if self.world.entities.kind[i] != EntityKind::Item {
                continue;
            }
            self.world.entities.age[i] += 1;
            if self.world.entities.pickup_delay[i] > 0 {
                self.world.entities.pickup_delay[i] -= 1;
            }
            let mut vel = self.world.entities.vel[i];
            vel.y -= 0.04;
            vel.y *= 0.98;
            vel.x *= 0.98;
            vel.z *= 0.98;
            let mut p = self.world.entities.pos[i] + vel;
            let feet = BlockPos::from_vec3(p);
            if self.world.solid(feet.x, feet.y, feet.z) {
                p.y = feet.y as f32 + 1.0;
                vel.y = 0.0;
            }
            self.world.entities.pos[i] = p;
            self.world.entities.vel[i] = vel;
            let player_pos = self.player.body.pos + Vec3::new(0.0, 0.9, 0.0);
            if self.world.entities.pickup_delay[i] == 0 && p.distance(player_pos) < 1.2 {
                if let Some(st) = self.world.entities.item[i] {
                    if let Some(rest) = collect_into(&mut self.player.hotbar, &mut self.player.main, st) {
                        self.world.entities.item[i] = Some(rest);
                    } else {
                        self.world.entities.despawn(id);
                        emit(out, ServerPlay::EntityDespawn { id });
                        emit(out, ServerPlay::Sound {
                            kind: SoundKind::Pickup,
                            pos: p,
                            block: 0,
                        });
                        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
                    }
                }
            }
        }
    }

}


fn in_reach(p: &Player, pos: BlockPos) -> bool {
    let e = p.body.eye();
    e.distance(pos.center()) < 6.0
}

fn sync_player_from_window(p: &mut Player, w: &Window) {
    p.hotbar = w.hotbar;
    p.main = w.main;
    p.selected = w.selected;
}

fn persist_chest(sim: &mut Sim, w: &Window) {
    if w.kind == WindowKind::Chest {
        if let Some(pos) = sim.player.chest_pos {
            sim.world.chests.insert(pos, w.chest);
        }
    }
}
