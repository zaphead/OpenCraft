use engine_assets::BlockRegistry;
use engine_world::{BlockPos, SparseVoxelOctree};
use glam::Vec3;

pub fn collides_aabb(
    world: &SparseVoxelOctree,
    registry: &BlockRegistry,
    position: Vec3,
    half_extents: Vec3,
) -> bool {
    let aabb_min = position - half_extents;
    let aabb_max = position + half_extents;
    let min = aabb_min.floor().as_ivec3();
    // Unit voxel [i, i+1) — ceil(max) - 1 is the highest overlapping index.
    let max = (aabb_max.ceil() - Vec3::ONE).as_ivec3();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let block = world.get_block(BlockPos::new(x, y, z));
                if registry.is_solid(block) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use engine_assets::load_block_registry;
    use engine_world::SparseVoxelOctree;

    use super::*;

    fn registry() -> BlockRegistry {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/blocks"
        ));
        load_block_registry(path)
    }

    #[test]
    fn centered_player_footprint_is_single_xy_voxel() {
        let registry = registry();
        let mut world = SparseVoxelOctree::new();
        let stone = registry.id_by_name("stone").expect("stone");
        world.set_block(BlockPos::new(8, 0, 0), stone);

        let half = Vec3::new(0.28, 0.28, 0.95);
        let center = Vec3::new(7.5, 0.5, 1.0);
        assert!(
            !collides_aabb(&world, &registry, center, half),
            "player centered in block 7 should not overlap block 8"
        );
    }
}
