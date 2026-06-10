use engine_assets::BlockRegistry;
use engine_core::SystemContext;
use engine_world::{BlockPos, WorldMutationQueue};

use crate::components::TerrainGeneration;

pub const WORLD_RADIUS: i32 = 16;
pub const DEBUG_BLOCK_Z: i32 = 0;
pub const DEBUG_BLOCK_SPACING: i32 = 7;
/// Alias used by mesh bootstrap and legacy callers.
pub const GRASS_PLANE_Z: i32 = DEBUG_BLOCK_Z;

const PLAYER_HALF_HEIGHT: f32 = 1.0;

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
    let Some(dirt) = registry.id_by_name("dirt") else {
        return;
    };
    let Some(stone) = registry.id_by_name("stone") else {
        return;
    };

    let Some(queue) = ctx.resources.get_mut::<WorldMutationQueue>() else {
        return;
    };

    let debug_blocks = [
        (BlockPos::new(0, 0, DEBUG_BLOCK_Z), grass),
        (
            BlockPos::new(DEBUG_BLOCK_SPACING, 0, DEBUG_BLOCK_Z),
            dirt,
        ),
        (
            BlockPos::new(DEBUG_BLOCK_SPACING * 2, 0, DEBUG_BLOCK_Z),
            stone,
        ),
    ];

    for (pos, block) in debug_blocks {
        queue.set_block(pos, block);
    }

    if let Some(progress) = ctx.resources.get_mut::<TerrainGeneration>() {
        progress.complete = true;
    }
}

pub fn terrain_surface_z(x: i32, y: i32) -> i32 {
    if y == 0
        && (x == 0 || x == DEBUG_BLOCK_SPACING || x == DEBUG_BLOCK_SPACING * 2)
    {
        DEBUG_BLOCK_Z
    } else {
        DEBUG_BLOCK_Z - 1
    }
}

pub fn player_spawn_center_z_at(x: i32, y: i32) -> f32 {
    terrain_surface_z(x, y) as f32 + 1.0 + PLAYER_HALF_HEIGHT
}

pub fn player_spawn_center_z() -> f32 {
    player_spawn_center_z_at(0, 0)
}
