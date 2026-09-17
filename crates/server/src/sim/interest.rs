use std::sync::mpsc::{self, Receiver, Sender};

use engine_core::ChunkPos;
use game::generate_chunk;
use protocol::ServerPlay;
use world::Chunk;

use super::{emit, Sim};

impl Sim {
    pub(super) fn stream_chunks(&mut self, out: &Sender<ServerPlay>) {
        let center = ChunkPos::from_block(
            self.player.body.pos.x.floor() as i32,
            self.player.body.pos.z.floor() as i32,
        );
        let view = self.view;
        let mut want = engine_core::FxHashSet::default();
        for dz in -view..=view {
            for dx in -view..=view {
                want.insert(ChunkPos::new(center.x + dx, center.z + dz));
            }
        }
        for pos in want.iter() {
            if self.player.known.contains(pos) {
                continue;
            }
            if let Some(chunk) = self.world.chunk(*pos) {
                emit(out, chunk_packet(chunk));
                self.player.known.insert(*pos);
            } else {
                self.gen_tx.send(*pos).ok();
            }
        }
        let stale: Vec<_> = self
            .player
            .known
            .iter()
            .copied()
            .filter(|p| !want.contains(p))
            .collect();
        for pos in stale {
            self.player.known.remove(&pos);
            emit(out, ServerPlay::UnloadChunk { pos });
        }
    }
}

pub(super) fn send_interest_sync(sim: &mut Sim, out: &Sender<ServerPlay>) {
    let center = ChunkPos::from_block(
        sim.player.body.pos.x.floor() as i32,
        sim.player.body.pos.z.floor() as i32,
    );
    for dz in -2..=2 {
        for dx in -2..=2 {
            let pos = ChunkPos::new(center.x + dx, center.z + dz);
            if let Some(chunk) = sim.world.chunk(pos) {
                emit(out, chunk_packet(chunk));
                sim.player.known.insert(pos);
            }
        }
    }
}

fn chunk_packet(chunk: &world::Chunk) -> ServerPlay {
    let mut sections = Vec::new();
    for (i, sec) in chunk.sections.iter().enumerate() {
        if sec.is_empty() {
            continue;
        }
        sections.push((i as u8, sec.palette().to_vec(), sec.indices().to_vec()));
    }
    ServerPlay::Chunk {
        pos: chunk.pos,
        sections,
    }
}

pub(super) fn spawn_gen_pool(seed: i64, workers: usize) -> (Sender<ChunkPos>, Receiver<Chunk>) {
    let (job_tx, job_rx) = mpsc::channel::<ChunkPos>();
    let (out_tx, out_rx) = mpsc::channel::<Chunk>();
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));
    for i in 0..workers.max(1) {
        let rx = std::sync::Arc::clone(&job_rx);
        let tx = out_tx.clone();
        std::thread::Builder::new()
            .name(format!("chunk-gen-{i}"))
            .spawn(move || loop {
                let pos = {
                    let lock = match rx.lock() {
                        Ok(g) => g,
                        Err(_) => break,
                    };
                    match lock.recv() {
                        Ok(p) => p,
                        Err(_) => break,
                    }
                };
                tx.send(generate_chunk(seed, pos)).ok();
            })
            .expect("chunk-gen thread"); // process cannot run without the approved gen pool
    }
    (job_tx, out_rx)
}

