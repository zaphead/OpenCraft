use crate::layouts::CubeFace;

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
