use engine_assets::{BlockRegistry, CubeFace, NeighborMask};
use engine_world::{BlockId, BlockPos, SparseVoxelOctree};

/// Horizontal neighbor bits for connected-texture resolution at mesh time.
pub fn neighbor_mask_for_face(
    world: &SparseVoxelOctree,
    registry: &BlockRegistry,
    pos: BlockPos,
    block: BlockId,
    face: CubeFace,
) -> NeighborMask {
    let [_, _, nz] = cube_face_normal(face);
    if nz.abs() > 0.5 {
        return NeighborMask::empty();
    }

    let connects = |neighbor: BlockId| neighbor == block;

    let north = connects_at(world, registry, pos, 0, 1, connects);
    let south = connects_at(world, registry, pos, 0, -1, connects);
    let east = connects_at(world, registry, pos, 1, 0, connects);
    let west = connects_at(world, registry, pos, -1, 0, connects);

    match face {
        CubeFace::Front => NeighborMask::from_cardinals(north, east, south, west),
        CubeFace::Back => NeighborMask::from_cardinals(south, west, north, east),
        CubeFace::Right => NeighborMask::from_cardinals(north, south, west, east),
        CubeFace::Left => NeighborMask::from_cardinals(north, south, east, west),
        _ => NeighborMask::empty(),
    }
}

fn connects_at(
    world: &SparseVoxelOctree,
    registry: &BlockRegistry,
    pos: BlockPos,
    dx: i32,
    dy: i32,
    connects: impl Fn(BlockId) -> bool,
) -> bool {
    let neighbor = world.get_block(BlockPos::new(pos.0.x + dx, pos.0.y + dy, pos.0.z));
    registry.is_solid(neighbor) && connects(neighbor)
}

fn cube_face_normal(face: CubeFace) -> [f32; 3] {
    match face {
        CubeFace::Top => [0.0, 0.0, 1.0],
        CubeFace::Bottom => [0.0, 0.0, -1.0],
        CubeFace::Left => [-1.0, 0.0, 0.0],
        CubeFace::Front => [0.0, 1.0, 0.0],
        CubeFace::Right => [1.0, 0.0, 0.0],
        CubeFace::Back => [0.0, -1.0, 0.0],
    }
}
