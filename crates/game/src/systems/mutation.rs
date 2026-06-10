use engine_core::SystemContext;
use engine_world::{SparseVoxelOctree, WorldMutationQueue};

use crate::components::{TerrainGeneration, WorldInitialized};

pub fn flush_world_mutations_system(ctx: &mut SystemContext<'_>) {
    let pending = ctx
        .resources
        .get_mut::<WorldMutationQueue>()
        .map(|queue| queue.take_pending());

    let Some(pending) = pending else {
        return;
    };
    if pending.is_empty() {
        return;
    }

    let Some(world) = ctx.resources.get_mut::<SparseVoxelOctree>() else {
        return;
    };

    let changes = WorldMutationQueue::apply(world, pending);
    for change in changes {
        ctx.events.send(change);
    }

    let terrain_done = ctx
        .resources
        .get::<TerrainGeneration>()
        .map(|progress| progress.complete)
        .unwrap_or(false);
    let already_initialized = ctx
        .resources
        .get::<WorldInitialized>()
        .map(|flag| flag.0)
        .unwrap_or(false);
    if terrain_done && !already_initialized {
        if let Some(flag) = ctx.resources.get_mut::<WorldInitialized>() {
            flag.0 = true;
        }
    }
}
