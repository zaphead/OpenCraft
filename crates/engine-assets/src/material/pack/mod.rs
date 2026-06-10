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
use faces::{albedo_sample_face, face_tint_mode, parse_faces};
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
        let sample = albedo_sample_face(face, overlay_uvs.contains_key(&face));
        let rect = alloc_face_tile(pixels, atlas_size, tile_size, grid, next_slot, &image, sample)?;
        let uv2 = overlay_uvs.get(&face).copied();
        let resolved = ResolvedFace {
            atlas_rect: rect,
            draw_category: draw,
            uv2,
            tint: face_tint_mode(face, tint),
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
    use crate::material::TintMode;
    use crate::server::blocks_asset_path;

    #[test]
    fn grass_bottom_face_has_no_biome_tint() {
        let client = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client");
        let registry = load_block_registry(&blocks_asset_path(client));
        let textures = crate::atlas::textures_asset_path(client);
        let packed = pack_block_materials(&textures, &registry).expect("pack");
        let grass = registry.id_by_name("grass").expect("grass");
        let bottom = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Bottom)
            .expect("bottom");
        let top = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Top)
            .expect("top");
        assert_eq!(bottom.tint, TintMode::None);
        assert_eq!(top.tint, TintMode::BiomeGrass);
    }

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
    fn grass_side_base_matches_dirt_not_black() {
        let client = concat!(env!("CARGO_MANIFEST_DIR"), "/../../client");
        let registry = load_block_registry(&blocks_asset_path(client));
        let textures = crate::atlas::textures_asset_path(client);
        let packed = pack_block_materials(&textures, &registry).expect("pack");
        let grass = registry.id_by_name("grass").expect("grass");
        let dirt = registry.id_by_name("dirt").expect("dirt");
        let side = packed
            .tables
            .default_faces
            .get(grass, CubeFace::Front)
            .expect("side");
        let dirt_face = packed
            .tables
            .default_faces
            .get(dirt, CubeFace::Top)
            .expect("dirt");
        let side_px = atlas_center_pixel(&packed, side.atlas_rect);
        let dirt_px = atlas_center_pixel(&packed, dirt_face.atlas_rect);
        assert!(
            side_px[0] > 20 && side_px[1] > 10,
            "grass side base should be dirt-colored, got {:?}",
            side_px
        );
        assert!(
            (side_px[0] as i32 - dirt_px[0] as i32).abs() < 30,
            "grass side base should match dirt, side={side_px:?} dirt={dirt_px:?}"
        );
    }

    fn atlas_center_pixel(materials: &ResolvedBlockMaterials, rect: crate::atlas::UvRect) -> [u8; 4] {
        let atlas = &materials.atlas;
        let u = (rect.min[0] + rect.max[0]) * 0.5;
        let v = (rect.min[1] + rect.max[1]) * 0.5;
        let x = (u * atlas.width as f32) as u32;
        let y = (v * atlas.height as f32) as u32;
        let idx = ((y * atlas.width + x) * 4) as usize;
        let p = &atlas.pixels[idx..idx + 4];
        [p[0], p[1], p[2], p[3]]
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
