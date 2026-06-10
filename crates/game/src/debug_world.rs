use engine_world::CHUNK_SIZE;
use glam::IVec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugWorldKind {
    #[default]
    Flat,
}

impl DebugWorldKind {
    pub fn label(self) -> &'static str {
        "Debug: flat"
    }
}

/// Active debug world profile for the local client. Absent on server.
#[derive(Debug, Clone, Copy, Default)]
pub struct ActiveDebugWorld(pub DebugWorldKind);

/// Chunk origins to mesh for the given debug world profile.
pub fn iter_mesh_chunks(world: DebugWorldKind) -> Vec<IVec3> {
    match world {
        DebugWorldKind::Flat => flat_mesh_chunks().collect(),
    }
}

fn flat_mesh_chunks() -> impl Iterator<Item = IVec3> {
    let radius = crate::systems::terrain::FLAT_WORLD_RADIUS;
    let min_xy = -radius;
    let max_xy = radius - 1;
    let min_cx = min_xy.div_euclid(CHUNK_SIZE);
    let max_cx = max_xy.div_euclid(CHUNK_SIZE);
    let min_cy = min_xy.div_euclid(CHUNK_SIZE);
    let max_cy = max_xy.div_euclid(CHUNK_SIZE);
    let max_cz = crate::systems::terrain::FLAT_SURFACE_Z.div_euclid(CHUNK_SIZE);

    (min_cx..=max_cx).flat_map(move |cx| {
        (min_cy..=max_cy).flat_map(move |cy| (0..=max_cz).map(move |cz| IVec3::new(cx, cy, cz)))
    })
}
