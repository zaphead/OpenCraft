use std::collections::HashSet;

use engine_assets::BlockRegistry;
use engine_core::{SystemContext, Time};
use engine_input::InputState;
use engine_world::{BlockPos, SparseVoxelOctree};
use glam::Vec3;
use hecs::Entity;

use crate::components::{Collider, Mounted, Player, Transform, Velocity};

const WALK_SPEED: f32 = 6.0;
const JUMP_SPEED: f32 = 8.5;
const GRAVITY: f32 = 24.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

pub fn player_look_system(ctx: &mut SystemContext<'_>) {
    let Some(input) = ctx.resources.get::<InputState>().cloned() else {
        return;
    };

    let mounted: HashSet<Entity> = ctx
        .world
        .query::<(&Player, &Mounted)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for (entity, (_, transform)) in ctx.world.query_mut::<(&Player, &mut Transform)>() {
        if mounted.contains(&entity) {
            continue;
        }
        transform.yaw += input.look_delta.x * MOUSE_SENSITIVITY;
        transform.pitch = (transform.pitch - input.look_delta.y * MOUSE_SENSITIVITY)
            .clamp(-1.5, 1.5);
    }
}

pub fn player_movement_system(ctx: &mut SystemContext<'_>) {
    let Some(input) = ctx.resources.get::<InputState>().cloned() else {
        return;
    };

    let mounted: HashSet<Entity> = ctx
        .world
        .query::<(&Player, &Mounted)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    let grounded = ctx
        .world
        .query::<&Player>()
        .iter()
        .map(|(entity, _)| (entity, is_grounded(ctx, player_position(ctx, entity))))
        .collect::<Vec<_>>();

    for (entity, (_, transform, velocity)) in
        ctx.world.query_mut::<(&Player, &Transform, &mut Velocity)>()
    {
        if mounted.contains(&entity) {
            continue;
        }

        let forward = Vec3::new(transform.yaw.sin(), 0.0, transform.yaw.cos());
        let right = Vec3::new(forward.z, 0.0, -forward.x);
        let wish = (forward * input.move_axis.y + right * input.move_axis.x).normalize_or_zero();

        velocity.0.x = wish.x * WALK_SPEED;
        velocity.0.z = wish.z * WALK_SPEED;

        let grounded = grounded
            .iter()
            .find(|(id, _)| *id == entity)
            .map(|(_, grounded)| *grounded)
            .unwrap_or(false);
        if input.jump && grounded {
            velocity.0.y = JUMP_SPEED;
        }
    }
}

pub fn player_physics_system(ctx: &mut SystemContext<'_>) {
    let delta = ctx.resources.get::<Time>().map(|time| time.delta).unwrap_or(0.0);

    let mounted: HashSet<Entity> = ctx
        .world
        .query::<(&Player, &Mounted)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    let updates: Vec<(Entity, Vec3, Vec3, Vec3)> = ctx
        .world
        .query::<(&Player, &Transform, &Velocity, &Collider)>()
        .iter()
        .filter(|(entity, _)| !mounted.contains(entity))
        .map(|(entity, (_, transform, velocity, collider))| {
            (entity, transform.position, velocity.0, collider.half_extents)
        })
        .collect();

    for (entity, start_position, mut velocity, half_extents) in updates {
        velocity.y -= GRAVITY * delta;
        let mut position = start_position;

        for axis in 0..3 {
            let delta_axis = velocity[axis] * delta;
            if delta_axis == 0.0 {
                continue;
            }
            position[axis] += delta_axis;
            if collides_at(ctx, position, half_extents) {
                position[axis] -= delta_axis;
                velocity[axis] = 0.0;
            }
        }

        if let Ok(mut transform) = ctx.world.get::<&mut Transform>(entity) {
            transform.position = position;
        }
        if let Ok(mut velocity_ref) = ctx.world.get::<&mut Velocity>(entity) {
            velocity_ref.0 = velocity;
        }
    }
}

fn player_position(ctx: &SystemContext<'_>, entity: Entity) -> Vec3 {
    ctx.world
        .get::<&Transform>(entity)
        .map(|transform| transform.position)
        .unwrap_or(Vec3::ZERO)
}

fn is_grounded(ctx: &SystemContext<'_>, position: Vec3) -> bool {
    collides_at(
        ctx,
        position + Vec3::new(0.0, -1.05, 0.0),
        Vec3::new(0.35, 0.05, 0.35),
    )
}

pub(crate) fn collides_at(ctx: &SystemContext<'_>, position: Vec3, half_extents: Vec3) -> bool {
    let Some(registry) = ctx.resources.get::<BlockRegistry>() else {
        return false;
    };
    let Some(world) = ctx.resources.get::<SparseVoxelOctree>() else {
        return false;
    };

    let min = (position - half_extents).floor().as_ivec3();
    let max = (position + half_extents).ceil().as_ivec3();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let block = world.get_block(BlockPos::new(x, y, z));
                if registry.is_solid(block) {
                    return true;
                }
            }
        }
    }
    false
}
