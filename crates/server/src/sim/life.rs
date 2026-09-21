//! Mob ticks, mounts, fluids, and set-piece population. Server authority.

use engine_core::{BlockPos, ChunkPos, Vec3};
use engine_phys::{Fluid, Ride};
use game::{
    biome_at, boss_name, drops, food_hearts, initial_stamina, initial_state, is_potion, max_hp,
    residents, spawner_kind, surface_y, Biome, ItemId, ADV_DRINK, ADV_HIDE, ADV_MOON, ADV_RIDE,
    ADV_TREE, ADV_WAKE, ADV_WYRM,
};
use protocol::{ParticleKind, ServerPlay, SoundKind};
use world::{solid_id, EntityKind};

use super::{emit, Sim};
use crate::session::snap_inv;

use game::mind::{Sight, ASLEEP, BABY, CALM, FLEE, SHUT};

impl Sim {
    pub(super) fn prepare_body(&mut self) {
        if self.player.adv & ADV_WAKE == 0 {
            self.player.adv |= ADV_WAKE;
        }
        let phase = game::at_tick(self.tick);
        if phase.blood {
            self.player.adv |= ADV_MOON;
        }
        if self.player.rooted > 0 {
            self.player.rooted -= 1;
        }
        self.player.fire = self.player.fire.saturating_sub(1);
        self.player.glow = self.player.glow.saturating_sub(1);
        self.player.swift = self.player.swift.saturating_sub(1);
        if self.player.hurt_cd > 0 {
            self.player.hurt_cd -= 1;
        }
        if self.player.input.sneak {
            if let Some(id) = self.player.mount.take() {
                if let Some(i) = self.world.entities.get(id) {
                    self.world.entities.link[i] = 0;
                }
            }
        }
        let mut input = self.player.input;
        input.ride = Ride::Foot;
        input.rooted = self.player.rooted > 0;
        input.swift = self.player.swift > 0;
        input.fluid = fluid_at(&self.world, self.player.body.pos);
        if let Some(id) = self.player.mount {
            if let Some(i) = self.world.entities.get(id) {
                input.ride = match self.world.entities.kind[i] {
                    EntityKind::Griffin => Ride::Griffin,
                    _ => Ride::Horse,
                };
                if input.ride == Ride::Horse && self.world.entities.stamina[i] == 0 {
                    input.sprint = false;
                }
            } else {
                self.player.mount = None;
            }
        }
        self.player.input = input;
    }

    pub(super) fn after_body(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let pos = self.player.body.pos;
        let feet = BlockPos::from_vec3(pos);
        let under = self.world.block(feet);
        if under == game::blocks::GEYSER {
            self.player.body.vel.y = 1.15;
        }
        if self.tick % 10 == 0 {
            if under == game::blocks::LAVA && self.player.fire == 0 {
                self.hurt(4, out);
            } else if under == game::blocks::SPRING {
                self.player.health = (self.player.health + 1).min(game::MAX_HEALTH);
            } else if thorn_touch(&self.world, pos) {
                self.hurt(2, out);
            }
        }
        if let Some(id) = self.player.mount {
            if let Some(i) = self.world.entities.get(id) {
                self.world.entities.pos[i] = pos;
                self.world.entities.yaw[i] = self.player.body.yaw;
                let sprinting = self.player.input.sprint && self.player.input.forward > 0.0;
                let st = &mut self.world.entities.stamina[i];
                if sprinting {
                    *st = st.saturating_sub(1);
                } else {
                    *st = (*st + 1).min(40);
                }
            }
        }
        let mono = BlockPos::new(26, surface_y(self.seed, 26, -10), -10);
        if pos.distance(mono.center()) < 6.0 && self.tick % 40 == 0 {
            emit(out, ServerPlay::Sound {
                kind: SoundKind::Hum,
                pos: mono.center(),
                block: 0,
            });
        }
    }

    pub(super) fn tick_mobs(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let phase = game::at_tick(self.tick);
        let player = self.player.body.pos;
        let scared = self.world.entities.iter_alive().any(|(_, i)| {
            self.world.entities.kind[i] == EntityKind::Deer && self.world.entities.state[i] == FLEE
        });
        let ids: Vec<_> = self.world.entities.iter_alive().map(|(id, i)| (id, i)).collect();
        for (id, i) in ids {
            if id == self.player.entity || self.world.entities.kind[i] == EntityKind::Item {
                continue;
            }
            if self.world.entities.kind[i] == EntityKind::Projectile {
                self.tick_shot(id, i, out);
                continue;
            }
            let kind = self.world.entities.kind[i];
            let pos = self.world.entities.pos[i];
            if pos.distance(player) > 96.0
                && boss_name(kind).is_none()
                && self.world.entities.tame[i] == 0
                && self.player.mount != Some(id)
            {
                self.world.entities.despawn(id);
                emit(out, ServerPlay::EntityDespawn { id });
                continue;
            }
            let feet = BlockPos::from_vec3(pos);
            let block = self.world.block(feet);
            let sight = Sight {
                tick: self.tick.wrapping_add(i as u32),
                night: phase.night,
                blood: phase.blood,
                pos,
                vel: self.world.entities.vel[i],
                yaw: self.world.entities.yaw[i],
                hp: self.world.entities.hp[i],
                state: self.world.entities.state[i],
                tame: self.world.entities.tame[i],
                stamina: self.world.entities.stamina[i],
                variant: self.world.entities.variant[i],
                timer: self.world.entities.timer[i],
                player,
                dist: pos.distance(player),
                herd: scared && kind == EntityKind::Deer,
                item: nearest_item(&self.world.entities, pos),
                water: block == game::blocks::WATER || block == game::blocks::SPRING,
                lava: block == game::blocks::LAVA,
                grass: grass_near(&self.world, feet),
                flower: flower_near(&self.world, feet),
                ridden: self.player.mount == Some(id),
            };
            let will = game::mob::decide(kind, &sight);
            self.world.entities.state[i] = will.state;
            self.world.entities.stamina[i] = will.stamina;
            self.world.entities.timer[i] = will.timer;
            self.world.entities.yaw[i] = will.yaw;
            if will.call {
                emit(out, ServerPlay::Sound {
                    kind: SoundKind::Call,
                    pos,
                    block: 0,
                });
            }
            if will.hit > 0 && sight.dist < 2.4 && self.player.hurt_cd == 0 {
                self.hurt(will.hit, out);
                self.player.hurt_cd = 10;
            }
            if will.root > 0 {
                self.player.rooted = will.root;
            }
            if will.flower {
                plant_flower(&mut self.world, feet);
            }
            if will.take_item {
                take_nearest_item(&mut self.world, i, out);
            }
            if will.spawn_goblin {
                let behind = pos - Vec3::new(will.yaw.sin(), 0.0, -will.yaw.cos()) * 3.0;
                self.spawn_mob(EntityKind::Goblin, behind, 0, out);
            }
            if will.boom {
                self.boom(i, id, out);
                continue;
            }
            if kind != EntityKind::Imp && block == game::blocks::LAVA && self.tick % 10 == 0 {
                self.damage_mob(i, id, 4, out);
            }
            if !sight.ridden {
                step_mob(&mut self.world, i, will.x, will.z, will.jump, will.fly, kind);
            }
            if self.world.entities.state[i] == BABY {
                self.world.entities.timer[i] = self.world.entities.timer[i].saturating_add(1);
                if self.world.entities.timer[i] > 600 {
                    self.world.entities.state[i] = CALM;
                    self.world.entities.timer[i] = 0;
                }
            }
        }
        self.tick_spawners(out);
    }

    fn tick_shot(&mut self, id: engine_core::EntityId, i: usize, out: &std::sync::mpsc::Sender<ServerPlay>) {
        self.world.entities.age[i] += 1;
        if self.world.entities.age[i] > 40 {
            self.world.entities.despawn(id);
            emit(out, ServerPlay::EntityDespawn { id });
            return;
        }
        let mut vel = self.world.entities.vel[i];
        vel.y -= 0.03;
        let pos = self.world.entities.pos[i] + vel;
        self.world.entities.pos[i] = pos;
        self.world.entities.vel[i] = vel;
        let player = self.player.body.pos;
        for (oid, j) in self.world.entities.iter_alive().collect::<Vec<_>>() {
            if oid == id || self.world.entities.kind[j] == EntityKind::Item || oid == self.player.entity {
                continue;
            }
            if self.world.entities.pos[j].distance(pos) < 1.2 {
                if self.world.entities.kind[j] == EntityKind::Wyrm {
                    self.player.adv |= ADV_WYRM;
                }
                self.damage_mob(j, oid, 4, out);
                self.world.entities.despawn(id);
                emit(out, ServerPlay::EntityDespawn { id });
                return;
            }
        }
        let _ = player;
        let bp = BlockPos::from_vec3(pos);
        if solid_id(self.world.block(bp)) {
            self.world.entities.despawn(id);
            emit(out, ServerPlay::EntityDespawn { id });
        }
    }

    fn boom(&mut self, i: usize, id: engine_core::EntityId, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let pos = self.world.entities.pos[i];
        if pos.distance(self.player.body.pos) < 3.0 {
            self.hurt(6, out);
        }
        let origin = BlockPos::from_vec3(pos);
        for dx in -2i32..=2 {
            for dy in -1i32..=1 {
                for dz in -2i32..=2 {
                    let p = origin.offset(dx, dy, dz);
                    let idb = self.world.block(p);
                    if game::is_flora(idb) || idb == game::blocks::FLOWER || idb == game::blocks::TUFT {
                        self.world.set_block(p, 0);
                        emit(out, ServerPlay::BlockUpdate { pos: p, id: 0 });
                    }
                }
            }
        }
        emit(out, ServerPlay::Particles {
            kind: ParticleKind::Spore,
            pos,
            block: game::blocks::MUSH_CAP,
        });
        self.world.entities.despawn(id);
        emit(out, ServerPlay::EntityDespawn { id });
    }

    pub(super) fn hurt(&mut self, dmg: u8, out: &std::sync::mpsc::Sender<ServerPlay>) {
        self.player.health = self.player.health.saturating_sub(dmg);
        emit(out, ServerPlay::Hurt { health: self.player.health });
        if self.player.health == 0 {
            self.die(out);
        }
    }

    pub(super) fn damage_mob(&mut self, i: usize, id: engine_core::EntityId, dmg: u8, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let kind = self.world.entities.kind[i];
        if self.world.entities.state[i] == SHUT || self.world.entities.state[i] == ASLEEP {
            return;
        }
        if kind == EntityKind::Aurochs || kind == EntityKind::Deer || kind == EntityKind::Rabbit || kind == EntityKind::Horse {
            if self.world.entities.state[i] != game::mind::AGGRO {
                self.world.entities.state[i] = FLEE;
            }
        }
        if kind == EntityKind::Boar {
            self.world.entities.state[i] = game::mind::AGGRO;
        }
        let hp = self.world.entities.hp[i].saturating_sub(dmg);
        self.world.entities.hp[i] = hp;
        if kind == EntityKind::Wyrm {
            self.player.adv |= ADV_WYRM;
        }
        if hp == 0 {
            let pos = self.world.entities.pos[i];
            for (item, n) in drops(kind) {
                let eid = self.world.spawn_item(pos, *item, *n);
                emit(out, ServerPlay::EntitySpawn {
                    id: eid,
                    kind: 1,
                    pos,
                    item: Some((*item, *n)),
                    hp: 0,
                    state: 0,
                    variant: 0,
                });
                if *item == ItemId::HIDE {
                    self.player.adv |= ADV_HIDE;
                }
            }
            if let Some((item, n)) = self.world.entities.item[i] {
                let eid = self.world.spawn_item(pos, item, n);
                emit(out, ServerPlay::EntitySpawn {
                    id: eid,
                    kind: 1,
                    pos,
                    item: Some((item, n)),
                    hp: 0,
                    state: 0,
                    variant: 0,
                });
            }
            self.world.entities.despawn(id);
            emit(out, ServerPlay::EntityDespawn { id });
            if self.player.mount == Some(id) {
                self.player.mount = None;
            }
        }
    }

    pub(super) fn spawn_mob(&mut self, kind: EntityKind, pos: Vec3, variant: u8, out: &std::sync::mpsc::Sender<ServerPlay>) -> engine_core::EntityId {
        let id = self.world.entities.spawn(kind, pos);
        if let Some(i) = self.world.entities.get(id) {
            self.world.entities.hp[i] = max_hp(kind);
            self.world.entities.state[i] = initial_state(kind);
            self.world.entities.stamina[i] = initial_stamina(kind);
            self.world.entities.variant[i] = variant;
        }
        let (hp, state) = self
            .world
            .entities
            .get(id)
            .map(|i| (self.world.entities.hp[i], self.world.entities.state[i]))
            .unwrap_or((1, 0));
        emit(out, ServerPlay::EntitySpawn {
            id,
            kind: kind.code(),
            pos,
            item: None,
            hp,
            state,
            variant,
        });
        id
    }

    pub(super) fn populate(&mut self, pos: ChunkPos, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let wx = pos.x * 16 + 8;
        let wz = pos.z * 16 + 8;
        let biome = biome_at(self.seed, wx, wz);
        let y = surface_y(self.seed, wx, wz) as f32 + 1.0;
        let here = Vec3::new(wx as f32 + 0.5, y, wz as f32 + 0.5);
        if count_kind(&self.world.entities, residents(biome).first().copied().unwrap_or(EntityKind::Rabbit)) < 3 {
            for (n, kind) in residents(biome).iter().enumerate() {
                if n > 1 {
                    break;
                }
                let variant = if *kind == EntityKind::Rabbit && biome == Biome::Frost { 1 } else { 0 };
                self.spawn_mob(*kind, here + Vec3::new(n as f32, 0.0, 0.0), variant, out);
            }
        }
        if biome == Biome::Meadow && !any_kind(&self.world.entities, EntityKind::Sage) {
            self.spawn_mob(EntityKind::Sage, here + Vec3::new(6.0, 0.0, 2.0), 0, out);
        }
        if biome == Biome::Oak && !any_kind(&self.world.entities, EntityKind::Treant) {
            self.spawn_mob(EntityKind::Treant, here, 0, out);
        }
        if biome == Biome::Ash && !any_kind(&self.world.entities, EntityKind::Wyrm) {
            let (cx, cz) = game::biome::cell_center(Biome::Ash);
            let wy = surface_y(self.seed, cx, cz) as f32 + 1.0;
            self.spawn_mob(EntityKind::Wyrm, Vec3::new(cx as f32, wy, cz as f32), 0, out);
        }
        if biome == Biome::Dune && !any_kind(&self.world.entities, EntityKind::Mimic) {
            self.spawn_mob(EntityKind::Mimic, here + Vec3::new(3.0, 0.0, 0.0), 0, out);
        }
        if matches!(biome, Biome::Swamp | Biome::Dune) && !any_kind(&self.world.entities, EntityKind::Sentinel) {
            self.spawn_mob(EntityKind::Sentinel, here + Vec3::new(-2.0, 0.0, 1.0), 0, out);
        }
        if !any_kind(&self.world.entities, EntityKind::Skywhale) && pos.x == 0 && pos.z == 0 {
            self.spawn_mob(EntityKind::Skywhale, Vec3::new(20.0, 90.0, 20.0), 0, out);
        }
        self.seed_chests(pos);
        let _ = out;
    }

    fn seed_chests(&mut self, pos: ChunkPos) {
        let ox = pos.x * 16;
        let oz = pos.z * 16;
        let mut found = Vec::new();
        if let Some(chunk) = self.world.chunk(pos) {
            for y in 0..engine_core::MAX_Y {
                for lz in 0..16u32 {
                    for lx in 0..16u32 {
                        let id = chunk.get(lx, y, lz);
                        if id == game::blocks::CHEST || id == game::blocks::VAULT {
                            found.push((BlockPos::new(ox + lx as i32, y, oz + lz as i32), id));
                        }
                    }
                }
            }
        }
        for (p, id) in found {
            if self.world.chests.contains_key(&p) {
                continue;
            }
            let mut slots = [None; 27];
            let biome = biome_at(self.seed, p.x, p.z);
            if biome == Biome::Mesa {
                slots[0] = Some((ItemId::GOLD, 1));
            } else if id == game::blocks::VAULT {
                slots[0] = Some((ItemId::WYRM_SCALE, 1));
            } else {
                slots[0] = Some((game::blocks::COBBLE, 8));
            }
            self.world.chests.insert(p, slots);
        }
    }

    fn tick_spawners(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        if self.tick % 80 != 0 {
            return;
        }
        let p = self.player.body.pos;
        let origin = BlockPos::from_vec3(p);
        for dx in -8i32..=8 {
            for dy in -3i32..=3 {
                for dz in -8i32..=8 {
                    let bp = origin.offset(dx, dy, dz);
                    if self.world.block(bp) != game::blocks::SPAWNER {
                        continue;
                    }
                    let biome = biome_at(self.seed, bp.x, bp.z);
                    let kind = spawner_kind(biome);
                    if count_kind(&self.world.entities, kind) > 6 {
                        continue;
                    }
                    self.spawn_mob(kind, bp.center() + Vec3::Y, 0, out);
                }
            }
        }
    }

    pub(super) fn try_use(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) -> bool {
        if let Some(held) = self.player.held() {
            if food_hearts(held) > 0 {
                self.eat(held, out);
                return true;
            }
            if is_potion(held) {
                self.drink(held, out);
                return true;
            }
            if held == ItemId::STAFF {
                self.cast(out);
                return true;
            }
        }
        let eye = self.player.body.eye();
        let dir = self.player.body.look_dir();
        let mut best: Option<(usize, f32)> = None;
        for (_, i) in self.world.entities.iter_alive() {
            if self.world.entities.kind[i] == EntityKind::Item || self.world.entities.kind[i] == EntityKind::Player {
                continue;
            }
            let d = self.world.entities.pos[i].distance(eye);
            if d < 4.5 && (best.is_none() || d < best.unwrap().1) {
                let to = self.world.entities.pos[i] - eye;
                if to.normalize_or_zero().dot(dir) > 0.4 {
                    best = Some((i, d));
                }
            }
        }
        let Some((i, _)) = best else { return false };
        let id = self.world.entities.id_at(i);
        let kind = self.world.entities.kind[i];
        let held = self.player.held();
        match kind {
            EntityKind::Glowbeetle => {
                self.give(ItemId::LANTERN, out);
                self.world.entities.despawn(id);
                emit(out, ServerPlay::EntityDespawn { id });
                true
            }
            EntityKind::Horse if self.world.entities.tame[i] == 1 && self.world.entities.state[i] != BABY => {
                self.player.mount = Some(id);
                self.world.entities.link[i] = 1;
                self.player.adv |= ADV_RIDE;
                true
            }
            EntityKind::Griffin => {
                self.player.mount = Some(id);
                self.world.entities.link[i] = 1;
                self.player.adv |= ADV_RIDE;
                true
            }
            EntityKind::Aurochs | EntityKind::Horse if held == Some(ItemId::BEEF) => {
                self.feed(i, kind, out);
                self.consume_held();
                true
            }
            EntityKind::Rabbit if held.is_some_and(|h| h == game::blocks::FLOWER) => {
                self.feed(i, kind, out);
                self.consume_held();
                true
            }
            EntityKind::Sage => {
                self.open_trade(out);
                true
            }
            EntityKind::Mimic if self.world.entities.state[i] == SHUT => {
                self.world.entities.state[i] = game::mind::AGGRO;
                self.hurt(4, out);
                true
            }
            _ => false,
        }
    }

    fn eat(&mut self, id: u16, out: &std::sync::mpsc::Sender<ServerPlay>) {
        self.player.health = (self.player.health + food_hearts(id)).min(game::MAX_HEALTH);
        self.consume_held();
        emit(out, ServerPlay::Hurt { health: self.player.health });
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    fn drink(&mut self, id: u16, out: &std::sync::mpsc::Sender<ServerPlay>) {
        if id == ItemId::POTION_FIRE {
            self.player.fire = 200;
        } else if id == ItemId::POTION_GLOW {
            self.player.glow = 200;
        } else if id == ItemId::POTION_SWIFT {
            self.player.swift = 200;
        } else if id == ItemId::POTION_HEAL {
            self.player.health = game::MAX_HEALTH;
        }
        self.player.adv |= ADV_DRINK;
        self.consume_held();
        emit(out, ServerPlay::Hurt { health: self.player.health });
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    fn cast(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        if self.player.hurt_cd > 0 {
            return;
        }
        self.player.hurt_cd = 8;
        let pos = self.player.body.eye();
        let vel = self.player.body.look_dir() * 0.8;
        let id = self.spawn_mob(EntityKind::Projectile, pos, 0, out);
        if let Some(i) = self.world.entities.get(id) {
            self.world.entities.vel[i] = vel;
        }
    }

    fn feed(&mut self, i: usize, kind: EntityKind, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let pos = self.world.entities.pos[i];
        emit(out, ServerPlay::Particles {
            kind: ParticleKind::Hearts,
            pos: pos + Vec3::Y,
            block: 0,
        });
        if self.world.entities.tame[i] == 0 {
            self.world.entities.tame[i] = 1;
            return;
        }
        let baby = self.spawn_mob(kind, pos + Vec3::new(0.6, 0.0, 0.0), self.world.entities.variant[i], out);
        if let Some(b) = self.world.entities.get(baby) {
            self.world.entities.state[b] = BABY;
            self.world.entities.tame[b] = 1;
        }
    }

    fn give(&mut self, item: u16, out: &std::sync::mpsc::Sender<ServerPlay>) {
        if game::collect_into(&mut self.player.hotbar, &mut self.player.main, (item, 1)).is_some() {
            let eid = self.world.spawn_item(self.player.body.pos, item, 1);
            emit(out, ServerPlay::EntitySpawn {
                id: eid,
                kind: 1,
                pos: self.player.body.pos,
                item: Some((item, 1)),
                hp: 0,
                state: 0,
                variant: 0,
            });
        }
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    fn consume_held(&mut self) {
        let i = self.player.selected as usize;
        if let Some((id, n)) = self.player.hotbar[i] {
            self.player.hotbar[i] = if n <= 1 { None } else { Some((id, n - 1)) };
        }
    }

    pub(super) fn open_trade(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        let mut w = game::Window::player(self.player.hotbar, self.player.main, self.player.selected);
        w.kind = game::WindowKind::Trade;
        w.craft[0] = Some((ItemId::STAFF, 1));
        w.craft[1] = Some((ItemId::KEY, 1));
        w.craft[2] = Some((ItemId::GLOWCAP, 1));
        self.player.window = Some(w);
        emit(out, ServerPlay::OpenWindow { kind: protocol::OpenKind::Trade, pos: None });
        emit(out, ServerPlay::Inventory(snap_inv(&self.player)));
    }

    pub(super) fn strike(&mut self, out: &std::sync::mpsc::Sender<ServerPlay>) {
        if self.player.hurt_cd > 0 {
            return;
        }
        let eye = self.player.body.eye();
        let dir = self.player.body.look_dir();
        let mut best: Option<(usize, f32)> = None;
        for (_, i) in self.world.entities.iter_alive() {
            let kind = self.world.entities.kind[i];
            if kind == EntityKind::Item || kind == EntityKind::Player || kind == EntityKind::Projectile {
                continue;
            }
            let d = self.world.entities.pos[i].distance(eye);
            if d < 4.0 {
                let to = (self.world.entities.pos[i] + Vec3::Y) - eye;
                if to.normalize_or_zero().dot(dir) > 0.5 && best.map(|b| d < b.1).unwrap_or(true) {
                    best = Some((i, d));
                }
            }
        }
        let Some((i, _)) = best else { return };
        self.player.hurt_cd = 8;
        let id = self.world.entities.id_at(i);
        self.player.digging = None;
        self.damage_mob(i, id, 2, out);
    }

    pub(super) fn note_break(&mut self, block: u16) {
        if game::is_log(block) {
            self.player.adv |= ADV_TREE;
        }
        if block == game::blocks::HIVE {
            let pos = self.player.body.pos;
            for (_, i) in self.world.entities.iter_alive().collect::<Vec<_>>() {
                if self.world.entities.kind[i] == EntityKind::Bee && self.world.entities.pos[i].distance(pos) < 16.0 {
                    self.world.entities.state[i] = game::mind::AGGRO;
                }
            }
        }
        if block == game::blocks::CHEST || block == game::blocks::VAULT {
            // looting a chest wakes sentinels
        }
    }

    pub(super) fn wake_sentinels(&mut self) {
        for (_, i) in self.world.entities.iter_alive().collect::<Vec<_>>() {
            if self.world.entities.kind[i] == EntityKind::Sentinel {
                self.world.entities.state[i] = game::mind::AGGRO;
            }
        }
    }
}

pub(super) fn ride_code(sim: &Sim) -> u8 {
    match sim.player.mount.and_then(|id| sim.world.entities.get(id).map(|i| sim.world.entities.kind[i])) {
        Some(EntityKind::Griffin) => 2,
        Some(_) => 1,
        None => 0,
    }
}

pub(super) fn fx_bits(sim: &Sim) -> u8 {
    let mut b = 0u8;
    if sim.player.fire > 0 { b |= 1; }
    if sim.player.glow > 0 { b |= 2; }
    if sim.player.swift > 0 { b |= 4; }
    b
}

fn fluid_at(world: &world::World, pos: Vec3) -> Fluid {
    let feet = world.block(BlockPos::from_vec3(pos));
    let body = world.block(BlockPos::from_vec3(pos + Vec3::Y * 0.8));
    let id = if feet == game::blocks::WATER || feet == game::blocks::LAVA || feet == game::blocks::SPRING {
        feet
    } else {
        body
    };
    match id {
        game::blocks::WATER => Fluid::Water,
        game::blocks::LAVA => Fluid::Lava,
        game::blocks::SPRING => Fluid::Spring,
        _ => Fluid::None,
    }
}

fn thorn_touch(world: &world::World, pos: Vec3) -> bool {
    let feet = BlockPos::from_vec3(pos);
    for dx in -1i32..=1 {
        for dz in -1i32..=1 {
            if world.block(feet.offset(dx, 0, dz)) == game::blocks::THORN
                || world.block(feet.offset(dx, 1, dz)) == game::blocks::THORN
            {
                return true;
            }
        }
    }
    false
}

fn nearest_item(ents: &world::Entities, pos: Vec3) -> Option<Vec3> {
    let mut best: Option<(f32, Vec3)> = None;
    for (_, i) in ents.iter_alive() {
        if ents.kind[i] != EntityKind::Item {
            continue;
        }
        let d = ents.pos[i].distance(pos);
        if d < 8.0 && best.map(|(b, _)| d < b).unwrap_or(true) {
            best = Some((d, ents.pos[i]));
        }
    }
    best.map(|(_, p)| p)
}

fn take_nearest_item(world: &mut world::World, fox: usize, out: &std::sync::mpsc::Sender<ServerPlay>) {
    let pos = world.entities.pos[fox];
    let mut best: Option<(f32, engine_core::EntityId, usize)> = None;
    for (id, i) in world.entities.iter_alive() {
        if world.entities.kind[i] != EntityKind::Item {
            continue;
        }
        let d = world.entities.pos[i].distance(pos);
        if d < 1.4 && best.map(|(b, _, _)| d < b).unwrap_or(true) {
            best = Some((d, id, i));
        }
    }
    if let Some((_, id, i)) = best {
        world.entities.item[fox] = world.entities.item[i];
        world.entities.despawn(id);
        emit(out, ServerPlay::EntityDespawn { id });
    }
}

fn grass_near(world: &world::World, feet: BlockPos) -> bool {
    for dx in -1i32..=1 {
        for dz in -1i32..=1 {
            let g = feet.offset(dx, -1, dz);
            if world.block(g) == game::blocks::GRASS && world.block(feet.offset(dx, 0, dz)) == 0 {
                return true;
            }
        }
    }
    false
}

fn flower_near(world: &world::World, feet: BlockPos) -> bool {
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            let id = world.block(feet.offset(dx, 0, dz));
            if id == game::blocks::FLOWER || id == game::blocks::SUNPETAL {
                return true;
            }
        }
    }
    false
}

fn plant_flower(world: &mut world::World, feet: BlockPos) {
    for dx in -1i32..=1 {
        for dz in -1i32..=1 {
            let ground = feet.offset(dx, -1, dz);
            let above = feet.offset(dx, 0, dz);
            if world.block(ground) == game::blocks::GRASS && world.block(above) == 0 {
                world.set_block(above, game::blocks::FLOWER);
                return;
            }
        }
    }
}

fn step_mob(world: &mut world::World, i: usize, wish_x: f32, wish_z: f32, jump: f32, fly: bool, kind: EntityKind) {
    let mut pos = world.entities.pos[i];
    let mut vel = world.entities.vel[i];
    vel.x = wish_x;
    vel.z = wish_z;
    let ground_y = surface_y_at(world, pos);
    if fly {
        let cruise = if kind == EntityKind::Skywhale { 90.0 } else { ground_y + 5.0 };
        vel.y = (cruise - pos.y).clamp(-0.25, 0.2);
    } else if kind == EntityKind::Fish && world.block(BlockPos::from_vec3(pos)) == game::blocks::WATER {
        vel.y = 0.0;
    } else {
        vel.y -= 0.06;
        let below = BlockPos::from_vec3(Vec3::new(pos.x, pos.y - 0.05, pos.z));
        let grounded = solid_id(world.block(below));
        if grounded && vel.y < 0.0 {
            vel.y = 0.0;
            pos.y = below.y as f32 + 1.0;
        }
        if jump > 0.0 && grounded {
            vel.y = jump;
        }
    }
    let next = pos + vel;
    let ahead = BlockPos::from_vec3(Vec3::new(next.x, pos.y + 0.2, next.z));
    if solid_id(world.block(ahead)) && !fly {
        vel.x = 0.0;
        vel.z = 0.0;
    } else {
        pos.x = next.x;
        pos.z = next.z;
    }
    let ny = pos.y + vel.y;
    let head = BlockPos::from_vec3(Vec3::new(pos.x, ny + 0.2, pos.z));
    if solid_id(world.block(head)) && vel.y > 0.0 {
        vel.y = 0.0;
    } else {
        pos.y = ny.max(1.0);
    }
    world.entities.pos[i] = pos;
    world.entities.vel[i] = vel;
}

fn surface_y_at(world: &world::World, pos: Vec3) -> f32 {
    let x = pos.x.floor() as i32;
    let z = pos.z.floor() as i32;
    for y in (0..engine_core::MAX_Y).rev() {
        if solid_id(world.block(BlockPos::new(x, y, z))) {
            return y as f32 + 1.0;
        }
    }
    40.0
}

fn count_kind(ents: &world::Entities, kind: EntityKind) -> usize {
    ents.iter_alive().filter(|(_, i)| ents.kind[*i] == kind).count()
}

fn any_kind(ents: &world::Entities, kind: EntityKind) -> bool {
    count_kind(ents, kind) > 0
}
