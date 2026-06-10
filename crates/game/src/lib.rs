//! Shared gameplay logic for client and server.

mod axes;
mod components;
mod events;
mod input;
mod mode;
mod play_mode;
mod plugin;
pub mod systems;

pub use axes::{
    grounded_probe_offset, horizontal_forward, horizontal_right, view_forward, UP,
};
pub use components::{TerrainGeneration, *};
pub use events::{BlockChangeIntent, PlayerStateChanged};
pub use input::{local_player_entity, GameplayInput, LocalPlayerId, PlayerInputs};
pub use mode::{AuthoritativeServer, NetworkClient};
pub use play_mode::{ActivePlayMode, PlayMode};
pub use plugin::{
    register_authoritative_block_system, register_local_client_systems,
    register_network_client_systems, register_player_systems, register_player_spawn_systems,
    register_server_systems, register_world_systems,
};
pub use systems::terrain::{
    player_spawn_center_z, player_spawn_center_z_at, terrain_surface_z, DEBUG_BLOCK_SPACING,
    DEBUG_BLOCK_Z, GRASS_PLANE_Z, WORLD_RADIUS,
};
pub use systems::spawn_net_player;
