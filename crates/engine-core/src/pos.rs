use glam::{IVec3, Vec3};

use crate::{MIN_Y, SECTION_EDGE, SECTIONS_PER_CHUNK};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec3(p: Vec3) -> Self {
        Self {
            x: p.x.floor() as i32,
            y: p.y.floor() as i32,
            z: p.z.floor() as i32,
        }
    }

    pub const fn to_ivec3(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }

    pub fn chunk(self) -> ChunkPos {
        ChunkPos::from_block(self.x, self.z)
    }

    pub fn section_index(self) -> Option<usize> {
        if self.y < MIN_Y || self.y >= crate::MAX_Y {
            return None;
        }
        Some(((self.y - MIN_Y) / SECTION_EDGE) as usize)
    }

    pub fn local(self) -> (u32, u32, u32) {
        let lx = self.x.rem_euclid(SECTION_EDGE) as u32;
        let ly = (self.y - MIN_Y).rem_euclid(SECTION_EDGE) as u32;
        let lz = self.z.rem_euclid(SECTION_EDGE) as u32;
        (lx, ly, lz)
    }

    pub fn offset(self, dx: i32, dy: i32, dz: i32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }

    pub fn center(self) -> Vec3 {
        Vec3::new(self.x as f32 + 0.5, self.y as f32 + 0.5, self.z as f32 + 0.5)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn from_block(x: i32, z: i32) -> Self {
        Self {
            x: x.div_euclid(SECTION_EDGE),
            z: z.div_euclid(SECTION_EDGE),
        }
    }

    pub fn origin_block(self) -> BlockPos {
        BlockPos::new(self.x * SECTION_EDGE, MIN_Y, self.z * SECTION_EDGE)
    }

    pub fn chebyshev(self, other: Self) -> i32 {
        (self.x - other.x).abs().max((self.z - other.z).abs())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn from_block(p: BlockPos) -> Option<Self> {
        let idx = p.section_index()?;
        if idx >= SECTIONS_PER_CHUNK as usize {
            return None;
        }
        Some(Self {
            x: p.x.div_euclid(SECTION_EDGE),
            y: idx as i32,
            z: p.z.div_euclid(SECTION_EDGE),
        })
    }
}
