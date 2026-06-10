pub mod debug_world;
pub mod extract;
pub mod hud;
pub mod input;
pub mod net;
pub mod play_mode;
pub mod present;
pub mod spectator;

use engine_assets::poll_assets_system;
use engine_core::Stage;

pub use extract::{
    extract_render_world_system, queue_initial_world_meshes_system, sync_block_changes_system,
};
pub use input::sync_local_input_system;
pub use net::client_net_system;
pub use present::{present_frame_system, ClientRenderer};
pub use debug_world::cycle_debug_world_system;
pub use play_mode::toggle_play_mode_system;
pub use spectator::spectator_camera_system;

pub fn register_client_schedule(app: &mut engine_core::App) {
    app.add_system(Stage::PreUpdate, poll_assets_system);
    app.add_system(Stage::PreUpdate, cycle_debug_world_system);
    app.add_system(Stage::PreUpdate, toggle_play_mode_system);
    app.add_system(Stage::PreUpdate, sync_local_input_system);
    app.add_system(Stage::PreUpdate, client_net_system);
    app.add_system(Stage::Update, spectator_camera_system);
    app.add_system(Stage::Extract, sync_block_changes_system);
    app.add_system(Stage::Extract, queue_initial_world_meshes_system);
    app.add_system(Stage::Extract, extract_render_world_system);
    app.add_system(Stage::Render, present_frame_system);
}
