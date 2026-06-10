use engine_world::{BlockId, BlockState};

use crate::atlas::{TextureAtlas, UvRect};
use crate::layouts::CubeFace;
use crate::material::ctm::NeighborMask;
use crate::material::face_table::MaterialTables;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawCategory {
    #[default]
    Opaque,
    Cutout,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TintMode {
    #[default]
    None,
    BiomeGrass,
    BiomeFoliage,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimIndex {
    pub frame_count: u32,
    pub frametime_ms: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedFace {
    pub atlas_rect: UvRect,
    pub draw_category: DrawCategory,
    pub uv2: Option<UvRect>,
    pub tint: TintMode,
    pub anim: Option<AnimIndex>,
}

impl ResolvedFace {
    pub fn opaque(rect: UvRect) -> Self {
        Self {
            atlas_rect: rect,
            draw_category: DrawCategory::Opaque,
            uv2: None,
            tint: TintMode::None,
            anim: None,
        }
    }

    pub fn has_overlay(&self) -> bool {
        self.uv2.is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasBudget {
    pub tile_size: u32,
    pub grid: u32,
    pub tiles_used: u32,
}

impl AtlasBudget {
    pub fn max_tiles(grid: u32) -> u32 {
        grid * grid - 1
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedBlockMaterials {
    pub atlas: TextureAtlas,
    pub colormap_atlas_rect: Option<UvRect>,
    pub budget: AtlasBudget,
    pub(crate) tables: MaterialTables,
}

impl ResolvedBlockMaterials {
    pub fn resolve_face(
        &self,
        block: BlockId,
        state: BlockState,
        face: CubeFace,
        neighbors: Option<NeighborMask>,
    ) -> &ResolvedFace {
        self.tables.resolve(block, state, face, neighbors)
    }

    pub fn tables(&self) -> &MaterialTables {
        &self.tables
    }
}
