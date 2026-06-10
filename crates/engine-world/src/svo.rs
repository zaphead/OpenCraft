use std::collections::HashMap;

use crate::block::{BlockId, BlockPos};

const AIR: BlockId = 0;

/// Minimal sparse voxel octree backed by a hash map at leaf resolution.
/// Interior aggregate propagation is tracked per parent region.
pub struct SparseVoxelOctree {
    leaves: HashMap<(i32, i32, i32), BlockId>,
    solid_regions: HashMap<(i32, i32, i32), u8>,
    region_shift: u32,
}

impl Default for SparseVoxelOctree {
    fn default() -> Self {
        Self::new(4)
    }
}

impl SparseVoxelOctree {
    pub fn new(region_shift: u32) -> Self {
        Self {
            leaves: HashMap::new(),
            solid_regions: HashMap::new(),
            region_shift,
        }
    }

    pub fn get_block(&self, position: BlockPos) -> BlockId {
        let key = (position.0.x, position.0.y, position.0.z);
        self.leaves.get(&key).copied().unwrap_or(AIR)
    }

    pub fn is_solid(&self, position: BlockPos) -> bool {
        self.get_block(position) != AIR
    }

    pub fn set_block(&mut self, position: BlockPos, block: BlockId) {
        let key = (position.0.x, position.0.y, position.0.z);
        if block == AIR {
            self.leaves.remove(&key);
        } else {
            self.leaves.insert(key, block);
        }
        self.propagate_aggregates(position);
    }

    pub fn for_each_solid_in_region<F>(&self, min: BlockPos, max: BlockPos, mut f: F)
    where
        F: FnMut(BlockPos, BlockId),
    {
        for ((x, y, z), block) in &self.leaves {
            if *block == AIR {
                continue;
            }
            if x >= &min.0.x
                && y >= &min.0.y
                && z >= &min.0.z
                && x < &max.0.x
                && y < &max.0.y
                && z < &max.0.z
            {
                f(BlockPos::new(*x, *y, *z), *block);
            }
        }
    }

    fn region_key(&self, position: BlockPos) -> (i32, i32, i32) {
        let shift = self.region_shift as i32;
        (
            position.0.x >> shift,
            position.0.y >> shift,
            position.0.z >> shift,
        )
    }

    fn propagate_aggregates(&mut self, position: BlockPos) {
        let mut current = position;
        for _ in 0..8 {
            let region = self.region_key(current);
            let size = 1i32 << self.region_shift;
            let origin = BlockPos::new(
                region.0 << self.region_shift,
                region.1 << self.region_shift,
                region.2 << self.region_shift,
            );
            let mut solid_count = 0u8;
            for x in 0..size {
                for y in 0..size {
                    for z in 0..size {
                        if self.get_block(BlockPos::new(
                            origin.0.x + x,
                            origin.0.y + y,
                            origin.0.z + z,
                        )) != AIR
                        {
                            solid_count = solid_count.saturating_add(1);
                        }
                    }
                }
            }
            if solid_count == 0 {
                self.solid_regions.remove(&region);
            } else {
                self.solid_regions.insert(region, solid_count);
            }
            current = BlockPos::new(
                current.0.x >> 1,
                current.0.y >> 1,
                current.0.z >> 1,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_round_trip() {
        let mut world = SparseVoxelOctree::default();
        let pos = BlockPos::new(1, 2, 3);
        world.set_block(pos, 2);
        assert_eq!(world.get_block(pos), 2);
        world.set_block(pos, AIR);
        assert_eq!(world.get_block(pos), AIR);
    }
}
