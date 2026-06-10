use image::RgbaImage;

use crate::import::model::ResolvedCube;
use crate::import::source::PackSource;
use crate::layouts::{face_region, CubeFace, ALBEDO_HEIGHT, ALBEDO_WIDTH};

pub fn compose_albedo(source: &mut PackSource, cube: &ResolvedCube) -> Result<RgbaImage, String> {
    let mut image = RgbaImage::new(ALBEDO_WIDTH, ALBEDO_HEIGHT);
    let dirt_ref = cube.faces.get(&CubeFace::Bottom);
    for face in CubeFace::ALL {
        let texture_ref = if cube.overlay_sides.is_some()
            && matches!(
                face,
                CubeFace::Left | CubeFace::Front | CubeFace::Right | CubeFace::Back
            )
        {
            dirt_ref.ok_or_else(|| "grass overlay block missing bottom/dirt face".to_string())?
        } else {
            cube.faces
                .get(&face)
                .ok_or_else(|| format!("missing face {face:?}"))?
        };
        let tile = source.read_texture_png(texture_ref)?;
        blit_face(&mut image, face, &tile);
    }
    Ok(image)
}

pub fn compose_overlay(source: &mut PackSource, overlay_ref: &str) -> Result<RgbaImage, String> {
    let tile = source.read_texture_png(overlay_ref)?;
    let mut image = RgbaImage::new(ALBEDO_WIDTH, ALBEDO_HEIGHT);
    for face in [
        CubeFace::Left,
        CubeFace::Front,
        CubeFace::Right,
        CubeFace::Back,
    ] {
        blit_face(&mut image, face, &tile);
    }
    Ok(image)
}

fn blit_face(dest: &mut RgbaImage, face: CubeFace, tile: &RgbaImage) {
    let region = face_region(face);
    for py in 0..region.h {
        for px in 0..region.w {
            let pixel = *tile.get_pixel(px, py);
            dest.put_pixel(region.x + px, region.y + py, pixel);
        }
    }
}
