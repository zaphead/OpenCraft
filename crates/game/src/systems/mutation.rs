use engine_core::SystemContext;
use engine_world::{SparseVoxelOctree, WorldMutationQueue};

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
}
