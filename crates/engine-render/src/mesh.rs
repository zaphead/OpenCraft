use bytemuck::{Pod, Zeroable};
use glam::{IVec3, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Debug, Default, Clone)]
pub struct SolidMesh {
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,
}

pub fn cube_mesh(origin: IVec3, size: f32, color: [f32; 3]) -> SolidMesh {
    let o = origin.as_vec3();
    let s = size;
    let min = o;
    let max = o + Vec3::splat(s);
    let mut mesh = SolidMesh::default();

    let faces = [
        ([1.0, 0.0, 0.0], [
            [max.x, min.y, min.z],
            [max.x, max.y, min.z],
            [max.x, max.y, max.z],
            [max.x, min.y, max.z],
        ]),
        ([-1.0, 0.0, 0.0], [
            [min.x, min.y, max.z],
            [min.x, max.y, max.z],
            [min.x, max.y, min.z],
            [min.x, min.y, min.z],
        ]),
        ([0.0, 1.0, 0.0], [
            [min.x, max.y, min.z],
            [max.x, max.y, min.z],
            [max.x, max.y, max.z],
            [min.x, max.y, max.z],
        ]),
        ([0.0, -1.0, 0.0], [
            [min.x, min.y, max.z],
            [max.x, min.y, max.z],
            [max.x, min.y, min.z],
            [min.x, min.y, min.z],
        ]),
        ([0.0, 0.0, 1.0], [
            [min.x, min.y, max.z],
            [min.x, max.y, max.z],
            [max.x, max.y, max.z],
            [max.x, min.y, max.z],
        ]),
        ([0.0, 0.0, -1.0], [
            [max.x, min.y, min.z],
            [max.x, max.y, min.z],
            [min.x, max.y, min.z],
            [min.x, min.y, min.z],
        ]),
    ];

    for (normal, corners) in faces {
        let base = mesh.vertices.len() as u32;
        for corner in corners {
            mesh.vertices.push(MeshVertex {
                position: corner,
                normal,
                color,
            });
        }
        mesh.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    mesh
}
