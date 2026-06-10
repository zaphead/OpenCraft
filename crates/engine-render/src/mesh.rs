use bytemuck::{Pod, Zeroable};
use engine_assets::{face_from_normal, AnimIndex, CubeFace, DrawCategory, ResolvedFace, TintMode, UvRect};
use engine_world::{BiomeMap, BlockPos};
use glam::Vec3;

pub const VERTEX_FLAG_OVERLAY: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub uv2: [f32; 2],
    pub tint_index: u32,
    pub flags: u32,
    pub anim_packed: u32,
}

#[derive(Debug, Default, Clone)]
pub struct SolidMesh {
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default, Clone)]
pub struct MeshBuckets {
    pub opaque: SolidMesh,
    pub cutout: SolidMesh,
}

impl MeshBuckets {
    pub fn push(&mut self, category: DrawCategory, mesh: &SolidMesh) {
        let target = match category {
            DrawCategory::Opaque => &mut self.opaque,
            DrawCategory::Cutout | DrawCategory::Transparent => &mut self.cutout,
        };
        let base = target.vertices.len() as u32;
        target.vertices.extend_from_slice(&mesh.vertices);
        target.indices.extend(mesh.indices.iter().map(|i| i + base));
    }

    pub fn is_empty(&self) -> bool {
        self.opaque.vertices.is_empty() && self.cutout.vertices.is_empty()
    }
}

pub fn tint_index_for(tint: TintMode, biome: &BiomeMap, position: BlockPos) -> u32 {
    match tint {
        TintMode::None => 0,
        TintMode::BiomeGrass | TintMode::BiomeFoliage => biome.biome_at(position) as u32,
    }
}

pub fn pack_vertex_anim(anim: Option<AnimIndex>, rect: UvRect) -> u32 {
    let Some(anim) = anim else {
        return 0;
    };
    let stride = (rect.max[0] - rect.min[0]).clamp(0.0, 1.0);
    let stride_u16 = (stride * 65535.0) as u32;
    let frame_count = anim.frame_count.min(255);
    let frametime = anim.frametime_ms.min(255);
    frame_count | (frametime << 8) | (stride_u16 << 16)
}

/// Side-face UVs keep texture V aligned with world +Z so grass fringe sits on the top edge.
#[cfg(test)]
pub fn side_face_grass_fringe_on_top_z(corners: [Vec3; 4], uvs: [[f32; 2]; 4]) -> bool {
    let mut top_z = f32::NEG_INFINITY;
    let mut bottom_z = f32::INFINITY;
    for corner in corners {
        top_z = top_z.max(corner.z);
        bottom_z = bottom_z.min(corner.z);
    }
    let top_v = uvs
        .iter()
        .zip(corners)
        .filter(|(_, corner)| (corner.z - top_z).abs() < 1e-5)
        .map(|(uv, _)| uv[1])
        .sum::<f32>()
        / corners
            .iter()
            .filter(|corner| (corner.z - top_z).abs() < 1e-5)
            .count()
            .max(1) as f32;
    let bottom_v = uvs
        .iter()
        .zip(corners)
        .filter(|(_, corner)| (corner.z - bottom_z).abs() < 1e-5)
        .map(|(uv, _)| uv[1])
        .sum::<f32>()
        / corners
            .iter()
            .filter(|corner| (corner.z - bottom_z).abs() < 1e-5)
            .count()
            .max(1) as f32;
    top_v < bottom_v
}

pub fn face_uvs(face: CubeFace, uv: UvRect) -> [[f32; 2]; 4] {
    let [u0, v0] = uv.min;
    let [u1, v1] = uv.max;
    match face {
        CubeFace::Right => [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
        CubeFace::Left => [
            [u0, v1],
            [u0, v0],
            [u1, v0],
            [u1, v1],
        ],
        CubeFace::Front => [[u0, v0], [u1, v0], [u1, v1], [u0, v1]],
        CubeFace::Top => [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
        CubeFace::Back => [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
        CubeFace::Bottom => [[u0, v0], [u0, v1], [u1, v1], [u1, v0]],
    }
}

pub fn face_corners(origin: Vec3, normal: [f32; 3]) -> [Vec3; 4] {
    let [nx, ny, nz] = normal;

    if nx > 0.0 {
        [
            origin + Vec3::new(1.0, 0.0, 0.0),
            origin + Vec3::new(1.0, 1.0, 0.0),
            origin + Vec3::new(1.0, 1.0, 1.0),
            origin + Vec3::new(1.0, 0.0, 1.0),
        ]
    } else if nx < 0.0 {
        [
            origin + Vec3::new(0.0, 0.0, 0.0),
            origin + Vec3::new(0.0, 0.0, 1.0),
            origin + Vec3::new(0.0, 1.0, 1.0),
            origin + Vec3::new(0.0, 1.0, 0.0),
        ]
    } else if ny > 0.0 {
        [
            origin + Vec3::new(0.0, 1.0, 1.0),
            origin + Vec3::new(1.0, 1.0, 1.0),
            origin + Vec3::new(1.0, 1.0, 0.0),
            origin + Vec3::new(0.0, 1.0, 0.0),
        ]
    } else if ny < 0.0 {
        [
            origin + Vec3::new(0.0, 0.0, 0.0),
            origin + Vec3::new(1.0, 0.0, 0.0),
            origin + Vec3::new(1.0, 0.0, 1.0),
            origin + Vec3::new(0.0, 0.0, 1.0),
        ]
    } else if nz > 0.0 {
        [
            origin + Vec3::new(0.0, 0.0, 1.0),
            origin + Vec3::new(1.0, 0.0, 1.0),
            origin + Vec3::new(1.0, 1.0, 1.0),
            origin + Vec3::new(0.0, 1.0, 1.0),
        ]
    } else {
        [
            origin + Vec3::new(0.0, 0.0, 0.0),
            origin + Vec3::new(0.0, 1.0, 0.0),
            origin + Vec3::new(1.0, 1.0, 0.0),
            origin + Vec3::new(1.0, 0.0, 0.0),
        ]
    }
}

pub fn append_face(
    mesh: &mut SolidMesh,
    origin: Vec3,
    normal: [f32; 3],
    face: &ResolvedFace,
    tint_index: u32,
) {
    let cube_face = face_from_normal(normal);
    let corners = face_corners(origin, normal);
    let uvs = face_uvs(cube_face, face.atlas_rect);
    let uv2_rect = face.uv2.unwrap_or(UvRect::BLACK);
    let uvs2 = face_uvs(cube_face, uv2_rect);
    let mut flags = 0u32;
    if face.has_overlay() {
        flags |= VERTEX_FLAG_OVERLAY;
    }
    let anim_packed = pack_vertex_anim(face.anim, face.atlas_rect);
    let base = mesh.vertices.len() as u32;

    for (corner, (tile_uv, tile_uv2)) in corners.iter().zip(uvs.iter().zip(uvs2.iter())) {
        mesh.vertices.push(MeshVertex {
            position: corner.to_array(),
            normal,
            uv: *tile_uv,
            uv2: *tile_uv2,
            tint_index,
            flags,
            anim_packed,
        });
    }

    mesh.indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}
