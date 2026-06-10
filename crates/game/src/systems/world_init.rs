use engine_core::SystemContext;
use glam::Vec3;

use crate::components::{Collider, Player, Transform, Velocity, WorldInitialized};
use crate::systems::terrain::spawn_height;

pub fn spawn_player_system(ctx: &mut SystemContext<'_>) {
    let initialized = ctx
        .resources
        .get::<WorldInitialized>()
        .map(|flag| flag.0)
        .unwrap_or(false);
    if !initialized {
        return;
    }

    if ctx.world.query::<&Player>().iter().next().is_some() {
        return;
    }

    let y = spawn_height(0, 0) as f32;
    ctx.world.spawn((
        Player,
        Transform {
            position: Vec3::new(0.5, y, 0.5),
            yaw: 0.0,
            pitch: 0.0,
        },
        Velocity::default(),
        Collider {
            half_extents: Vec3::new(0.35, 0.9, 0.35),
        },
    ));
}
