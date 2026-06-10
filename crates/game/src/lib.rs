//! Shared gameplay logic for client and server.

mod components;
mod plugin;
pub mod systems;

pub use components::{TerrainGeneration, *};
pub use plugin::register_game_systems;
pub use systems::terrain::WORLD_RADIUS;
