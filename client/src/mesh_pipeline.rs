use engine_assets::BlockRegistry;
use engine_render::{RenderExtractState, CHUNK_MESH_RENDER_DISTANCE};
use engine_world::{SparseVoxelOctree, CHUNK_SIZE};
use game::{GRASS_PLANE_Z, WORLD_RADIUS};
use glam::{IVec3, Vec3};

pub const MESH_BATCH_SIZE: usize = 16;

fn terrain_chunk_coords() -> impl Iterator<Item = IVec3> {
    let min = -WORLD_RADIUS;
    let max = WORLD_RADIUS - 1;
    let min_cx = min.div_euclid(CHUNK_SIZE);
    let max_cx = max.div_euclid(CHUNK_SIZE);
    let min_cy = min.div_euclid(CHUNK_SIZE);
    let max_cy = max.div_euclid(CHUNK_SIZE);
    let chunk_z = GRASS_PLANE_Z.div_euclid(CHUNK_SIZE);
    (min_cx..=max_cx).flat_map(move |cx| {
        (min_cy..=max_cy).map(move |cy| IVec3::new(cx, cy, chunk_z))
    })
}

pub fn bootstrap_terrain_meshes(state: &mut RenderExtractState) {
    if state.world_mesh_queued {
        return;
    }
    state.mesh_cache = engine_render::ChunkMeshCache::default();
    for chunk in terrain_chunk_coords() {
        state.mesh_cache.mark_dirty(chunk);
    }
    state.world_mesh_queued = true;
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
    camera_position: Vec3,
    full_rebuild: bool,
) -> usize {
    let top_faces_only = true;
    if full_rebuild {
        return state
            .mesh_cache
            .rebuild_all_dirty(world, registry, top_faces_only);
    }
    state.mesh_cache.rebuild_dirty_near(
        world,
        registry,
        camera_position,
        CHUNK_MESH_RENDER_DISTANCE,
        top_faces_only,
    )
}
