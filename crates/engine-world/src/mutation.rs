use crate::block::{BlockId, BlockPos};
use crate::svo::SparseVoxelOctree;
use crate::voxel::VoxelCell;

#[derive(Debug, Clone, Copy)]
pub struct VoxelChanged {
    pub position: BlockPos,
    pub old_cell: VoxelCell,
    pub new_cell: VoxelCell,
}

#[derive(Debug, Clone, Copy)]
pub enum WorldMutation {
    SetBlock { position: BlockPos, block: BlockId },
    SetVoxel { position: BlockPos, cell: VoxelCell },
}

#[derive(Default)]
pub struct WorldMutationQueue {
    pending: Vec<WorldMutation>,
}

impl WorldMutationQueue {
    pub fn set_block(&mut self, position: BlockPos, block: BlockId) {
        self.pending.push(WorldMutation::SetBlock { position, block });
    }

    pub fn set_voxel(&mut self, position: BlockPos, cell: VoxelCell) {
        self.pending.push(WorldMutation::SetVoxel { position, cell });
    }

    pub fn take_pending(&mut self) -> Vec<WorldMutation> {
        std::mem::take(&mut self.pending)
    }

    pub fn apply(
        world: &mut SparseVoxelOctree,
        pending: Vec<WorldMutation>,
    ) -> Vec<VoxelChanged> {
        let mut changes = Vec::with_capacity(pending.len());

        for mutation in pending {
            match mutation {
                WorldMutation::SetBlock { position, block } => {
                    let old_cell = world.get_voxel(position);
                    let new_cell = if block == 0 {
                        VoxelCell::AIR
                    } else {
                        VoxelCell::from_id(block)
                    };
                    if old_cell != new_cell {
                        world.set_voxel(position, new_cell);
                        changes.push(VoxelChanged {
                            position,
                            old_cell,
                            new_cell,
                        });
                    }
                }
                WorldMutation::SetVoxel { position, cell } => {
                    let old_cell = world.get_voxel(position);
                    if old_cell != cell {
                        world.set_voxel(position, cell);
                        changes.push(VoxelChanged {
                            position,
                            old_cell,
                            new_cell: cell,
                        });
                    }
                }
            }
        }

        changes
    }

    pub fn flush(&mut self, world: &mut SparseVoxelOctree) -> Vec<VoxelChanged> {
        Self::apply(world, self.take_pending())
    }
}
