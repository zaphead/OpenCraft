use std::collections::HashMap;

use serde::Deserialize;

use crate::import::source::PackSource;
use crate::layouts::CubeFace;

#[derive(Debug, Clone)]
pub struct ResolvedCube {
    pub faces: HashMap<CubeFace, String>,
    pub overlay_sides: Option<String>,
    pub tint_grass: bool,
    pub tint_foliage: bool,
}

#[derive(Debug, Deserialize)]
struct BlockModel {
    #[serde(default)]
    parent: Option<String>,
    #[serde(default)]
    textures: HashMap<String, String>,
    #[serde(default)]
    elements: Vec<ModelElement>,
}

#[derive(Debug, Deserialize)]
struct ModelElement {
    #[serde(default)]
    faces: HashMap<String, ModelFace>,
}

#[derive(Debug, Deserialize)]
struct ModelFace {
    #[serde(default)]
    texture: Option<String>,
    #[serde(default)]
    tintindex: Option<i32>,
}

pub fn resolve_cube_model(source: &mut PackSource, model_name: &str) -> Result<ResolvedCube, String> {
    let path = format!("models/block/{model_name}.json");
    let json = source.read_mc_text(&path)?;
    let model: BlockModel =
        serde_json::from_str(&json).map_err(|error| format!("parse {path}: {error}"))?;
    resolve_model(model)
}

fn resolve_model(model: BlockModel) -> Result<ResolvedCube, String> {
    let mut textures: HashMap<String, String> = model
        .textures
        .into_iter()
        .map(|(key, value)| (key, normalize_texture_value(&value)))
        .collect();

    if matches!(
        model.parent.as_deref(),
        Some("block/cube_all") | Some("block/leaves")
    ) {
        expand_all_texture(&mut textures);
    }

    let mut faces = HashMap::new();
    let mut overlay_sides = textures.get("overlay").cloned();
    let mut tint_grass = false;
    let mut tint_foliage = false;

    if let Some(all) = textures.get("all").cloned() {
        for face in CubeFace::ALL {
            faces.insert(face, all.clone());
        }
    }

    if let (Some(top), Some(bottom), Some(side)) = (
        textures.get("top").cloned(),
        textures.get("bottom").cloned(),
        textures.get("side").cloned(),
    ) {
        faces.insert(CubeFace::Top, top);
        faces.insert(CubeFace::Bottom, bottom);
        for face in [
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
        ] {
            faces.insert(face, side.clone());
        }
    }

    for element in &model.elements {
        for (mc_face, face) in &element.faces {
            let Some(texture_key) = face.texture.as_deref() else {
                continue;
            };
            let lookup = normalize_texture_key(texture_key);
            let texture = textures
                .get(&lookup)
                .cloned()
                .ok_or_else(|| format!("unresolved texture key '{texture_key}'"))?;
            if let Some(cube_face) = mc_face_to_cube_face(mc_face) {
                faces.insert(cube_face, texture.clone());
            }
            if lookup == "overlay" {
                overlay_sides = Some(texture);
            }
            if face.tintindex == Some(0) {
                tint_grass = true;
            }
            if face.tintindex == Some(1) {
                tint_foliage = true;
            }
        }
    }

    if faces.len() != 6 {
        return Err(format!(
            "model must resolve to six faces, got {} (keys: {:?})",
            faces.len(),
            textures.keys().collect::<Vec<_>>()
        ));
    }

    Ok(ResolvedCube {
        faces,
        overlay_sides,
        tint_grass,
        tint_foliage,
    })
}

fn normalize_texture_value(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("block/") {
        format!("block/{rest}")
    } else if let Some(rest) = value.strip_prefix('#') {
        rest.to_string()
    } else {
        value.to_string()
    }
}

fn normalize_texture_key(key: &str) -> String {
    key.trim_start_matches('#').to_string()
}

fn expand_all_texture(textures: &mut HashMap<String, String>) {
    let Some(all) = textures.get("all").cloned() else {
        return;
    };
    textures.entry("top".into()).or_insert(all.clone());
    textures.entry("bottom".into()).or_insert(all.clone());
    textures.entry("side".into()).or_insert(all);
}

fn mc_face_to_cube_face(name: &str) -> Option<CubeFace> {
    match name {
        "up" => Some(CubeFace::Top),
        "down" => Some(CubeFace::Bottom),
        "west" => Some(CubeFace::Left),
        "east" => Some(CubeFace::Right),
        "south" => Some(CubeFace::Front),
        "north" => Some(CubeFace::Back),
        _ => None,
    }
}

pub fn read_texture_mcmeta(source: &mut PackSource, texture_ref: &str) -> Result<String, String> {
    let path = format!("textures/{texture_ref}.png.mcmeta");
    source.read_mc_text(&path)
}
