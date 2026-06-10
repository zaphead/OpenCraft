use engine_core::SystemContext;
use engine_render::RenderExtractState;
use engine_world::{SparseVoxelOctree, WorldMutationQueue};
use game::{
    local_player_entity, ActiveDebugWorld, ActivePlayMode, DebugWorldKind, LocalPlayerId,
    NetworkClient, PlayMode, TerrainGeneration, WorldInitialized,
};

use crate::systems::input::PendingWinitInput;
use crate::systems::spectator::{reset_spectator_for_world, SpectatorCamera};

pub fn cycle_debug_world_system(ctx: &mut SystemContext<'_>) {
    if ctx.resources.get::<NetworkClient>().is_some() {
        return;
    }
    let cycle = ctx
        .resources
        .get::<PendingWinitInput>()
        .map(|pending| pending.0.cycle_debug_world)
        .unwrap_or(false);
    if !cycle {
        return;
    }

    let next = ctx
        .resources
        .get::<ActiveDebugWorld>()
        .map(|active| active.0.next())
        .unwrap_or(DebugWorldKind::RollingHills);

    if let Some(active) = ctx.resources.get_mut::<ActiveDebugWorld>() {
        active.0 = next;
    }

    reset_world_state(ctx, next);
}

fn reset_world_state(ctx: &mut SystemContext<'_>, world: DebugWorldKind) {
    if let Some(svo) = ctx.resources.get_mut::<SparseVoxelOctree>() {
        *svo = SparseVoxelOctree::new();
    }
    if let Some(queue) = ctx.resources.get_mut::<WorldMutationQueue>() {
        queue.take_pending();
    }
    if let Some(progress) = ctx.resources.get_mut::<TerrainGeneration>() {
        progress.complete = false;
    }
    if let Some(flag) = ctx.resources.get_mut::<WorldInitialized>() {
        flag.0 = false;
    }
    if let Some(state) = ctx.resources.get_mut::<RenderExtractState>() {
        state.mesh_cache = engine_render::ChunkMeshCache::default();
        state.world_mesh_queue.clear();
        state.terrain_bootstrapped = false;
        state.pending_full_rebuild = true;
    }

    if let Some(entity) = local_player_entity(ctx) {
        let _ = ctx.world.despawn(entity);
    }
    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.spawned = false;
        local.id = None;
    }
    if let Some(mode) = ctx.resources.get_mut::<ActivePlayMode>() {
        mode.0 = PlayMode::Survival;
    }
    if let Some(camera) = ctx.resources.get_mut::<SpectatorCamera>() {
        *camera = reset_spectator_for_world(world);
    }
}
