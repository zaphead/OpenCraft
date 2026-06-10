use std::path::PathBuf;

use engine_assets::load_block_registry;
use engine_core::{App, Time};
use engine_world::{SparseVoxelOctree, WorldMutationQueue};
use game::{register_game_systems, TerrainGeneration, WorldInitialized};

const TICK_RATE: f32 = 60.0;
const MAX_TICKS: u64 = 600;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.insert_resource(Time::new(1.0 / TICK_RATE));
    app.insert_resource(SparseVoxelOctree::default());
    app.insert_resource(WorldMutationQueue::default());
    app.insert_resource(WorldInitialized::default());
    app.insert_resource(TerrainGeneration::default());

    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("blocks");
    app.insert_resource(load_block_registry(&assets));

    register_game_systems(&mut app);

    log::info!("Chicken Jockey server starting (headless, {MAX_TICKS} ticks)");
    app.run_headless(1.0 / TICK_RATE, MAX_TICKS);
    log::info!(
        "Server finished — world initialized: {:?}, entities: {}",
        app.resource::<WorldInitialized>().map(|flag| flag.0),
        app.world.len()
    );
}
