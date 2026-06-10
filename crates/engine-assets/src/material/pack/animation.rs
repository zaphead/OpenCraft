use crate::layouts::CubeFace;
use crate::material::resolved::AnimIndex;

pub fn pack_face_animation(
    material_dir: &std::path::Path,
    face: CubeFace,
) -> Result<Option<AnimIndex>, String> {
    let _ = face;
    let mcmeta_path = material_dir.join("albedo.png.mcmeta");
    if !mcmeta_path.is_file() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&mcmeta_path)
        .map_err(|error| format!("read {}: {error}", mcmeta_path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|error| format!("parse {}: {error}", mcmeta_path.display()))?;
    let animation = value
        .get("animation")
        .ok_or_else(|| format!("missing animation in {}", mcmeta_path.display()))?;
    let frametime = animation.get("frametime").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
    let frame_count = animation
        .get("frames")
        .and_then(|v| v.as_array())
        .map(|a| a.len() as u32)
        .unwrap_or(1);
    Ok(Some(AnimIndex {
        frame_count: frame_count.max(1),
        frametime_ms: frametime.max(1),
    }))
}
