use engine_world::CHUNK_SIZE;
use glam::IVec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugWorldKind {
    ThreeBlocks,
    #[default]
    RollingHills,
}

impl DebugWorldKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::ThreeBlocks => "Debug: 3 blocks",
            Self::RollingHills => "Debug: rolling hills",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::ThreeBlocks => Self::RollingHills,
            Self::RollingHills => Self::ThreeBlocks,
        }
    }
}

/// Active debug world profile for the local client. Absent on server.
#[derive(Debug, Clone, Copy, Default)]
pub struct ActiveDebugWorld(pub DebugWorldKind);

/// Chunk origins to mesh for the given debug world profile.
pub fn iter_mesh_chunks(world: DebugWorldKind) -> Vec<IVec3> {
    match world {
        DebugWorldKind::ThreeBlocks => vec![IVec3::ZERO],
        DebugWorldKind::RollingHills => rolling_hills_mesh_chunks().collect(),
    }
}

fn rolling_hills_mesh_chunks() -> impl Iterator<Item = IVec3> {
    let radius = crate::systems::terrain::ROLLING_HILLS_RADIUS;
    let min_xy = -radius;
    let max_xy = radius - 1;
    let min_cx = min_xy.div_euclid(CHUNK_SIZE);
    let max_cx = max_xy.div_euclid(CHUNK_SIZE);
    let min_cy = min_xy.div_euclid(CHUNK_SIZE);
    let max_cy = max_xy.div_euclid(CHUNK_SIZE);
    let max_cz = crate::systems::terrain::rolling_hills_max_surface_z().div_euclid(CHUNK_SIZE);

    (min_cx..=max_cx).flat_map(move |cx| {
        (min_cy..=max_cy).flat_map(move |cy| (0..=max_cz).map(move |cz| IVec3::new(cx, cy, cz)))
    })
}
