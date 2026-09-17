use server::{pace_tick, ServerConfig, Sim};
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let seed = parse_seed();
    eprintln!("OpenCraft dedicated seed={seed} (headless)");
    let mut sim = Sim::new(ServerConfig::new(seed));
    let (tx, rx) = mpsc::channel();

    let dt = Duration::from_millis(engine_core::TICK_MS);
    let mut next = Instant::now() + dt;
    loop {
        sim.tick(&[], &tx);
        while rx.try_recv().is_ok() {}
        pace_tick(&mut next, dt);
    }
}

fn parse_seed() -> i64 {
    std::env::args()
        .skip_while(|a| a != "--seed")
        .nth(1)
        .or_else(|| std::env::var("OPENCRAFT_SEED").ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(random_seed)
}

fn random_seed() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(1)
}
