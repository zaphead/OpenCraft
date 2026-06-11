use engine_assets::UvRect;
use glam::Vec3;

use crate::mesh::{MeshVertex, SolidMesh};

const SKIN_PX: f32 = 64.0;
pub const HUMANOID_PART_COUNT: usize = 6;
pub const HUMANOID_PART_HEAD: usize = 0;
pub const HUMANOID_PART_BODY: usize = 1;
pub const HUMANOID_PART_RIGHT_ARM: usize = 2;
pub const HUMANOID_PART_LEFT_ARM: usize = 3;
pub const HUMANOID_PART_RIGHT_LEG: usize = 4;
pub const HUMANOID_PART_LEFT_LEG: usize = 5;

pub fn humanoid_part_mask_without_head() -> u32 {
    let all = (1u32 << HUMANOID_PART_COUNT) - 1;
    all & !(1u32 << HUMANOID_PART_HEAD)
}

/// Steve model parts with pivot-local geometry (head, body, arms, legs).
pub struct HumanoidModelParts {
    pub meshes: [SolidMesh; HUMANOID_PART_COUNT],
    pub pivots: [Vec3; HUMANOID_PART_COUNT],
}

/// Minecraft `HumanoidModel` (Steve / 4px arms) in local space: feet at origin, Z-up.
pub fn build_humanoid_model_parts() -> HumanoidModelParts {
    let hip = Vec3::new(0.0, 0.0, 24.0 / 16.0);
    let specs: [(Vec3, Vec3, Vec3, (u32, u32), bool); HUMANOID_PART_COUNT] = [
        (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-4.0, -8.0, -4.0),
            Vec3::new(8.0, 8.0, 8.0),
            (0, 0),
            false,
        ),
        (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-4.0, 0.0, -2.0),
            Vec3::new(8.0, 12.0, 4.0),
            (16, 16),
            false,
        ),
        (
            Vec3::new(-5.0, 2.0, 0.0),
            Vec3::new(-3.0, -2.0, -2.0),
            Vec3::new(4.0, 12.0, 4.0),
            (40, 16),
            false,
        ),
        (
            Vec3::new(5.0, 2.0, 0.0),
            Vec3::new(-1.0, -2.0, -2.0),
            Vec3::new(4.0, 12.0, 4.0),
            (32, 48),
            false,
        ),
        (
            Vec3::new(-2.0, 12.0, 0.0),
            Vec3::new(-2.0, 0.0, -2.0),
            Vec3::new(4.0, 12.0, 4.0),
            (0, 16),
            false,
        ),
        (
            Vec3::new(2.0, 12.0, 0.0),
            Vec3::new(-2.0, 0.0, -2.0),
            Vec3::new(4.0, 12.0, 4.0),
            (16, 48),
            false,
        ),
    ];

    let mut meshes = std::array::from_fn(|_| SolidMesh::default());
    let mut pivots = [Vec3::ZERO; HUMANOID_PART_COUNT];
    for (i, (anchor, min, size, tex_off, mirror_right)) in specs.into_iter().enumerate() {
        let (mesh, pivot) = build_mc_part(hip, anchor, min, size, tex_off, mirror_right);
        meshes[i] = mesh;
        pivots[i] = pivot;
    }

    HumanoidModelParts { meshes, pivots }
}

pub fn build_player_model_mesh() -> SolidMesh {
    let parts = build_humanoid_model_parts();
    let mut mesh = SolidMesh::default();
    for (part, pivot) in parts.meshes.into_iter().zip(parts.pivots) {
        let offset = pivot.to_array();
        let base = mesh.vertices.len() as u32;
        for mut vertex in part.vertices {
            vertex.position[0] += offset[0];
            vertex.position[1] += offset[1];
            vertex.position[2] += offset[2];
            mesh.vertices.push(vertex);
        }
        for index in part.indices {
            mesh.indices.push(base + index);
        }
    }
    mesh
}

fn build_mc_part(
    hip: Vec3,
    anchor: Vec3,
    min: Vec3,
    size: Vec3,
    tex_off: (u32, u32),
    mirror_right: bool,
) -> (SolidMesh, Vec3) {
    let pivot = hip + mc_position(anchor);
    let mut mesh = SolidMesh::default();
    append_mc_box(&mut mesh, hip, anchor, min, size, tex_off, mirror_right);
    for vertex in &mut mesh.vertices {
        vertex.position[0] -= pivot.x;
        vertex.position[1] -= pivot.y;
        vertex.position[2] -= pivot.z;
    }
    (mesh, pivot)
}

/// MC `HumanoidModel` space (Y-up, faces +Z, right limb at −X) → OpenCraft model space
/// (Z-up, faces +Y, right limb at +X). X is negated when forward moves from MC +Z to +Y.
fn mc_position(offset: Vec3) -> Vec3 {
    Vec3::new(-offset.x, offset.z, -offset.y) / 16.0
}

fn skin_uv_rect(u: f32, v: f32, w: f32, h: f32) -> UvRect {
    let inset = 0.5 / SKIN_PX;
    UvRect {
        min: [
            u / SKIN_PX + inset,
            v / SKIN_PX + inset,
        ],
        max: [
            (u + w) / SKIN_PX - inset,
            (v + h) / SKIN_PX - inset,
        ],
    }
}

/// Matches Minecraft `ModelPart` cube face UV layout.
fn append_mc_box(
    mesh: &mut SolidMesh,
    pivot: Vec3,
    anchor: Vec3,
    min: Vec3,
    size: Vec3,
    tex_off: (u32, u32),
    mirror_right: bool,
) {
    let anchor_world = pivot + mc_position(anchor);
    let corner0 = anchor_world + mc_position(min);
    let corner1 = anchor_world + mc_position(min + size);
    let box_min = corner0.min(corner1);
    let box_max = corner0.max(corner1);
    let size_world = box_max - box_min;
    let (tu, tv) = tex_off;
    let dx = size.x;
    let dy = size.y;
    let dz = size.z;

    let faces: [([f32; 3], f32, f32, f32, f32); 6] = [
        ([0.0, 0.0, -1.0], tu as f32 + dz + dx, tv as f32 + dz, dx, dz), // down
        ([0.0, 0.0, 1.0], tu as f32 + dz, tv as f32, dx, dz),             // up
        ([0.0, -1.0, 0.0], tu as f32 + dz + dx + dz, tv as f32 + dz, dz, dy), // north / back
        ([0.0, 1.0, 0.0], tu as f32 + dz, tv as f32 + dz, dz, dy),            // south / front
        ([-1.0, 0.0, 0.0], tu as f32, tv as f32 + dz, dz, dy),                // west / left
        ([1.0, 0.0, 0.0], tu as f32 + dz + dx, tv as f32 + dz, dz, dy),       // east / right
    ];

    for (normal, u, v, w, h) in faces {
        let mirror_u = mirror_right && normal[0] > 0.0;
        append_oriented_face(
            mesh,
            box_min,
            size_world,
            normal,
            skin_uv_rect(u, v, w, h),
            mirror_u,
        );
    }
}

fn append_oriented_face(
    mesh: &mut SolidMesh,
    origin: Vec3,
    size: Vec3,
    normal: [f32; 3],
    uv: UvRect,
    mirror_u: bool,
) {
    let corners = face_corners(origin, size, normal);
    let uvs = face_uvs(normal, uv, mirror_u);
    let base = mesh.vertices.len() as u32;

    for (corner, tile_uv) in corners.iter().zip(uvs.iter()) {
        mesh.vertices.push(MeshVertex {
            position: corner.to_array(),
            normal,
            uv: *tile_uv,
            uv2: [0.0, 0.0],
            tint_index: 0,
            flags: 0,
            anim_packed: 0,
        });
    }

    mesh.indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn face_corners(origin: Vec3, size: Vec3, normal: [f32; 3]) -> [Vec3; 4] {
    let [nx, ny, nz] = normal;
    if nx > 0.0 {
        let x = origin.x + size.x;
        [
            Vec3::new(x, origin.y, origin.z),
            Vec3::new(x, origin.y + size.y, origin.z),
            Vec3::new(x, origin.y + size.y, origin.z + size.z),
            Vec3::new(x, origin.y, origin.z + size.z),
        ]
    } else if nx < 0.0 {
        [
            origin,
            Vec3::new(origin.x, origin.y, origin.z + size.z),
            Vec3::new(origin.x, origin.y + size.y, origin.z + size.z),
            Vec3::new(origin.x, origin.y + size.y, origin.z),
        ]
    } else if ny > 0.0 {
        let y = origin.y + size.y;
        [
            Vec3::new(origin.x, y, origin.z + size.z),
            Vec3::new(origin.x + size.x, y, origin.z + size.z),
            Vec3::new(origin.x + size.x, y, origin.z),
            Vec3::new(origin.x, y, origin.z),
        ]
    } else if ny < 0.0 {
        [
            origin,
            Vec3::new(origin.x + size.x, origin.y, origin.z),
            Vec3::new(origin.x + size.x, origin.y, origin.z + size.z),
            Vec3::new(origin.x, origin.y, origin.z + size.z),
        ]
    } else if nz > 0.0 {
        let z = origin.z + size.z;
        [
            Vec3::new(origin.x, origin.y, z),
            Vec3::new(origin.x + size.x, origin.y, z),
            Vec3::new(origin.x + size.x, origin.y + size.y, z),
            Vec3::new(origin.x, origin.y + size.y, z),
        ]
    } else {
        [
            origin,
            Vec3::new(origin.x, origin.y + size.y, origin.z),
            Vec3::new(origin.x + size.x, origin.y + size.y, origin.z),
            Vec3::new(origin.x + size.x, origin.y, origin.z),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_model_has_six_part_cubes() {
        let mesh = build_player_model_mesh();
        assert_eq!(mesh.vertices.len(), HUMANOID_PART_COUNT * 6 * 4);
        assert!(!mesh.vertices.is_empty());
    }

    #[test]
    fn humanoid_parts_are_pivot_local() {
        let parts = build_humanoid_model_parts();
        for mesh in &parts.meshes {
            assert_eq!(mesh.vertices.len(), 6 * 4);
        }
    }

    #[test]
    fn mc_right_limb_maps_to_positive_model_x() {
        let parts = build_humanoid_model_parts();
        assert!(
            parts.pivots[HUMANOID_PART_RIGHT_ARM].x > 0.0,
            "right arm pivot should be on +X"
        );
        assert!(
            parts.pivots[HUMANOID_PART_LEFT_ARM].x < 0.0,
            "left arm pivot should be on -X"
        );
        assert!(parts.pivots[HUMANOID_PART_RIGHT_LEG].x > 0.0);
        assert!(parts.pivots[HUMANOID_PART_LEFT_LEG].x < 0.0);
    }
}

fn face_uvs(normal: [f32; 3], uv: UvRect, mirror_u: bool) -> [[f32; 2]; 4] {
    let [u0, v0] = uv.min;
    let [u1, v1] = uv.max;
    let [nx, ny, nz] = normal;
    if nx > 0.0 {
        if mirror_u {
            [[u1, v1], [u0, v1], [u0, v0], [u1, v0]]
        } else {
            [[u0, v1], [u1, v1], [u1, v0], [u0, v0]]
        }
    } else if nx < 0.0 {
        [[u0, v1], [u0, v0], [u1, v0], [u1, v1]]
    } else if ny > 0.0 {
        [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]
    } else if ny < 0.0 {
        [[u0, v1], [u1, v1], [u1, v0], [u0, v0]]
    } else if nz > 0.0 {
        [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]
    } else {
        [[u0, v1], [u0, v0], [u1, v0], [u1, v1]]
    }
}
