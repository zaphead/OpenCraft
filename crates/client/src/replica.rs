use engine_core::{BlockPos, ChunkPos};
use world::{Chunk, Section, World};

pub fn apply_chunk(world: &mut World, pos: ChunkPos, sections: &[(u8, Vec<u16>, Vec<u16>)]) {
    let mut chunk = Chunk::empty(pos);
    for (sy, pal, idx) in sections {
        let i = *sy as usize;
        if i < chunk.sections.len() {
            chunk.sections[i] = Section::from_parts(pal.clone(), idx.clone());
        }
    }
    world.insert_chunk(chunk);
}

pub fn apply_block(world: &mut World, pos: BlockPos, id: u16) {
    world.set_block(pos, id);
}
