use std::path::Path;

use serde::Deserialize;

use crate::blocks::TintModeToml;
use crate::material::DrawCategory;

#[derive(Debug, Clone, Deserialize)]
pub struct ImportManifest {
    #[serde(default)]
    pub colormaps: ColormapManifest,
    #[serde(default)]
    pub blocks: Vec<BlockImportSpec>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ColormapManifest {
    #[serde(default)]
    pub grass: Option<String>,
    #[serde(default)]
    pub foliage: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockImportSpec {
    pub engine: String,
    pub model: String,
    #[serde(default)]
    pub draw: Option<DrawCategory>,
    #[serde(default)]
    pub tint: Option<TintModeToml>,
    #[serde(default)]
    pub overlay: bool,
}

pub fn load_manifest(path: &Path) -> Result<ImportManifest, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("read {}: {error}", path.display()))?;
    toml::from_str(&contents).map_err(|error| format!("parse {}: {error}", path.display()))
}
