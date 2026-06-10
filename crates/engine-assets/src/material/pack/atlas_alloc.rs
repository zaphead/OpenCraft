use image::RgbaImage;

use crate::atlas::{tile_uv_rect, UvRect};
use crate::layouts::{face_region, CubeFace, FACE_SIZE, ALBEDO_HEIGHT, ALBEDO_WIDTH};
use crate::material::resolved::AtlasBudget;

pub fn load_albedo(path: &std::path::Path) -> Result<RgbaImage, String> {
    let image = image::open(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let rgba = image.into_rgba8();
    if rgba.width() != ALBEDO_WIDTH || rgba.height() != ALBEDO_HEIGHT {
        return Err(format!(
            "expected {ALBEDO_WIDTH}x{ALBEDO_HEIGHT} at {}, got {}x{}",
            path.display(),
            rgba.width(),
            rgba.height()
        ));
    }
    Ok(rgba)
}

pub fn alloc_face_tile(
    pixels: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    grid: u32,
    next_slot: &mut u32,
    source: &RgbaImage,
    face: CubeFace,
) -> Result<UvRect, String> {
    if *next_slot >= AtlasBudget::max_tiles(grid) {
        return Err("block material atlas is full".into());
    }
    let slot = *next_slot;
    *next_slot += 1;
    let col = slot % grid;
    let row = slot / grid;
    blit_face(pixels, atlas_size, tile_size, col, row, source, face_region(face));
    Ok(tile_uv_rect(col, row, tile_size, atlas_size, atlas_size))
}

pub fn alloc_raw_tile(
    pixels: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    grid: u32,
    next_slot: &mut u32,
    source: &RgbaImage,
) -> Result<UvRect, String> {
    if *next_slot >= AtlasBudget::max_tiles(grid) {
        return Err("block material atlas is full".into());
    }
    let w = source.width().min(tile_size);
    let h = source.height().min(tile_size);
    let slot = *next_slot;
    *next_slot += 1;
    let col = slot % grid;
    let row = slot / grid;
    let dst_x = col * tile_size;
    let dst_y = row * tile_size;
    for py in 0..h {
        for px in 0..w {
            let pixel = source.get_pixel(px, py);
            let idx = (((dst_y + py) * atlas_size + dst_x + px) * 4) as usize;
            pixels[idx..idx + 4].copy_from_slice(&pixel.0);
        }
    }
    Ok(tile_uv_rect(col, row, tile_size, atlas_size, atlas_size))
}

fn blit_face(
    atlas: &mut [u8],
    atlas_size: u32,
    tile_size: u32,
    col: u32,
    row: u32,
    source: &RgbaImage,
    region: crate::layouts::PixelRect,
) {
    let dst_x = col * tile_size;
    let dst_y = row * tile_size;
    for py in 0..FACE_SIZE.min(tile_size) {
        for px in 0..FACE_SIZE.min(tile_size) {
            let pixel = source.get_pixel(region.x + px, region.y + py);
            let atlas_x = dst_x + px;
            let atlas_y = dst_y + py;
            let idx = ((atlas_y * atlas_size + atlas_x) * 4) as usize;
            atlas[idx..idx + 4].copy_from_slice(&pixel.0);
        }
    }
}
