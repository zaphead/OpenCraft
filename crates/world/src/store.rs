use engine_core::{BlockPos, ChunkPos, EntityId, FxHashMap, Vec3, MIN_Y, MAX_Y};

use crate::chunk::{Chunk, ChunkSnapshot};
use crate::entities::{Entities, EntityKind};

pub struct World {
    chunks: FxHashMap<ChunkPos, Chunk>,
    pub entities: Entities,
    pub chests: FxHashMap<BlockPos, [Stack; 27]>,
}

pub type Stack = Option<(u16, u8)>;

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: FxHashMap::default(),
            entities: Entities::new(),
            chests: FxHashMap::default(),
        }
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.pos, chunk);
    }

    pub fn remove_chunk(&mut self, pos: ChunkPos) -> Option<Chunk> {
        self.chunks.remove(&pos)
    }

    pub fn chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }

    pub fn has_chunk(&self, pos: ChunkPos) -> bool {
        self.chunks.contains_key(&pos)
    }

    pub fn block(&self, p: BlockPos) -> u16 {
        if p.y < MIN_Y || p.y >= MAX_Y {
            return 0;
        }
        let c = p.chunk();
        let Some(chunk) = self.chunks.get(&c) else {
            return 0;
        };
        let lx = p.x.rem_euclid(16) as u32;
        let lz = p.z.rem_euclid(16) as u32;
        chunk.get(lx, p.y, lz)
    }

    pub fn set_block(&mut self, p: BlockPos, id: u16) {
        if p.y < MIN_Y || p.y >= MAX_Y {
            return;
        }
        let c = p.chunk();
        let chunk = self
            .chunks
            .entry(c)
            .or_insert_with(|| Chunk::empty(c));
        let lx = p.x.rem_euclid(16) as u32;
        let lz = p.z.rem_euclid(16) as u32;
        chunk.set(lx, p.y, lz, id);
        if id == 0 {
            self.chests.remove(&p);
        }
    }

    pub fn snapshot(&self, pos: ChunkPos) -> Option<ChunkSnapshot> {
        self.chunks.get(&pos).map(|c| c.snapshot())
    }

    pub fn loaded_chunks(&self) -> impl Iterator<Item = ChunkPos> + '_ {
        self.chunks.keys().copied()
    }

    pub fn spawn_item(&mut self, pos: Vec3, item: u16, count: u8) -> EntityId {
        let id = self.entities.spawn(EntityKind::Item, pos);
        if let Some(i) = self.entities.get(id) {
            self.entities.item[i] = Some((item, count.max(1)));
            self.entities.pickup_delay[i] = 10;
            self.entities.vel[i] = Vec3::new(
                ((pos.x * 12.3).sin() as f32) * 0.1,
                0.2,
                ((pos.z * 8.1).cos() as f32) * 0.1,
            );
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_block() {
        let mut w = World::new();
        let p = BlockPos::new(4, 10, 4);
        w.set_block(p, 3);
        assert_eq!(w.block(p), 3);
        w.set_block(p, 0);
        assert_eq!(w.block(p), 0);
    }
}
