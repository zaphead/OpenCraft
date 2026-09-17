use engine_core::Vec3;
use engine_render::ParticleDraw;

use crate::atlas::tile_uv;

pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub tile: u32,
}

pub fn burst(out: &mut Vec<Particle>, pos: Vec3, tile: u32, n: i32, speed: f32) {
    for i in 0..n {
        let a = i as f32 * 2.1;
        out.push(Particle {
            pos: pos + Vec3::new((a.sin()) * 0.2, (i % 3) as f32 * 0.1, a.cos() * 0.2),
            vel: Vec3::new(a.sin() * speed, 0.12 + (i as f32) * 0.01, a.cos() * speed),
            life: 0.6,
            tile,
        });
    }
}

pub fn tick(ps: &mut Vec<Particle>, dt: f32) {
    for p in ps.iter_mut() {
        p.vel.y -= 1.2 * dt;
        p.pos += p.vel * dt * 8.0;
        p.life -= dt;
    }
    ps.retain(|p| p.life > 0.0);
}

pub fn draw(ps: &[Particle]) -> Vec<ParticleDraw> {
    ps.iter()
        .map(|p| {
            let (uv0, uv1) = tile_uv(p.tile);
            ParticleDraw {
                pos: p.pos,
                size: 0.06 * p.life.max(0.2),
                uv_min: uv0,
                uv_max: uv1,
                color: [1.0, 1.0, 1.0, (p.life / 0.6).clamp(0.0, 1.0)],
            }
        })
        .collect()
}
