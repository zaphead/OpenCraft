use std::path::{Path, PathBuf};

use crate::atlas::TextureAtlas;

#[derive(Debug, Clone)]
pub struct PlayerSkin {
    pub atlas: TextureAtlas,
}

pub fn player_skin_path(manifest_dir: &str) -> PathBuf {
    crate::textures_asset_path(manifest_dir).join("player").join("skin.png")
}

pub fn load_player_skin(manifest_dir: &str) -> PlayerSkin {
    let path = player_skin_path(manifest_dir);
    match load_skin_rgba(&path) {
        Ok(image) => PlayerSkin {
            atlas: TextureAtlas {
                tile_size: image.width(),
                width: image.width(),
                height: image.height(),
                pixels: image.into_raw(),
            },
        },
        Err(error) => {
            log::warn!("player skin load failed ({}): {error}", path.display());
            fallback_skin()
        }
    }
}

fn load_skin_rgba(path: &Path) -> Result<image::RgbaImage, String> {
    image::open(path)
        .map(|image| image.into_rgba8())
        .map_err(|error| format!("load {}: {error}", path.display()))
}

fn fallback_skin() -> PlayerSkin {
    let mut pixels = vec![0u8; 64 * 64 * 4];
    for y in 8..16 {
        for x in 8..16 {
            let i = ((y * 64 + x) * 4) as usize;
            pixels[i] = 200;
            pixels[i + 1] = 160;
            pixels[i + 2] = 120;
            pixels[i + 3] = 255;
        }
    }
    PlayerSkin {
        atlas: TextureAtlas {
            tile_size: 64,
            width: 64,
            height: 64,
            pixels,
        },
    }
}
