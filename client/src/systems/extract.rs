use std::sync::Arc;

use engine_assets::{BlockRegistry, ResolvedBlockMaterials};
use engine_core::{SystemContext, Time};
use engine_render::{Camera, RenderExtractState, RenderSurfaceInfo, RenderWorld};
use engine_world::{BiomeMap, SparseVoxelOctree, VoxelChanged};
use game::{
    local_player_entity, ActiveDebugWorld, ActivePlayMode, DebugWorldKind, PlayMode, Transform,
    WorldInitialized, PLAYER_EYE_OFFSET_Z,
};

use crate::mesh_pipeline::{bootstrap_terrain_meshes, rebuild_budget_for_extract, rebuild_chunk_meshes};
use crate::systems::spectator::SpectatorCamera;

pub fn sync_block_changes_system(ctx: &mut SystemContext<'_>) {
    let Some(state) = ctx.resources.get_mut::<RenderExtractState>() else {
        return;
    };
    let changes: Vec<VoxelChanged> = ctx.events.drain::<VoxelChanged>();
    if changes.len() > 64 {
        return;
    }
    for change in changes {
        state.mesh_cache.mark_dirty_neighbors(change.position);
    }
}

pub fn queue_initial_world_meshes_system(ctx: &mut SystemContext<'_>) {
    let initialized = ctx
        .resources
        .get::<WorldInitialized>()
        .map(|flag| flag.0)
        .unwrap_or(false);
    if !initialized {
        return;
    }
    let world_kind = ctx
        .resources
        .get::<ActiveDebugWorld>()
        .map(|active| active.0)
        .unwrap_or(DebugWorldKind::RollingHills);
    let Some(state) = ctx.resources.get_mut::<RenderExtractState>() else {
        return;
    };
    bootstrap_terrain_meshes(state, world_kind);
}

pub fn extract_render_world_system(ctx: &mut SystemContext<'_>) {
    let aspect = ctx
        .resources
        .get::<RenderSurfaceInfo>()
        .map(|info| info.aspect)
        .unwrap_or(16.0 / 9.0);
    let camera = extract_camera(ctx, aspect);
    let animation_tick = ctx
        .resources
        .get::<Time>()
        .map(|time| (time.elapsed * 1000.0) as u32)
        .unwrap_or(0);

    let Some(materials) = ctx.resources.get::<Arc<ResolvedBlockMaterials>>().cloned() else {
        return;
    };

    let biome = ctx
        .resources
        .get::<BiomeMap>()
        .cloned()
        .unwrap_or_default();
    let buckets = ctx
        .resources
        .with_triple::<SparseVoxelOctree, BlockRegistry, RenderExtractState, _>(
            |world, registry, state| {
                if state.mesh_cache.has_dirty_chunks() {
                    let budget = rebuild_budget_for_extract(state);
                    rebuild_chunk_meshes(
                        state,
                        world,
                        registry,
                        &materials,
                        &biome,
                        camera.position,
                        budget,
                    );
                }
                state.mesh_cache.merged_buckets()
            },
        )
        .unwrap_or_default();

    if let Some(render_world) = ctx.resources.get_mut::<RenderWorld>() {
        render_world.camera = camera;
        render_world.opaque = buckets.opaque;
        render_world.cutout = buckets.cutout;
        render_world.animation_tick = animation_tick;
        render_world.ready = true;
    }
}

fn extract_camera(ctx: &SystemContext<'_>, aspect: f32) -> Camera {
    let survival = ctx
        .resources
        .get::<ActivePlayMode>()
        .is_none_or(|mode| mode.0 == PlayMode::Survival);

    if survival {
        if let Some(entity) = local_player_entity(ctx) {
            if let Ok(transform) = ctx.world.get::<&Transform>(entity) {
                return Camera {
                    position: transform.position + glam::Vec3::new(0.0, 0.0, PLAYER_EYE_OFFSET_Z),
                    yaw: transform.yaw,
                    pitch: transform.pitch,
                    aspect,
                    ..Camera::default()
                };
            }
        }
    }

    let spectator = ctx
        .resources
        .get::<SpectatorCamera>()
        .expect("SpectatorCamera must be registered");
    Camera {
        position: spectator.position,
        yaw: spectator.yaw,
        pitch: spectator.pitch,
        aspect,
        ..Camera::default()
    }
}
