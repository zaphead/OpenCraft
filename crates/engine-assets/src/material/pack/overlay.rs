use std::collections::HashMap;
use std::path::Path;

use crate::atlas::UvRect;
use crate::blocks::BlockDefinition;
use crate::layouts::CubeFace;
use crate::material::pack::atlas_alloc::{alloc_face_tile, load_albedo};
use crate::material::pack::faces::parse_faces;

pub fn pack_overlay_layers(
    pixels: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    grid: u32,
    next_slot: &mut u32,
    definition: &BlockDefinition,
    material_dir: &Path,
) -> Result<HashMap<CubeFace, UvRect>, String> {
    let mut map = HashMap::new();
    for overlay in &definition.overlays {
        let path = material_dir.join(&overlay.path);
        if !path.is_file() {
            return Err(format!(
                "overlay for '{}' missing at {}",
                definition.name,
                path.display()
            ));
        }
        let image = load_albedo(&path)?;
        for face in parse_faces(&overlay.faces)? {
            let rect = alloc_face_tile(pixels, atlas_size, tile_size, grid, next_slot, &image, face)?;
            map.insert(face, rect);
        }
    }
    Ok(map)
}
