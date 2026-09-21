use std::sync::mpsc::{Receiver, Sender};

use engine_core::ChunkPos;
use game::{generate_chunk, spawn_pos};
use protocol::{ClientPlay, ServerPlay};
use world::{Chunk, EntityKind, World};

use crate::embed::ServerConfig;
use crate::session::{snap_inv, Player};
use self::interest::{send_interest_sync, spawn_gen_pool};

pub const VIEW_CHUNKS: i32 = 12;

pub struct Sim {
    pub world: World,
    pub player: Player,
    pub tick: u32,
    pub seed: i64,
    pub view: i32,
    gen_tx: Sender<ChunkPos>,
    gen_rx: Receiver<Chunk>,
}

impl Sim {
    pub fn new(config: ServerConfig) -> Self {
        let seed = config.seed;
        let workers = (std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)).clamp(2, 8);
        let (gen_tx, gen_rx) = spawn_gen_pool(seed, workers);

        let mut world = World::new();
        let spawn = spawn_pos(seed);
        let origin = ChunkPos::from_block(spawn.x as i32, spawn.z as i32);
        for dz in -2..=2 {
            for dx in -2..=2 {
                let p = ChunkPos::new(origin.x + dx, origin.z + dz);
                world.insert_chunk(generate_chunk(seed, p));
            }
        }
        let entity = world.entities.spawn(EntityKind::Player, spawn);
        let player = Player::new(entity, spawn);
        Self {
            world,
            player,
            tick: 0,
            seed,
            view: config.view_chunks,
            gen_tx,
            gen_rx,
        }
    }

    pub fn join_player(&mut self, out: &Sender<ServerPlay>) {
        let loaded: Vec<_> = self.world.loaded_chunks().collect();
        for pos in loaded {
            self.populate(pos, out);
        }
        send_interest_sync(self, out);
        emit(out, ServerPlay::JoinGame {
            entity: self.player.entity,
            seed: self.seed,
            tick: self.tick,
            spawn: self.player.spawn,
            pos: self.player.body.pos,
            health: self.player.health,
            inventory: snap_inv(&self.player),
        });
    }

    pub fn tick(&mut self, inbox: &[ClientPlay], out: &Sender<ServerPlay>) {
        while let Ok(chunk) = self.gen_rx.try_recv() {
            if !self.world.has_chunk(chunk.pos) {
                let pos = chunk.pos;
                self.world.insert_chunk(chunk);
                self.populate(pos, out);
            }
        }
        for pkt in inbox {
            self.handle(pkt, out);
        }
        self.tick = self.tick.wrapping_add(1);
        self.prepare_body();
        self.simulate_player(out);
        self.tick_items(out);
        self.tick_mobs(out);
        self.progress_dig(out);
        self.stream_chunks(out);
        emit(out, ServerPlay::KeepTick { tick: self.tick });
        emit(out, ServerPlay::PlayerCorrection {
            tick: self.tick,
            pos: self.player.body.pos,
            vel: self.player.body.vel,
            on_ground: self.player.body.on_ground,
            health: self.player.health,
            fall_distance: self.player.body.fall_distance,
            ride: life::ride_code(self),
            rooted: if self.player.rooted > 0 { 1 } else { 0 },
            fx: life::fx_bits(self),
            adv: self.player.adv,
        });
        if let Some(i) = self.world.entities.get(self.player.entity) {
            self.world.entities.pos[i] = self.player.body.pos;
            self.world.entities.yaw[i] = self.player.body.yaw;
            self.world.entities.pitch[i] = self.player.body.pitch;
        }
        for (id, i) in self.world.entities.iter_alive().collect::<Vec<_>>() {
            if id == self.player.entity {
                continue;
            }
            emit(out, ServerPlay::EntityPos {
                id,
                pos: self.world.entities.pos[i],
                yaw: self.world.entities.yaw[i],
                pitch: self.world.entities.pitch[i],
                hp: self.world.entities.hp[i],
                state: self.world.entities.state[i],
            });
        }
    }
}

mod action;
mod interest;
mod life;

#[cfg(test)]
mod tests;

pub(crate) fn emit(out: &Sender<ServerPlay>, msg: ServerPlay) {
    out.send(msg).ok();
}
