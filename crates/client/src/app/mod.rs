use std::collections::HashMap;
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

use crate::mesh::mesh_chunk;
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
    mesh_tx: Sender<ChunkSnapshot>,
    mesh_rx: Receiver<(ChunkPos, engine_render::MeshData)>,
    pending_upload: Vec<(ChunkPos, engine_render::MeshData)>,
    pending_remove: Vec<ChunkPos>,
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
            mesh_rx,
            pending_upload: Vec::new(),
            pending_remove: Vec::new(),
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
}

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
        if let Some(root) = self.pack_path.as_deref() {
            self.sounds = pack::load_oggs(std::path::Path::new(root), &mut self.mixer);
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

fn spawn_mesh_workers() -> (Sender<ChunkSnapshot>, Receiver<(ChunkPos, engine_render::MeshData)>) {
    let (job_tx, job_rx) = mpsc::channel::<ChunkSnapshot>();
    let (out_tx, out_rx) = mpsc::channel();
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));
    let n = (std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)).clamp(2, 6);
    for i in 0..n {
        let rx = std::sync::Arc::clone(&job_rx);
        let tx = out_tx.clone();
        std::thread::Builder::new()
            .name(format!("mesh-{i}"))
            .spawn(move || loop {
                let snap = {
                    let lock = match rx.lock() {
                        Ok(g) => g,
                        Err(_) => break,
                    };
                    match lock.recv() {
                        Ok(s) => s,
                        Err(_) => break,
                    }
                };
                let mesh = mesh_chunk(&snap);
                tx.send((snap.pos, mesh)).ok();
            })
            .expect("mesh worker"); // mesh pool is part of the approved thread graph
    }
    (job_tx, out_rx)
}
