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

/// Air, fluids, and soft flora. Ids match `game::blocks`. Thorn stays solid.
pub fn solid_id(id: u16) -> bool {
    !matches!(
        id,
        0 | 10 | 11 | 12 | 38 | 39 | 40 | 41 | 43 | 44 | 45 | 46 | 59 | 60
    )
}

impl VoxelSolid for World {
    fn solid(&self, x: i32, y: i32, z: i32) -> bool {
        if y < MIN_Y {
            return true;
        }
        solid_id(self.block(BlockPos::new(x, y, z)))
    }
}

pub fn section_local_index(lx: u32, ly: u32, lz: u32) -> usize {
    (ly * 16 * 16 + lz * 16 + lx) as usize
}

pub fn in_section(lx: u32, ly: u32, lz: u32) -> bool {
    lx < SECTION_EDGE as u32 && ly < SECTION_EDGE as u32 && lz < SECTION_EDGE as u32
}
