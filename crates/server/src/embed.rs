use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use engine_core::TICK_MS;
use protocol::{ClientPlay, ServerPlay};

use crate::sim::{Sim, VIEW_CHUNKS};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub seed: i64,
    pub view_chunks: i32,
}

impl ServerConfig {
    pub fn new(seed: i64) -> Self {
        Self {
            seed,
            view_chunks: VIEW_CHUNKS,
        }
    }
}

pub struct EmbeddedHandle {
    pub to_sim: Sender<ClientPlay>,
    pub from_sim: Receiver<ServerPlay>,
    pause: Arc<Mutex<bool>>,
    shutdown: Arc<Mutex<bool>>,
}

impl EmbeddedHandle {
    pub fn set_paused(&self, paused: bool) {
        if let Ok(mut g) = self.pause.lock() {
            *g = paused;
        }
    }

    pub fn shutdown(&self) {
        if let Ok(mut g) = self.shutdown.lock() {
            *g = true;
        }
    }
}

pub fn spawn_embedded(config: ServerConfig) -> EmbeddedHandle {
    let (c2s_tx, c2s_rx) = mpsc::channel::<ClientPlay>();
    let (s2c_tx, s2c_rx) = mpsc::channel::<ServerPlay>();
    let pause = Arc::new(Mutex::new(false));
    let shutdown = Arc::new(Mutex::new(false));
    let pause_t = Arc::clone(&pause);
    let shutdown_t = Arc::clone(&shutdown);

    thread::Builder::new()
        .name("opencraft-sim".into())
        .spawn(move || {
            let mut sim = Sim::new(config);
            sim.join_player(&s2c_tx);
            let tick_dt = Duration::from_millis(TICK_MS);
            let mut next = Instant::now() + tick_dt;
            loop {
                if shutdown_t.lock().map(|g| *g).unwrap_or(true) {
                    break;
                }
                let paused = pause_t.lock().map(|g| *g).unwrap_or(false);
                let mut inbox = Vec::new();
                while let Ok(p) = c2s_rx.try_recv() {
                    inbox.push(p);
                }
                if !paused {
                    sim.tick(&inbox, &s2c_tx);
                }
                pace_tick(&mut next, tick_dt);
            }
        })
        .expect("sim thread"); // thread spawn failure is process-fatal

    EmbeddedHandle {
        to_sim: c2s_tx,
        from_sim: s2c_rx,
        pause,
        shutdown,
    }
}

pub fn pace_tick(next: &mut Instant, dt: Duration) {
    let now = Instant::now();
    if *next > now {
        thread::sleep(*next - now);
    }
    *next += dt;
    if Instant::now() > *next + dt * 4 {
        *next = Instant::now();
    }
}
