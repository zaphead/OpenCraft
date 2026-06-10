mod animation;
mod atlas_alloc;
mod colormap;
mod faces;
mod overlay;

use std::path::Path;

use crate::atlas::{TextureAtlas, DEFAULT_GRID, DEFAULT_TILE_SIZE};
use crate::blocks::{BlockDefinition, BlockRegistry};
use crate::layouts::{CubeFace, UvLayoutId};
use crate::material::face_table::{BlockFaceTable, CtmFaceTable, MaterialTables, StateFaceTable};
use crate::material::resolved::{
    AtlasBudget, DrawCategory, ResolvedBlockMaterials, ResolvedFace,
};

use atlas_alloc::{alloc_face_tile, load_albedo};
use colormap::{ensure_atlas_budget, pack_colormaps};
use faces::parse_faces;
use overlay::pack_overlay_layers;

pub fn pack_block_materials(
    textures_dir: &Path,
    registry: &BlockRegistry,
) -> Result<ResolvedBlockMaterials, String> {
    let tile_size = DEFAULT_TILE_SIZE;
    let grid = DEFAULT_GRID;
    let atlas_size = tile_size * grid;
    let mut pixels = vec![0u8; (atlas_size * atlas_size * 4) as usize];
    let mut next_slot = 0u32;

    let mut default_faces = BlockFaceTable::default();
    let mut state_overrides = StateFaceTable::default();
    let ctm_overrides = CtmFaceTable::default();

    for definition in registry.definitions() {
        if !definition.solid {
            continue;
        }

        let material_dir = textures_dir.join(definition.material_path());
        let draw = definition.draw;

        match definition.layout {
            UvLayoutId::CubeV1 => {
                let albedo_path = material_dir.join("albedo.png");
                if !albedo_path.is_file() {
                    return Err(format!(
                        "solid block '{}' missing albedo at {}",
                        definition.name,
                        albedo_path.display()
                    ));
                }

                pack_cube_v1_block(
                    &mut pixels,
                    atlas_size,
                    tile_size,
                    grid,
                    &mut next_slot,
                    definition,
                    &material_dir,
                    &albedo_path,
                    draw,
                    &mut default_faces,
                    &mut state_overrides,
                )?;
            }
        }
    }

    let colormap_atlas_rect =
        pack_colormaps(textures_dir, &mut pixels, atlas_size, tile_size, grid, &mut next_slot)?;
    ensure_atlas_budget(grid, next_slot)?;

    Ok(ResolvedBlockMaterials {
        atlas: TextureAtlas {
            tile_size,
            width: atlas_size,
            height: atlas_size,
            pixels,
        },
        colormap_atlas_rect,
        budget: AtlasBudget {
            tile_size,
            grid,
            tiles_used: next_slot,
        },
        tables: MaterialTables {
            default_faces,
            state_overrides,
            ctm_overrides,
        },
    })
}

fn pack_cube_v1_block(
    pixels: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    grid: u32,
    next_slot: &mut u32,
    definition: &BlockDefinition,
    material_dir: &Path,
    albedo_path: &Path,
    draw: DrawCategory,
    default_faces: &mut BlockFaceTable,
    state_overrides: &mut StateFaceTable,
) -> Result<(), String> {
    let image = image::open(albedo_path)
        .map_err(|error| format!("read {}: {error}", albedo_path.display()))?
        .into_rgba8();

    if image.width() != crate::layouts::ALBEDO_WIDTH || image.height() != crate::layouts::ALBEDO_HEIGHT
    {
        return Err(format!(
            "albedo {} must be {}x{}, got {}x{}",
            albedo_path.display(),
            crate::layouts::ALBEDO_WIDTH,
            crate::layouts::ALBEDO_HEIGHT,
            image.width(),
            image.height()
        ));
    }

    let overlay_uvs = pack_overlay_layers(
        pixels,
        atlas_size,
        tile_size,
        grid,
        next_slot,
        definition,
        material_dir,
    )?;

    let tint = definition.tint_mode();

    for face in CubeFace::ALL {
        let rect = alloc_face_tile(pixels, atlas_size, tile_size, grid, next_slot, &image, face)?;
        let uv2 = overlay_uvs.get(&face).copied();
        let resolved = ResolvedFace {
            atlas_rect: rect,
            draw_category: draw,
            uv2,
            tint,
            anim: animation::pack_face_animation(material_dir, face)?,
        };
        default_faces.insert(definition.id, face, resolved);
    }

    for variant in &definition.state_variants {
        for face in parse_faces(&variant.faces)? {
            let source = variant
                .path
                .as_ref()
                .map(|p| material_dir.join(p))
                .unwrap_or_else(|| albedo_path.to_path_buf());
            let patch = load_albedo(&source)?;
            let rect = alloc_face_tile(pixels, atlas_size, tile_size, grid, next_slot, &patch, face)?;
            state_overrides.insert(
                definition.id,
                variant.state,
                face,
                ResolvedFace {
                    atlas_rect: rect,
                    draw_category: draw,
                    uv2: None,
                    tint,
                    anim: None,
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::load_block_registry;
    use crate::server::blocks_asset_path;

    #[test]
    fn grass_side_faces_have_overlay_uv2() {
        let client = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client");
        let registry = load_block_registry(&blocks_asset_path(client));
        let textures = crate::atlas::textures_asset_path(client);
        let packed = pack_block_materials(&textures, &registry).expect("pack");
        let grass = registry.id_by_name("grass").expect("grass");
        let side = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Front)
            .expect("side");
        assert!(side.uv2.is_some());
        let top = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Top)
            .expect("top");
        assert!(top.uv2.is_none());
    }

    #[test]
    fn leaves_pack_as_cutout() {
        let client = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client");
        let registry = load_block_registry(&blocks_asset_path(client));
        let textures = crate::atlas::textures_asset_path(client);
        let packed = pack_block_materials(&textures, &registry).expect("pack");
        let leaves = registry.id_by_name("leaves").expect("leaves");
        let face = packed
            .tables
            .default_faces
            .get(leaves, CubeFace::Top)
            .expect("face");
        assert_eq!(face.draw_category, DrawCategory::Cutout);
    }

    #[test]
    fn packs_grass_with_distinct_top_and_side_uvs() {
        let client = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client");
        let registry = load_block_registry(&blocks_asset_path(client));
        let textures = crate::atlas::textures_asset_path(client);
        let packed = pack_block_materials(&textures, &registry).expect("pack");
        let grass = registry.id_by_name("grass").expect("grass");
        let top = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Top)
            .expect("top");
        let side = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Front)
            .expect("side");
        assert_ne!(top.atlas_rect, side.atlas_rect);
    }
}
