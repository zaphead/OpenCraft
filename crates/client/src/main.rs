mod app;
mod atlas;
mod input_map;
mod mesh;
mod pack;
mod particles;
mod predict;
mod replica;
mod ui;

use anyhow::Context;
use std::time::{SystemTime, UNIX_EPOCH};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> anyhow::Result<()> {
    let seed = parse_seed();
    let pack = std::env::args()
        .skip_while(|a| a != "--pack")
        .nth(1)
        .or_else(|| std::env::var("OPENCRAFT_PACK").ok());
    if let Some(p) = &pack {
        if !std::path::Path::new(p).exists() {
            eprintln!("pack path missing: {p} (missing-texture fallback)");
        }
    }
    let event_loop = EventLoop::new().context("event loop")?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = app::App::new(seed, pack);
    event_loop.run_app(&mut app).context("run")?;
    Ok(())
}

fn parse_seed() -> i64 {
    std::env::args()
        .skip_while(|a| a != "--seed")
        .nth(1)
        .or_else(|| std::env::var("OPENCRAFT_SEED").ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos() as i64)
                .unwrap_or(1)
        })
}
