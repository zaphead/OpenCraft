use engine_core::{Vec2, Vec3};

use crate::submit::{
    Camera, GroundShadow, ItemDraw, MeshData, ParticleDraw, PlayerDraw, UiQuad, Vertex, ANCHOR_UV,
};

pub(crate) fn emit_ui_quad(mesh: &mut MeshData, q: &UiQuad) {
    let i = mesh.vertices.len() as u32;
    let z = q.z;
    let n = [0.0, 0.0, 1.0];
    let c = q.color;
    mesh.vertices.push(Vertex {
        pos: [q.min.x, q.min.y, z],
        normal: n,
        uv: [q.uv_min.x, q.uv_min.y],
        color: c,
    });
    mesh.vertices.push(Vertex {
        pos: [q.max.x, q.min.y, z],
        normal: n,
        uv: [q.uv_max.x, q.uv_min.y],
        color: c,
    });
    mesh.vertices.push(Vertex {
        pos: [q.max.x, q.max.y, z],
        normal: n,
        uv: [q.uv_max.x, q.uv_max.y],
        color: c,
    });
    mesh.vertices.push(Vertex {
        pos: [q.min.x, q.max.y, z],
        normal: n,
        uv: [q.uv_min.x, q.uv_max.y],
        color: c,
    });
    mesh.indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
}

pub(crate) fn emit_particles(mesh: &mut MeshData, parts: &[ParticleDraw], cam: Camera) {
    let f = cam.look();
    let right = f.cross(Vec3::Y).normalize_or_zero();
    let up = right.cross(f).normalize_or_zero();
    for p in parts {
        let r = right * p.size;
        let u = up * p.size;
        quad(
            mesh,
            p.pos - r - u,
            p.pos + r - u,
            p.pos + r + u,
            p.pos - r + u,
            p.uv_min,
            p.uv_max,
            p.color,
            f,
        );
    }
}

pub(crate) fn emit_items(mesh: &mut MeshData, items: &[ItemDraw]) {
    for it in items {
        let s = if it.is_block { 0.25 } else { 0.2 };
        let yaw = it.yaw;
        let shade = [it.shade, it.shade, it.shade, 1.0];
        cube_at(mesh, it.pos, Vec3::splat(s), yaw, it.uv_min, it.uv_max, shade);
    }
}

/// One streak per body: a flat quad from the feet running away from the sun.
/// Uniform dark, hard edges, normal up so it reads as shade on the ground.
pub(crate) fn emit_ground_shadows(mesh: &mut MeshData, shadows: &[GroundShadow]) {
    for g in shadows {
        if g.len <= 0.01 || g.half <= 0.0 {
            continue;
        }
        let side = [-g.run[1], g.run[0]];
        let ax = g.foot.x - side[0] * g.half;
        let az = g.foot.z - side[1] * g.half;
        let bx = g.foot.x + side[0] * g.half;
        let bz = g.foot.z + side[1] * g.half;
        let cx = bx + g.run[0] * g.len;
        let cz = bz + g.run[1] * g.len;
        let dx = ax + g.run[0] * g.len;
        let dz = az + g.run[1] * g.len;
        let y = g.foot.y;
        // Wound CCW seen from above (+Y), matching the terrain faces.
        // Samples the reserved white atlas anchor: rgb dies in the zero
        // color, alpha stays 1, so the streak never depends on tile art.
        let (uv_min, uv_max) = (Vec2::new(ANCHOR_UV.0[0], ANCHOR_UV.0[1]), Vec2::new(ANCHOR_UV.1[0], ANCHOR_UV.1[1]));
        quad(
            mesh,
            Vec3::new(ax, y, az),
            Vec3::new(bx, y, bz),
            Vec3::new(cx, y, cz),
            Vec3::new(dx, y, dz),
            uv_min,
            uv_max,
            [0.0, 0.0, 0.0, 0.45],
            Vec3::Y,
        );
    }
}

pub(crate) fn emit_player(mesh: &mut MeshData, p: &PlayerDraw) {
    let yaw = p.yaw;
    let (sin, cos) = (yaw.sin(), yaw.cos());
    let origin = p.pos;
    let sneak = if p.sneaking { 0.08 } else { 0.0 };
    let tint = [p.shade, p.shade, p.shade, 1.0];
    if p.first_person_arm {
        let hand = origin
            + Vec3::new(0.25 * cos + 0.4 * sin, 1.15 - sneak, 0.25 * sin - 0.4 * cos);
        skin_box(mesh, hand, Vec3::new(0.12, 0.36, 0.12), yaw, SkinPart::Arm, tint);
        return;
    }
    skin_box(
        mesh,
        origin + Vec3::new(0.0, 1.5 - sneak, 0.0),
        Vec3::new(0.25, 0.25, 0.25),
        yaw,
        SkinPart::Head,
        tint,
    );
    skin_box(
        mesh,
        origin + Vec3::new(0.0, 0.84 - sneak, 0.0),
        Vec3::new(0.25, 0.375, 0.125),
        yaw,
        SkinPart::Body,
        tint,
    );
    skin_box(
        mesh,
        origin + Vec3::new(0.32 * cos, 0.84 - sneak, 0.32 * sin),
        Vec3::new(0.125, 0.375, 0.125),
        yaw,
        SkinPart::Arm,
        tint,
    );
    skin_box(
        mesh,
        origin + Vec3::new(-0.32 * cos, 0.84 - sneak, -0.32 * sin),
        Vec3::new(0.125, 0.375, 0.125),
        yaw,
        SkinPart::Arm,
        tint,
    );
    skin_box(
        mesh,
        origin + Vec3::new(0.1 * cos, 0.22, 0.1 * sin),
        Vec3::new(0.125, 0.375, 0.125),
        yaw,
        SkinPart::Leg,
        tint,
    );
    skin_box(
        mesh,
        origin + Vec3::new(-0.1 * cos, 0.22, -0.1 * sin),
        Vec3::new(0.125, 0.375, 0.125),
        yaw,
        SkinPart::Leg,
        tint,
    );
}

pub(crate) fn emit_held(mesh: &mut MeshData, p: &PlayerDraw) {
    let Some((uv0, uv1, block)) = p.held_uv else {
        return;
    };
    let tint = [p.shade, p.shade, p.shade, 1.0];
    let yaw = p.yaw;
    let (sin, cos) = (yaw.sin(), yaw.cos());
    let sneak = if p.sneaking { 0.08 } else { 0.0 };
    let hand = if p.first_person_arm {
        p.pos + Vec3::new(0.25 * cos + 0.4 * sin, 1.05 - sneak, 0.25 * sin - 0.4 * cos)
    } else {
        p.pos + Vec3::new(0.38 * cos, 0.7 - sneak, 0.38 * sin)
    };
    if block {
        cube_at(mesh, hand, Vec3::splat(0.18), yaw, uv0, uv1, tint);
    } else {
        cube_at(mesh, hand, Vec3::new(0.08, 0.32, 0.08), yaw, uv0, uv1, tint);
    }
}

enum SkinPart {
    Head,
    Body,
    Arm,
    Leg,
}

fn skin_uv(part: SkinPart) -> (Vec2, Vec2) {
    // Vanilla 64x64 layout (public format).
    match part {
        SkinPart::Head => (Vec2::new(8.0 / 64.0, 8.0 / 64.0), Vec2::new(16.0 / 64.0, 16.0 / 64.0)),
        SkinPart::Body => (Vec2::new(20.0 / 64.0, 20.0 / 64.0), Vec2::new(28.0 / 64.0, 32.0 / 64.0)),
        SkinPart::Arm => (Vec2::new(44.0 / 64.0, 20.0 / 64.0), Vec2::new(48.0 / 64.0, 32.0 / 64.0)),
        SkinPart::Leg => (Vec2::new(4.0 / 64.0, 20.0 / 64.0), Vec2::new(8.0 / 64.0, 32.0 / 64.0)),
    }
}

fn skin_box(mesh: &mut MeshData, center: Vec3, half: Vec3, yaw: f32, part: SkinPart, tint: [f32; 4]) {
    let (uv0, uv1) = skin_uv(part);
    cube_at(mesh, center, half, yaw, uv0, uv1, tint);
}

fn cube_at(mesh: &mut MeshData, center: Vec3, half: Vec3, yaw: f32, uv0: Vec2, uv1: Vec2, color: [f32; 4]) {
    let (s, c) = (yaw.sin(), yaw.cos());
    let rot = |v: Vec3| Vec3::new(v.x * c - v.z * s, v.y, v.x * s + v.z * c);
    let hx = half.x;
    let hy = half.y;
    let hz = half.z;
    let p = |x: f32, y: f32, z: f32| center + rot(Vec3::new(x, y, z));
    // 6 faces
    face(mesh, p(-hx, -hy, hz), p(hx, -hy, hz), p(hx, hy, hz), p(-hx, hy, hz), uv0, uv1, color, rot(Vec3::Z));
    face(mesh, p(hx, -hy, -hz), p(-hx, -hy, -hz), p(-hx, hy, -hz), p(hx, hy, -hz), uv0, uv1, color, rot(-Vec3::Z));
    face(mesh, p(-hx, -hy, -hz), p(-hx, -hy, hz), p(-hx, hy, hz), p(-hx, hy, -hz), uv0, uv1, color, rot(-Vec3::X));
    face(mesh, p(hx, -hy, hz), p(hx, -hy, -hz), p(hx, hy, -hz), p(hx, hy, hz), uv0, uv1, color, rot(Vec3::X));
    face(mesh, p(-hx, hy, hz), p(hx, hy, hz), p(hx, hy, -hz), p(-hx, hy, -hz), uv0, uv1, color, Vec3::Y);
    face(mesh, p(-hx, -hy, -hz), p(hx, -hy, -hz), p(hx, -hy, hz), p(-hx, -hy, hz), uv0, uv1, color, -Vec3::Y);
}

fn face(
    mesh: &mut MeshData,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    uv0: Vec2,
    uv1: Vec2,
    color: [f32; 4],
    n: Vec3,
) {
    quad(mesh, a, b, c, d, uv0, uv1, color, n);
}

fn quad(
    mesh: &mut MeshData,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    uv0: Vec2,
    uv1: Vec2,
    color: [f32; 4],
    n: Vec3,
) {
    let i = mesh.vertices.len() as u32;
    let n = n.normalize_or_zero().to_array();
    mesh.vertices.push(Vertex {
        pos: a.to_array(),
        normal: n,
        uv: [uv0.x, uv1.y],
        color,
    });
    mesh.vertices.push(Vertex {
        pos: b.to_array(),
        normal: n,
        uv: [uv1.x, uv1.y],
        color,
    });
    mesh.vertices.push(Vertex {
        pos: c.to_array(),
        normal: n,
        uv: [uv1.x, uv0.y],
        color,
    });
    mesh.vertices.push(Vertex {
        pos: d.to_array(),
        normal: n,
        uv: [uv0.x, uv0.y],
        color,
    });
    mesh.indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
}
