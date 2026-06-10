use engine_assets::{BlockRegistry, ResolvedBlockMaterials};
use engine_render::{RenderExtractState, RebuildBudget, CHUNK_MESH_RENDER_DISTANCE};
use engine_world::{BiomeMap, SparseVoxelOctree};
use game::{iter_mesh_chunks, DebugWorldKind};
use glam::Vec3;

pub const MESH_BATCH_SIZE: usize = 16;

pub fn bootstrap_terrain_meshes(state: &mut RenderExtractState, world: DebugWorldKind) {
    if state.terrain_bootstrapped {
        return;
    }
    state.mesh_cache = engine_render::ChunkMeshCache::default();
    for chunk in iter_mesh_chunks(world) {
        state.mesh_cache.mark_dirty(chunk);
    }
    state.terrain_bootstrapped = true;
    state.pending_full_rebuild = true;
}

pub fn enqueue_mesh_batch(state: &mut RenderExtractState) {
    let batch = state.world_mesh_queue.len().min(MESH_BATCH_SIZE);
    for chunk in state.world_mesh_queue.drain(..batch) {
        state.mesh_cache.mark_dirty(chunk);
    }
}

pub fn rebuild_chunk_meshes(
    state: &mut RenderExtractState,
    world: &SparseVoxelOctree,
    registry: &BlockRegistry,
    materials: &ResolvedBlockMaterials,
    biome: &BiomeMap,
    camera_position: Vec3,
    budget: RebuildBudget,
) -> usize {
    state.mesh_cache.rebuild(
        world,
        registry,
        materials,
        biome,
        camera_position,
        budget,
        false,
    )
}

pub fn rebuild_budget_for_extract(state: &mut RenderExtractState) -> RebuildBudget {
    if state.pending_full_rebuild {
        state.pending_full_rebuild = false;
        RebuildBudget::all()
    } else {
        RebuildBudget::near(CHUNK_MESH_RENDER_DISTANCE)
    }
}
