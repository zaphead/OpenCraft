//! Shared gameplay logic for client and server.

mod axes;
mod components;
mod debug_world;
mod events;
mod input;
mod mode;
mod movement;
mod play_mode;
mod plugin;
pub mod systems;

pub use axes::{
    grounded_probe_offset, horizontal_forward, horizontal_right, view_forward,
    PLAYER_EYE_OFFSET_Z, PLAYER_HALF_EXTENTS, UP,
};
pub use components::{TerrainGeneration, *};
pub use events::{BlockChangeIntent, PlayerStateChanged};
pub use input::{
    local_player_entity, resolve_input, GameplayInput, LocalPlayerId, PlayerInputs,
};
pub use mode::{AuthoritativeServer, NetworkClient};
pub use movement::{
    accelerate_toward, apply_ice_drag, max_speed, wish_direction_fly, wish_direction_horizontal,
    LocomotionConfig, MOUSE_SENSITIVITY,
};
pub use debug_world::{iter_mesh_chunks, ActiveDebugWorld, DebugWorldKind};
pub use play_mode::{ActivePlayMode, PlayMode};
pub use plugin::{
    register_authoritative_block_system, register_local_client_systems,
    register_network_client_systems, register_player_systems, register_player_spawn_systems,
    register_server_systems, register_world_systems,
};
pub use systems::terrain::{
    player_spawn_center_z, player_spawn_center_z_at, rolling_hills_max_surface_z,
    terrain_surface_z, DEBUG_BLOCK_SPACING, DEBUG_BLOCK_Z, GRASS_PLANE_Z, ROLLING_HILLS_RADIUS,
    WORLD_RADIUS,
};
pub use systems::spawn_net_player;
