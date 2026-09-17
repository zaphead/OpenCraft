use engine_core::{BlockPos, ChunkPos, SECTIONS_PER_CHUNK};

use crate::palette::Section;
use crate::section_local_index;

#[derive(Clone, Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub sections: Vec<Section>,
}

impl Chunk {
    pub fn empty(pos: ChunkPos) -> Self {
        Self {
            pos,
            sections: (0..SECTIONS_PER_CHUNK).map(|_| Section::air()).collect(),
        }
    }

    pub fn get(&self, local_x: u32, y: i32, local_z: u32) -> u16 {
        let Some(si) = BlockPos::new(0, y, 0).section_index() else {
            return 0;
        };
        if si >= self.sections.len() {
            return 0;
        }
        let ly = (y - engine_core::MIN_Y).rem_euclid(16) as u32;
        self.sections[si].get(section_local_index(local_x, ly, local_z))
    }

    pub fn set(&mut self, local_x: u32, y: i32, local_z: u32, id: u16) {
        let Some(si) = BlockPos::new(0, y, 0).section_index() else {
            return;
        };
        if si >= self.sections.len() {
            return;
        }
        let ly = (y - engine_core::MIN_Y).rem_euclid(16) as u32;
        self.sections[si].set(section_local_index(local_x, ly, local_z), id);
    }
}

#[derive(Clone, Debug)]
pub struct ChunkSnapshot {
    pub pos: ChunkPos,
    pub blocks: Vec<[u16; 4096]>,
}

impl Chunk {
    pub fn snapshot(&self) -> ChunkSnapshot {
        ChunkSnapshot {
            pos: self.pos,
            blocks: self.sections.iter().map(|s| s.fill()).collect(),
        }
    }
}
