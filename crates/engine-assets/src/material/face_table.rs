use std::collections::HashMap;

use engine_world::{BlockId, BlockState};

use crate::layouts::CubeFace;
use crate::material::ctm::NeighborMask;
use crate::material::resolved::ResolvedFace;

type FaceKey = (BlockId, CubeFace);
type StateFaceKey = (BlockId, BlockState, CubeFace);

#[derive(Debug, Default, Clone)]
pub struct BlockFaceTable {
    faces: HashMap<FaceKey, ResolvedFace>,
}

#[derive(Debug, Default, Clone)]
pub struct StateFaceTable {
    faces: HashMap<StateFaceKey, ResolvedFace>,
}

#[derive(Debug, Default, Clone)]
pub struct CtmFaceTable {
    /// (block_id, face, neighbor_mask_bits) → ResolvedFace
    faces: HashMap<(BlockId, CubeFace, u8), ResolvedFace>,
}

#[derive(Debug, Clone)]
pub struct MaterialTables {
    pub default_faces: BlockFaceTable,
    pub state_overrides: StateFaceTable,
    pub ctm_overrides: CtmFaceTable,
}

impl BlockFaceTable {
    pub fn insert(&mut self, block: BlockId, face: CubeFace, resolved: ResolvedFace) {
        self.faces.insert((block, face), resolved);
    }

    pub fn get(&self, block: BlockId, face: CubeFace) -> Option<&ResolvedFace> {
        self.faces.get(&(block, face))
    }
}

impl StateFaceTable {
    pub fn insert(
        &mut self,
        block: BlockId,
        state: BlockState,
        face: CubeFace,
        resolved: ResolvedFace,
    ) {
        self.faces.insert((block, state, face), resolved);
    }

    pub fn get(&self, block: BlockId, state: BlockState, face: CubeFace) -> Option<&ResolvedFace> {
        self.faces.get(&(block, state, face))
    }
}

impl CtmFaceTable {
    pub fn insert(
        &mut self,
        block: BlockId,
        face: CubeFace,
        mask: NeighborMask,
        resolved: ResolvedFace,
    ) {
        self.faces.insert((block, face, mask.bits()), resolved);
    }

    pub fn get(
        &self,
        block: BlockId,
        face: CubeFace,
        mask: NeighborMask,
    ) -> Option<&ResolvedFace> {
        self.faces.get(&(block, face, mask.bits()))
    }
}

impl MaterialTables {
    pub fn resolve<'a>(
        &'a self,
        block: BlockId,
        state: BlockState,
        face: CubeFace,
        neighbors: Option<NeighborMask>,
    ) -> &'a ResolvedFace {
        if let Some(mask) = neighbors {
            if let Some(face) = self.ctm_overrides.get(block, face, mask) {
                return face;
            }
        }
        if state.0 != 0 {
            if let Some(face) = self.state_overrides.get(block, state, face) {
                return face;
            }
        }
        self.default_faces.get(block, face).unwrap_or_else(|| {
            panic!("missing ResolvedFace for block {block:?} face {face:?}");
        })
    }
}
