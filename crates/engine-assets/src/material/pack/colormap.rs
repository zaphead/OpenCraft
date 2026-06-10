use std::path::Path;

use crate::atlas::UvRect;
use crate::material::pack::atlas_alloc::alloc_raw_tile;
use crate::material::resolved::AtlasBudget;

pub fn pack_colormaps(
    textures_dir: &Path,
    pixels: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    grid: u32,
    next_slot: &mut u32,
) -> Result<Option<UvRect>, String> {
    let grass = textures_dir.join("colormap/grass.png");
    if !grass.is_file() {
        return Ok(None);
    }
    let image = image::open(&grass)
        .map_err(|e| format!("read {}: {e}", grass.display()))?
        .into_rgba8();
    let rect = alloc_raw_tile(pixels, atlas_size, tile_size, grid, next_slot, &image)?;
    Ok(Some(rect))
}

pub fn ensure_atlas_budget(grid: u32, next_slot: u32) -> Result<(), String> {
    if next_slot >= AtlasBudget::max_tiles(grid) {
        return Err(format!(
            "block material atlas full: used {next_slot} tiles, max {}",
            AtlasBudget::max_tiles(grid)
        ));
    }
    Ok(())
}
