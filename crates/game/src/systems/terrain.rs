use engine_assets::BlockRegistry;
use engine_core::SystemContext;
use engine_world::{BlockPos, WorldMutationQueue};

use crate::components::TerrainGeneration;

pub const WORLD_RADIUS: i32 = 64;
/// Grass plane lies on XY; Z is the vertical axis.
pub const GRASS_PLANE_Z: i32 = 0;
const PLAYER_HALF_HEIGHT: f32 = 0.9;

pub fn generate_terrain_system(ctx: &mut SystemContext<'_>) {
    let Some(progress) = ctx.resources.get::<TerrainGeneration>().copied() else {
        return;
    };
    if progress.complete {
        return;
    }

    let Some(registry) = ctx.resources.get::<BlockRegistry>() else {
        return;
    };
    let Some(grass) = registry.id_by_name("grass") else {
        return;
    };

    let Some(queue) = ctx.resources.get_mut::<WorldMutationQueue>() else {
        return;
    };

    for x in -WORLD_RADIUS..WORLD_RADIUS {
        for y in -WORLD_RADIUS..WORLD_RADIUS {
            queue.set_block(BlockPos::new(x, y, GRASS_PLANE_Z), grass);
        }
    }

    if let Some(progress) = ctx.resources.get_mut::<TerrainGeneration>() {
        progress.complete = true;
    }
}

/// Collider-center height for entities standing on the grass plane.
pub fn player_spawn_center_z() -> f32 {
    GRASS_PLANE_Z as f32 + 1.0 + PLAYER_HALF_HEIGHT
}
