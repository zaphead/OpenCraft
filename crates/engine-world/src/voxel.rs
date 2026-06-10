use serde::{Deserialize, Serialize};

use crate::block::BlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BlockState(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VoxelCell {
    pub id: BlockId,
    pub state: BlockState,
}

impl VoxelCell {
    pub const AIR: Self = Self {
        id: 0,
        state: BlockState(0),
    };

    pub fn from_id(id: BlockId) -> Self {
        Self {
            id,
            state: BlockState(0),
        }
    }

    pub fn block_id(self) -> BlockId {
        self.id
    }
}
