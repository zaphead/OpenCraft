//! Asset handles and synchronous block definition loading.

mod atlas;
mod blocks;
pub mod import;
mod layouts;
mod material;
mod poll;
mod server;

pub use atlas::{textures_asset_path, tile_uv_rect, TextureAtlas, UvRect, DEFAULT_GRID, DEFAULT_TILE_SIZE};
pub use blocks::{load_block_registry, BlockDefinition, BlockRegistry};
pub use import::{import_texture_pack, load_manifest, ImportManifest, ImportReport};
pub use layouts::{
    face_from_normal, face_region, CubeFace, PixelRect, UvLayoutId, ALBEDO_HEIGHT, ALBEDO_WIDTH,
    FACE_SIZE,
};
pub use material::{
    pack_block_materials, AnimIndex, DrawCategory, NeighborMask, ResolvedBlockMaterials,
    ResolvedFace, TintMode,
};
pub use poll::poll_assets_system;
pub use server::{assets_dir, blocks_asset_path, AssetServer, Handle, LoadState};
