use crate::block::{BlockId, BlockPos};
use crate::svo::SparseVoxelOctree;

#[derive(Debug, Clone, Copy)]
pub struct BlockChanged {
    pub position: BlockPos,
    pub old_block: BlockId,
    pub new_block: BlockId,
}

#[derive(Debug, Clone, Copy)]
pub enum WorldMutation {
    SetBlock { position: BlockPos, block: BlockId },
}

#[derive(Default)]
pub struct WorldMutationQueue {
    pending: Vec<WorldMutation>,
}

impl WorldMutationQueue {
    pub fn set_block(&mut self, position: BlockPos, block: BlockId) {
        self.pending.push(WorldMutation::SetBlock { position, block });
    }

    pub fn take_pending(&mut self) -> Vec<WorldMutation> {
        std::mem::take(&mut self.pending)
    }

    pub fn apply(
        world: &mut SparseVoxelOctree,
        pending: Vec<WorldMutation>,
    ) -> Vec<BlockChanged> {
        let mut changes = Vec::with_capacity(pending.len());

        for mutation in pending {
            match mutation {
                WorldMutation::SetBlock { position, block } => {
                    let old_block = world.get_block(position);
                    if old_block != block {
                        world.set_block(position, block);
                        changes.push(BlockChanged {
                            position,
                            old_block,
                            new_block: block,
                        });
                    }
                }
            }
        }

        changes
    }

    pub fn flush(&mut self, world: &mut SparseVoxelOctree) -> Vec<BlockChanged> {
        Self::apply(world, self.take_pending())
    }
}
