use engine_core::{Vec3, MIN_Y, SECTIONS_PER_CHUNK};
use engine_render::{MeshData, Vertex};
use game::face_offset;
use world::ChunkSnapshot;

use crate::atlas::{block_tile, tile_uv};

/// Hard shadow tone baked into vertex color. One value, no blur: a shadowed
/// face is uniformly dark, so every shadow keeps a crisp block-edge boundary.
pub const SHADOW_TONE: f32 = 0.38;
/// Voxel steps marched toward the sun per face. Snapshot-local heightfield
/// occlusion: cheap on the mesh worker, free on the render thread.
const SHADOW_STEPS: i32 = 40;
const SHADOW_STEP: f32 = 0.5;

/// Sun for one mesh job. `up` is false at night: shadows sleep and the
/// world goes dark through the brightness uniform instead of vertex data,
/// so night costs no remesh.
#[derive(Clone, Copy, Debug)]
pub struct Sun {
    pub dir: Vec3,
    pub up: bool,
}

pub fn mesh_chunk(snap: &ChunkSnapshot, sun: Sun) -> MeshData {
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
                    if game::is_cross(id) || id == game::blocks::LILY {
                        emit_deco(&mut mesh, wx, wy, wz, id);
                        continue;
                    }
                    for face in 0..6u8 {
                        try_face(&mut mesh, snap, sun, wx, wy, wz, x, y, z, si, id, face);
                    }
                }
            }
        }
    }
    mesh
}

fn try_face(
    mesh: &mut MeshData,
    snap: &ChunkSnapshot,
    sun: Sun,
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
    let (dx, dy, dz) = face_offset(face);
    if neighbor_solid(snap, si, lx as i32 + dx, ly as i32 + dy, lz as i32 + dz) {
        return;
    }
    let tile = block_tile(id, face);
    let (uv0, uv1) = tile_uv(tile);
    let shade = face_shade(snap, sun, wx, wy, wz, face) ;
    let flag = block_flag(id);
    emit_face(mesh, wx, wy, wz, face, uv0, uv1, shade, flag, id);
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

/// Hard-edged cast darkening for one face. Marches from the face center
/// toward the sun through the same snapshot the mesh came from: a darkened
/// face always traces back to a real caster block. Returns 1.0 when lit.
fn face_shade(snap: &ChunkSnapshot, sun: Sun, wx: i32, wy: i32, wz: i32, face: u8) -> f32 {
    if !sun.up {
        return 1.0;
    }
    let (dx, dy, dz) = face_offset(face);
    let mut px = wx as f32 + 0.5 + dx as f32 * 0.5;
    let mut py = wy as f32 + 0.5 + dy as f32 * 0.5;
    let mut pz = wz as f32 + 0.5 + dz as f32 * 0.5;
    let sx = sun.dir.x * SHADOW_STEP;
    let sy = sun.dir.y * SHADOW_STEP;
    let sz = sun.dir.z * SHADOW_STEP;
    for _ in 0..SHADOW_STEPS {
        px += sx;
        py += sy;
        pz += sz;
        let (bx, by, bz) = (px.floor() as i32, py.floor() as i32, pz.floor() as i32);
        if bx == wx && by == wy && bz == wz {
            continue;
        }
        if snapshot_solid(snap, bx, by, bz) {
            return SHADOW_TONE;
        }
        // Leaving the snapshot means leaving known casters: lit, not guessed.
        // Snapshot-local by design (the cheap path): border faces stay lit
        // instead of inventing geometry, and the answer is a pure function of
        // (snapshot, sun step), so it never flickers between frames.
        if bx < snap.pos.x * 16
            || bx >= snap.pos.x * 16 + 16
            || bz < snap.pos.z * 16
            || bz >= snap.pos.z * 16 + 16
        {
            return 1.0;
        }
    }
    1.0
}

fn snapshot_solid(snap: &ChunkSnapshot, wx: i32, wy: i32, wz: i32) -> bool {
    let lx = wx - snap.pos.x * 16;
    let lz = wz - snap.pos.z * 16;
    if !(0..16).contains(&lx) || !(0..16).contains(&lz) {
        return false;
    }
    let si = (wy - MIN_Y).div_euclid(16);
    if si < 0 || si as usize >= snap.blocks.len() {
        return false;
    }
    let ly = (wy - MIN_Y).rem_euclid(16);
    let i = (ly as u32 * 256 + lz as u32 * 16 + lx as u32) as usize;
    game::is_solid(snap.blocks[si as usize][i])
}

/// Outward-CCW corner orders. Every triangle's geometric normal
/// `(b-a)x(c-a)` equals the face normal, so backface culling keeps exactly
/// the faces a viewer outside the block should see. Faces 0/1 used to be
/// wound the other way, which made east/west sides vanish from outside.
fn block_flag(id: u16) -> f32 {
    if id == game::blocks::SUNPETAL {
        0.75
    } else if id == game::blocks::STARBLOOM {
        0.40
    } else if matches!(id, game::blocks::GLOWVINE | game::blocks::CRYSTAL | game::blocks::KING_CAP) {
        0.55
    } else if game::is_leaf(id) || id == game::blocks::GRASS || game::is_cross(id) || id == game::blocks::TUFT {
        0.90
    } else {
        1.0
    }
}

fn tint_for(id: u16) -> [f32; 3] {
    match id {
        game::blocks::PINE_LEAVES | game::blocks::REDWOOD_LEAVES => [0.85, 0.55, 0.22],
        game::blocks::BIRCH_LEAVES => [0.7, 0.85, 0.4],
        game::blocks::FLOWER => [0.95, 0.45, 0.55],
        game::blocks::SUNPETAL => [0.98, 0.85, 0.2],
        _ => [1.0, 1.0, 1.0],
    }
}

fn emit_deco(mesh: &mut MeshData, x: i32, y: i32, z: i32, id: u16) {
    let tile = block_tile(id, 4);
    let (uv0, uv1) = tile_uv(tile);
    let flag = block_flag(id);
    let tint = tint_for(id);
    if id == game::blocks::LILY {
        emit_face(mesh, x, y, z, 3, uv0, uv1, 1.0, flag, id);
        return;
    }
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);
    let y1 = if id == game::blocks::REED { fy + 1.4 } else { fy + 1.0 };
    cross_quad(mesh, [fx, fy, fz], [fx + 1.0, y1, fz + 1.0], uv0, uv1, tint, flag);
    cross_quad(mesh, [fx + 1.0, fy, fz], [fx, y1, fz + 1.0], uv0, uv1, tint, flag);
}

fn cross_quad(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], uv0: engine_core::Vec2, uv1: engine_core::Vec2, tint: [f32; 3], flag: f32) {
    let i = mesh.vertices.len() as u32;
    let pts = [
        [a[0], a[1], a[2]],
        [b[0], a[1], b[2]],
        [b[0], b[1], b[2]],
        [a[0], b[1], a[2]],
    ];
    let uvs = [[uv0.x, uv1.y], [uv1.x, uv1.y], [uv1.x, uv0.y], [uv0.x, uv0.y]];
    for k in 0..4 {
        mesh.vertices.push(Vertex {
            pos: pts[k],
            normal: [0.0, 1.0, 0.0],
            uv: uvs[k],
            color: [tint[0], tint[1], tint[2], flag],
        });
    }
    mesh.indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3, i, i + 2, i + 1, i, i + 3, i + 2]);
}

fn emit_face(
    mesh: &mut MeshData,
    x: i32,
    y: i32,
    z: i32,
    face: u8,
    uv0: engine_core::Vec2,
    uv1: engine_core::Vec2,
    shade: f32,
    flag: f32,
    id: u16,
) {
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);
    let (n, pts): ([f32; 3], [[f32; 3]; 4]) = match face {
        0 => ([-1.0, 0.0, 0.0], [[fx, fy, fz], [fx, fy, fz + 1.0], [fx, fy + 1.0, fz + 1.0], [fx, fy + 1.0, fz]]),
        1 => ([1.0, 0.0, 0.0], [[fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy, fz], [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy + 1.0, fz + 1.0]]),
        2 => ([0.0, -1.0, 0.0], [[fx, fy, fz], [fx + 1.0, fy, fz], [fx + 1.0, fy, fz + 1.0], [fx, fy, fz + 1.0]]),
        3 => ([0.0, 1.0, 0.0], [[fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz]]),
        4 => ([0.0, 0.0, -1.0], [[fx, fy, fz], [fx, fy + 1.0, fz], [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy, fz]]),
        _ => ([0.0, 0.0, 1.0], [[fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx, fy + 1.0, fz + 1.0], [fx, fy, fz + 1.0]]),
    };
    // Orientation lives in this table, per face: on the four sides V runs
    // with world Y (image bottom row at min-Y) so side grain stands vertical;
    // top/bottom show the whole tile once. Faces 4/5 list bottom-top-top-
    // bottom corners, so they take a different row than faces 0/1.
    let i = mesh.vertices.len() as u32;
    let uvs = match face {
        4 | 5 => [
            [uv0.x, uv1.y],
            [uv0.x, uv0.y],
            [uv1.x, uv0.y],
            [uv1.x, uv1.y],
        ],
        _ => [
            [uv0.x, uv1.y],
            [uv1.x, uv1.y],
            [uv1.x, uv0.y],
            [uv0.x, uv0.y],
        ],
    };
    for k in 0..4 {
        mesh.vertices.push(Vertex {
            pos: pts[k],
            normal: n,
            uv: uvs[k],
            color: [shade * tint_for(id)[0], shade * tint_for(id)[1], shade * tint_for(id)[2], flag],
        });
    }
    mesh.indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::T_STONE;
    use engine_core::ChunkPos;

    fn lone_block(id: u16) -> ChunkSnapshot {
        let mut blocks = vec![[0u16; 4096]; SECTIONS_PER_CHUNK as usize];
        // Block at local (1, 40, 1): section 2 (MIN_Y=0), ly 8.
        blocks[2][8 * 256 + 1 * 16 + 1] = id;
        ChunkSnapshot {
            pos: ChunkPos::new(0, 0),
            blocks,
        }
    }

    fn tri_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ]
    }

    #[test]
    fn all_faces_wound_outward() {
        let sun = Sun {
            dir: Vec3::new(0.3, 1.0, 0.2).normalize(),
            up: true,
        };
        let mesh = mesh_chunk(&lone_block(3), sun);
        // A lone cube emits 6 faces x 2 triangles; every triangle's geometric
        // normal must agree with its stored face normal, or culling eats it.
        assert_eq!(mesh.indices.len(), 36);
        for tri in mesh.indices.chunks(3) {
            let p: Vec<_> = tri.iter().map(|i| mesh.vertices[*i as usize].pos).collect();
            let n = tri_normal(p[0], p[1], p[2]);
            let stored = mesh.vertices[tri[0] as usize].normal;
            let dot = n[0] * stored[0] + n[1] * stored[1] + n[2] * stored[2];
            assert!(dot > 0.0, "inward-wound triangle facing {stored:?}");
        }
    }

    #[test]
    fn hidden_faces_culled() {
        // Two stacked blocks share one face: 10 visible, not 12.
        let mut snap = lone_block(3);
        snap.blocks[2][9 * 256 + 1 * 16 + 1] = 3;
        let sun = Sun {
            dir: Vec3::new(0.3, 1.0, 0.2).normalize(),
            up: true,
        };
        let mesh = mesh_chunk(&snap, sun);
        assert_eq!(mesh.indices.len(), 60);
    }

    #[test]
    fn shade_traces_to_caster() {
        // A lone tall column east of a floor block with the sun in the east:
        // the floor block's faces must darken, and only because the column
        // stands between them and the sun.
        let mut snap = lone_block(1);
        let si = 2usize;
        for ly in 8..13 {
            snap.blocks[si][ly * 256 + 1 * 16 + 2] = 5;
        }
        let sun = Sun {
            dir: Vec3::new(1.0, 0.6, 0.05).normalize(),
            up: true,
        };
        let mesh = mesh_chunk(&snap, sun);
        let dark = mesh
            .vertices
            .iter()
            .filter(|v| v.color[0] < 0.99)
            .count();
        assert!(dark > 0, "column cast no shadow");
        let bare = mesh_chunk(&lone_block(1), sun);
        assert!(
            bare.vertices.iter().all(|v| v.color[0] > 0.99),
            "shadow without a caster"
        );
    }

    #[test]
    fn night_bakes_lit() {
        let sun = Sun {
            dir: Vec3::new(0.0, -1.0, 0.0),
            up: false,
        };
        let mesh = mesh_chunk(&lone_block(1), sun);
        assert!(mesh.vertices.iter().all(|v| v.color[0] > 0.99));
    }

    #[test]
    fn side_grain_runs_vertical() {
        // Stone wears one tile everywhere, so UVs isolate orientation from
        // art: every side face must put the image bottom row at min-Y and the
        // top row at max-Y, or side grain lies down.
        let (uv0, uv1) = tile_uv(T_STONE);
        let sun = Sun {
            dir: Vec3::new(0.3, 1.0, 0.2).normalize(),
            up: false,
        };
        let mesh = mesh_chunk(&lone_block(3), sun);
        assert_eq!(mesh.vertices.len(), 24);
        for v in mesh.vertices.chunks(4) {
            let n = v[0].normal;
            if n[1].abs() > 0.5 {
                continue;
            }
            for vert in v {
                let at_top = (vert.pos[1] - 41.0).abs() < 0.01;
                let want_v = if at_top { uv0.y } else { uv1.y };
                assert!(
                    (vert.uv[1] - want_v).abs() < 1e-6,
                    "side uv v={} at y={} (normal {n:?})",
                    vert.uv[1],
                    vert.pos[1]
                );
            }
        }
    }

    #[test]
    fn top_face_keeps_corners() {
        // The top face must show the whole tile exactly once: four distinct
        // corners, no stretch, no rotation of the mapping itself.
        let (uv0, uv1) = tile_uv(T_STONE);
        let sun = Sun {
            dir: Vec3::new(0.3, 1.0, 0.2).normalize(),
            up: false,
        };
        let mesh = mesh_chunk(&lone_block(3), sun);
        let top: Vec<_> = mesh
            .vertices
            .chunks(4)
            .find(|v| v[0].normal == [0.0, 1.0, 0.0])
            .expect("lone block has a top face")[..]
            .to_vec();
        let mut corners: Vec<_> = top.iter().map(|v| v.uv).collect();
        // total_cmp: UVs are finite by construction (tile math, no NaN).
        corners.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
        let mut want = [[uv0.x, uv0.y], [uv0.x, uv1.y], [uv1.x, uv0.y], [uv1.x, uv1.y]];
        want.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
        assert_eq!(corners, want);
    }
}
