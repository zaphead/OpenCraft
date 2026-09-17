use engine_core::{MIN_Y, SECTIONS_PER_CHUNK};
use engine_render::{MeshData, Vertex};
use game::face_offset;
use world::ChunkSnapshot;

use crate::atlas::{block_tile, tile_uv};

pub fn mesh_chunk(snap: &ChunkSnapshot) -> MeshData {
    let mut mesh = MeshData::default();
    for si in 0..snap.blocks.len().min(SECTIONS_PER_CHUNK as usize) {
        let base_y = MIN_Y + si as i32 * 16;
        let sec = &snap.blocks[si];
        for y in 0..16u32 {
            for z in 0..16u32 {
                for x in 0..16u32 {
                    let i = (y * 256 + z * 16 + x) as usize;
                    let id = sec[i];
                    if id == 0 {
                        continue;
                    }
                    let wx = snap.pos.x * 16 + x as i32;
                    let wy = base_y + y as i32;
                    let wz = snap.pos.z * 16 + z as i32;
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 0);
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 1);
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 2);
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 3);
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 4);
                    try_face(&mut mesh, snap, wx, wy, wz, x, y, z, si, id, 5);
                }
            }
        }
    }
    mesh
}

fn try_face(
    mesh: &mut MeshData,
    snap: &ChunkSnapshot,
    wx: i32,
    wy: i32,
    wz: i32,
    lx: u32,
    ly: u32,
    lz: u32,
    si: usize,
    id: u16,
    face: u8,
) {
    let (dx, dy, dz) = if face > 5 {
        return;
    } else {
        face_offset(face)
    };
    if neighbor_solid(snap, si, lx as i32 + dx, ly as i32 + dy, lz as i32 + dz) {
        return;
    }
    let tile = block_tile(id, face);
    let (uv0, uv1) = tile_uv(tile);
    emit_face(mesh, wx, wy, wz, face, uv0, uv1);
}

fn neighbor_solid(snap: &ChunkSnapshot, si: usize, lx: i32, ly: i32, lz: i32) -> bool {
    if !(0..16).contains(&lx) || !(0..16).contains(&lz) {
        return false;
    }
    let mut si = si as i32;
    let mut ly = ly;
    if ly < 0 {
        si -= 1;
        ly += 16;
    }
    if ly >= 16 {
        si += 1;
        ly -= 16;
    }
    if si < 0 || si >= snap.blocks.len() as i32 {
        return false;
    }
    let i = (ly as u32 * 256 + lz as u32 * 16 + lx as u32) as usize;
    game::is_solid(snap.blocks[si as usize][i])
}

fn emit_face(mesh: &mut MeshData, x: i32, y: i32, z: i32, face: u8, uv0: engine_core::Vec2, uv1: engine_core::Vec2) {
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);
    let (n, pts): ([f32; 3], [[f32; 3]; 4]) = match face {
        0 => ([-1.0, 0.0, 0.0], [[fx, fy, fz + 1.0], [fx, fy, fz], [fx, fy + 1.0, fz], [fx, fy + 1.0, fz + 1.0]]),
        1 => ([1.0, 0.0, 0.0], [[fx + 1.0, fy, fz], [fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz]]),
        2 => ([0.0, -1.0, 0.0], [[fx, fy, fz], [fx + 1.0, fy, fz], [fx + 1.0, fy, fz + 1.0], [fx, fy, fz + 1.0]]),
        3 => ([0.0, 1.0, 0.0], [[fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz]]),
        4 => ([0.0, 0.0, -1.0], [[fx, fy, fz], [fx, fy + 1.0, fz], [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy, fz]]),
        _ => ([0.0, 0.0, 1.0], [[fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx, fy + 1.0, fz + 1.0], [fx, fy, fz + 1.0]]),
    };
    let i = mesh.vertices.len() as u32;
    let uvs = [
        [uv0.x, uv1.y],
        [uv1.x, uv1.y],
        [uv1.x, uv0.y],
        [uv0.x, uv0.y],
    ];
    for k in 0..4 {
        mesh.vertices.push(Vertex {
            pos: pts[k],
            normal: n,
            uv: uvs[k],
            color: [1.0, 1.0, 1.0, 1.0],
        });
    }
    mesh.indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
}
