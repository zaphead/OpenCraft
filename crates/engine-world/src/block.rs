use glam::IVec3;
use serde::{Deserialize, Serialize};

pub const CHUNK_SIZE: i32 = 16;

pub type BlockId = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos(pub IVec3);

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn chunk_key(self) -> IVec3 {
        IVec3::new(
            self.0.x.div_euclid(CHUNK_SIZE),
            self.0.y.div_euclid(CHUNK_SIZE),
            self.0.z.div_euclid(CHUNK_SIZE),
        )
    }

    pub fn local_chunk_offset(self) -> IVec3 {
        IVec3::new(
            self.0.x.rem_euclid(CHUNK_SIZE),
            self.0.y.rem_euclid(CHUNK_SIZE),
            self.0.z.rem_euclid(CHUNK_SIZE),
        )
    }
}
