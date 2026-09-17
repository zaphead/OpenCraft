mod chunk;
mod entities;
mod palette;
mod store;

pub use chunk::{Chunk, ChunkSnapshot};
pub use palette::Section;
pub use entities::{Entities, EntityKind};
pub use store::{Stack, World};

use engine_core::{BlockPos, MIN_Y, SECTION_EDGE};
use engine_phys::VoxelSolid;

/// Empty cell is id 0. Block solidity rules live in `game::is_solid`.
impl VoxelSolid for World {
    fn solid(&self, x: i32, y: i32, z: i32) -> bool {
        if y < MIN_Y {
            return true;
        }
        self.block(BlockPos::new(x, y, z)) != 0
    }
}

pub fn section_local_index(lx: u32, ly: u32, lz: u32) -> usize {
    (ly * 16 * 16 + lz * 16 + lx) as usize
}

pub fn in_section(lx: u32, ly: u32, lz: u32) -> bool {
    lx < SECTION_EDGE as u32 && ly < SECTION_EDGE as u32 && lz < SECTION_EDGE as u32
}
