use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};

use engine_audio::Mixer;
use engine_core::{BlockPos, ChunkPos, EntityId, Vec3, TICK_MS};
use engine_input::{Snapshot, Tracker};
use engine_render::Renderer;
use protocol::{ClientPlay, InventorySnap, OpenKind};
use server::{spawn_embedded, EmbeddedHandle, ServerConfig};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use world::{ChunkSnapshot, World};

use crate::mesh::{mesh_chunk, Sun};
use crate::pack;
use crate::particles::Particle;
use crate::predict::Predict;
use crate::ui;

pub struct App {
    seed: i64,
    pack_path: Option<String>,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    server: Option<EmbeddedHandle>,
    mixer: Mixer,
    replica: World,
    predict: Option<Predict>,
    inv: InventorySnap,
    health: u8,
    overlay: Option<OpenKind>,
    paused: bool,
    confirm_quit: bool,
    debug: bool,
    third: bool,
    snap: Snapshot,
    edges: Tracker,
    last_tick: Instant,
    start: Instant,
    particles: Vec<Particle>,
    items: HashMap<EntityId, (Vec3, Option<(u16, u8)>)>,
    /// One queue per mesh worker, fed round-robin from the event thread.
    /// No shared lock: workers block on their own channel and mesh in parallel.
    mesh_tx: Vec<Sender<MeshJob>>,
    mesh_next: usize,
    mesh_rx: Receiver<MeshedChunk>,
    pending_upload: Vec<(ChunkPos, engine_render::MeshData)>,
    pending_remove: Vec<ChunkPos>,
    /// Rolling mesh cost: the worker stamps microseconds per chunk, so the
    /// perf log reports the hot path instead of asserting it.
    mesh_ema_ms: f64,
    meshes_done: u64,
    /// Chunks with a mesh job in flight, by pos. Re-edits while in flight go
    /// to `dirty` instead of spamming the worker pool with stale snapshots.
    in_flight: HashSet<ChunkPos>,
    dirty: HashSet<ChunkPos>,
    /// Sun step each loaded chunk was last meshed at. Sun motion re-queues
    /// stale chunks a few per frame so shadows swing without a hitch.
    meshed_step: HashMap<ChunkPos, u8>,
    remesh_queue: VecDeque<ChunkPos>,
    sun: Sun,
    sun_step: u8,
    sun_was_up: bool,
    frame_ema_ms: f64,
    last_frame: Instant,
    last_perf_log: Instant,
    spawn: Vec3,
    look_sens: f32,
    digging: Option<(BlockPos, Instant, u16)>,
    size: (u32, u32),
    slot_hits: Vec<(u16, engine_core::Vec2, engine_core::Vec2)>,
    pause_back: Option<ui::Button>,
    pause_quit: Option<ui::Button>,
    sounds: pack::PackSounds,
}

impl App {
    pub fn new(seed: i64, pack_path: Option<String>) -> Self {
        let (mesh_tx, mesh_rx) = spawn_mesh_workers();
        Self {
            seed,
            pack_path,
            window: None,
            renderer: None,
            server: None,
            mixer: Mixer::silent(),
            replica: World::new(),
            predict: None,
            inv: InventorySnap {
                slots: vec![None; 36],
                cursor: None,
                selected: 0,
            },
            health: 20,
            overlay: None,
            paused: false,
            confirm_quit: false,
            debug: false,
            third: false,
            snap: Snapshot::default(),
            edges: Tracker::default(),
            last_tick: Instant::now(),
            start: Instant::now(),
            particles: Vec::new(),
            items: HashMap::new(),
            mesh_tx,
            mesh_next: 0,
            mesh_rx,
            pending_upload: Vec::new(),
            pending_remove: Vec::new(),
            mesh_ema_ms: 2.0,
            meshes_done: 0,
            in_flight: HashSet::new(),
            dirty: HashSet::new(),
            meshed_step: HashMap::new(),
            remesh_queue: VecDeque::new(),
            sun: Sun {
                dir: Vec3::new(0.3, 1.0, 0.2).normalize_or_zero(),
                up: true,
            },
            sun_step: 0,
            sun_was_up: true,
            frame_ema_ms: 16.0,
            last_frame: Instant::now(),
            last_perf_log: Instant::now(),
            spawn: Vec3::ZERO,
            look_sens: 0.0022,
            digging: None,
            size: (1280, 720),
            slot_hits: Vec::new(),
            pause_back: None,
            pause_quit: None,
            sounds: pack::PackSounds::default(),
        }
    }

    /// Queue one chunk for the mesh pool. In-flight chunks are marked dirty
    /// instead of queued twice, so a burst of edits costs one remesh each.
    pub(super) fn queue_chunk(&mut self, pos: ChunkPos) {
        if self.in_flight.contains(&pos) {
            self.dirty.insert(pos);
            return;
        }
        if self.mesh_tx.is_empty() {
            return;
        }
        if let Some(snap) = self.replica.snapshot(pos) {
            // Single event thread owns the rotation: no atomics, no lock.
            let tx = &self.mesh_tx[self.mesh_next % self.mesh_tx.len()];
            if tx.send((snap, self.sun)).is_ok() {
                self.mesh_next = self.mesh_next.wrapping_add(1);
                self.in_flight.insert(pos);
                self.meshed_step.insert(pos, self.sun_step);
            }
        }
    }

    /// Sun moved a step: re-queue every chunk meshed under an older sun. The
    /// queue drains a few per frame, so a noon-to-dusk swing never hitches.
    /// Rebuilt from scratch each step: O(chunks), and the drain skips any
    /// chunk a fresher edit already re-queued. Nearest first: the pop the
    /// player can see refreshes before the far horizon.
    pub(super) fn queue_sun_refresh(&mut self) {
        if !self.sun.up && !self.sun_was_up {
            return;
        }
        self.remesh_queue.clear();
        let here = self
            .predict
            .as_ref()
            .map(|p| ChunkPos::from_block(p.body.pos.x as i32, p.body.pos.z as i32))
            .unwrap_or(ChunkPos::new(0, 0));
        let mut stale: Vec<ChunkPos> = self
            .replica
            .loaded_chunks()
            .filter(|p| self.meshed_step.get(p) != Some(&self.sun_step))
            .collect();
        stale.sort_by_key(|p| here.chebyshev(*p));
        self.remesh_queue.extend(stale);
    }
}

/// One meshing job: the snapshot truth plus the sun it is shaded under.
pub(super) type MeshJob = (ChunkSnapshot, Sun);
/// One finished job: chunk pos, mesh bytes, and worker-measured micros.
pub(super) type MeshedChunk = (ChunkPos, engine_render::MeshData, u64);

mod draw;
mod net;
mod play;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("OpenCraft")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        let window = event_loop.create_window(attrs).expect("window"); // winit requires a window to attach the surface
        let window = Arc::new(window);
        let mut renderer = match engine_render::platform::create(window.as_ref()) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("renderer: {e}");
                event_loop.exit();
                return;
            }
        };
        let pack = pack::load(self.pack_path.as_deref());
        let atlas = renderer.upload_rgba(crate::atlas::ATLAS, crate::atlas::ATLAS, &pack.atlas);
        renderer.set_atlas(atlas);
        renderer.set_ui_atlas(atlas);
        let skin = renderer.upload_rgba(64, 64, &pack.skin);
        renderer.set_skin(skin);
        self.mixer = Mixer::start().unwrap_or_else(|e| {
            eprintln!("audio: {e}");
            Mixer::silent()
        });
        if let Some(root) = self
            .pack_path
            .clone()
            .map(std::path::PathBuf::from)
            .or_else(pack::default_pack_dir)
        {
            self.sounds = pack::load_oggs(&root, &mut self.mixer);
        }
        self.server = Some(spawn_embedded(ServerConfig::new(self.seed)));
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.grab(true);
        self.last_tick = Instant::now();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(s) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(s.width.max(1), s.height.max(1));
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                crate::input_map::apply_key(&mut self.snap, &event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                crate::input_map::apply_mouse_button(&mut self.snap, button, state);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                crate::input_map::apply_scroll(&mut self.snap, delta);
            }
            WindowEvent::CursorMoved { position, .. } => {
                crate::input_map::apply_cursor(&mut self.snap, position);
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.look(delta.0, delta.1);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.pump();
        let e = self.edges.edges(self.snap);
        if e.debug {
            self.debug = !self.debug;
        }
        if e.perspective && self.overlay.is_none() && !self.paused {
            self.third = !self.third;
        }
        if e.inventory && !self.paused {
            if self.overlay.is_none() {
                self.send(ClientPlay::OpenInventory);
            } else {
                self.send(ClientPlay::CloseWindow);
            }
        }
        if e.pause {
            if self.overlay.is_some() {
                self.send(ClientPlay::CloseWindow);
            } else if self.confirm_quit {
                self.confirm_quit = false;
            } else {
                let p = !self.paused;
                self.set_paused(p);
            }
        }
        if self.snap.hotbar.is_some() {
            if let Some(h) = self.snap.hotbar {
                self.inv.selected = h;
                self.send(ClientPlay::SelectHotbar { slot: h });
                self.snap.hotbar = None;
            }
        }
        if self.snap.scroll != 0 && self.overlay.is_none() {
            let s = self.inv.selected as i32 - self.snap.scroll.signum();
            self.inv.selected = ((s % 9) + 9) as u8 % 9;
            self.send(ClientPlay::SelectHotbar { slot: self.inv.selected });
            self.snap.scroll = 0;
        }
        if self.paused && self.snap.left_click {
            self.snap.left_click = false;
            if self.pause_clicks() {
                event_loop.exit();
            }
        }
        self.play_clicks(e);
        self.overlay_clicks(e);
        if self.last_tick.elapsed() >= Duration::from_millis(TICK_MS) {
            self.tick_predict();
        }
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}

fn spawn_mesh_workers() -> (Vec<Sender<MeshJob>>, Receiver<MeshedChunk>) {
    let (out_tx, out_rx) = mpsc::channel();
    let n = (std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)).clamp(2, 6);
    let mut job_txs = Vec::with_capacity(n);
    for i in 0..n {
        let (job_tx, job_rx) = mpsc::channel::<MeshJob>();
        job_txs.push(job_tx);
        let tx = out_tx.clone();
        std::thread::Builder::new()
            .name(format!("mesh-{i}"))
            .spawn(move || {
                // Blocking on a private channel: no shared lock, and the
                // worker count still matches the approved thread graph.
                for (snap, sun) in job_rx {
                    let t0 = std::time::Instant::now();
                    let mesh = mesh_chunk(&snap, sun);
                    let us = t0.elapsed().as_micros() as u64;
                    tx.send((snap.pos, mesh, us)).ok();
                }
            })
            .expect("mesh worker"); // mesh pool is part of the approved thread graph
    }
    (job_txs, out_rx)
}
