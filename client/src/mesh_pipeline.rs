use engine_assets::{BlockRegistry, ResolvedBlockMaterials};
use engine_render::{RenderExtractState, RebuildBudget, CHUNK_MESH_RENDER_DISTANCE};
use engine_world::{BiomeMap, SparseVoxelOctree};
use glam::{IVec3, Vec3};

pub const MESH_BATCH_SIZE: usize = 16;

fn terrain_chunk_coords() -> impl Iterator<Item = IVec3> {
    std::iter::once(IVec3::ZERO)
}

pub fn bootstrap_terrain_meshes(state: &mut RenderExtractState) {
    if state.terrain_bootstrapped {
        return;
    }
    state.mesh_cache = engine_render::ChunkMeshCache::default();
    for chunk in terrain_chunk_coords() {
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
