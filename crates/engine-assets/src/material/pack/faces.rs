use crate::layouts::CubeFace;
use crate::material::TintMode;

/// Side faces with a grass overlay use the bottom (dirt) region from cube_v1 albedo.
pub fn albedo_sample_face(face: CubeFace, has_overlay: bool) -> CubeFace {
    if has_overlay && is_side_face(face) {
        CubeFace::Bottom
    } else {
        face
    }
}

pub fn face_tint_mode(face: CubeFace, tint: TintMode) -> TintMode {
    if face == CubeFace::Bottom && tint == TintMode::BiomeGrass {
        TintMode::None
    } else {
        tint
    }
}

pub fn is_side_face(face: CubeFace) -> bool {
    matches!(
        face,
        CubeFace::Left | CubeFace::Front | CubeFace::Right | CubeFace::Back
    )
}

pub fn parse_faces(spec: &str) -> Result<Vec<CubeFace>, String> {
    match spec {
        "all" => Ok(CubeFace::ALL.to_vec()),
        "side" => Ok(vec![
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
        ]),
        "top" => Ok(vec![CubeFace::Top]),
        "bottom" => Ok(vec![CubeFace::Bottom]),
        other => Err(format!("unknown face group '{other}'")),
    }
}
