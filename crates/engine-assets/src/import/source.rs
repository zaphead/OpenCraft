use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

const MC_ASSETS_PREFIX: &str = "assets/minecraft/";

pub struct PackSource {
    root: PackRoot,
}

enum PackRoot {
    Dir(PathBuf),
    Zip(ZipArchive<std::fs::File>),
}

impl PackSource {
    pub fn open(path: &Path) -> Result<Self, String> {
        if path.is_dir() {
            return Ok(Self {
                root: PackRoot::Dir(path.to_path_buf()),
            });
        }

        let file = fs::File::open(path)
            .map_err(|error| format!("open pack {}: {error}", path.display()))?;
        let archive = ZipArchive::new(file)
            .map_err(|error| format!("read zip {}: {error}", path.display()))?;
        Ok(Self {
            root: PackRoot::Zip(archive),
        })
    }

    pub fn read_mc(&mut self, relative: &str) -> Result<Vec<u8>, String> {
        let key = normalize_mc_path(relative);
        match &mut self.root {
            PackRoot::Dir(root) => {
                let path = root.join(&key);
                fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))
            }
            PackRoot::Zip(archive) => {
                let mut file = archive
                    .by_name(&key)
                    .map_err(|_| format!("missing pack entry {key}"))?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)
                    .map_err(|error| format!("read {key}: {error}"))?;
                Ok(bytes)
            }
        }
    }

    pub fn read_mc_text(&mut self, relative: &str) -> Result<String, String> {
        let bytes = self.read_mc(relative)?;
        String::from_utf8(bytes).map_err(|error| format!("utf8 {relative}: {error}"))
    }

    pub fn read_texture_png(&mut self, texture_ref: &str) -> Result<image::RgbaImage, String> {
        let path = format!("textures/{texture_ref}.png");
        let bytes = self.read_mc(&path)?;
        let image = image::load_from_memory(&bytes)
            .map_err(|error| format!("decode {path}: {error}"))?
            .into_rgba8();
        if image.width() != 16 || image.height() != 16 {
            return Err(format!(
                "texture {path} must be 16x16, got {}x{}",
                image.width(),
                image.height()
            ));
        }
        Ok(image)
    }
}

fn normalize_mc_path(relative: &str) -> String {
    let trimmed = relative.trim_start_matches('/');
    if trimmed.starts_with("assets/minecraft/") {
        trimmed.to_string()
    } else {
        format!("{MC_ASSETS_PREFIX}{trimmed}")
    }
}
