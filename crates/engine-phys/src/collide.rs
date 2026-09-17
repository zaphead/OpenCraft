use engine_core::Vec3;

use crate::Aabb;

pub trait VoxelSolid {
    fn solid(&self, x: i32, y: i32, z: i32) -> bool;
}

const EPSILON: f32 = 1.0e-5;

pub fn move_colliding(
    aabb: &mut Aabb,
    vel: &mut Vec3,
    world: &impl VoxelSolid,
    step_height: f32,
) -> Collision {
    let mut hit = Collision::default();
    let mut dy = vel.y;
    dy = clip_axis(aabb, world, 1, dy);
    aabb.min.y += dy;
    aabb.max.y += dy;
    if dy != vel.y {
        hit.y = true;
        vel.y = 0.0;
    }

    let saved = *aabb;
    let want_x = vel.x;
    let want_z = vel.z;

    let mut dx = want_x;
    dx = clip_axis(aabb, world, 0, dx);
    aabb.min.x += dx;
    aabb.max.x += dx;

    let mut dz = want_z;
    dz = clip_axis(aabb, world, 2, dz);
    aabb.min.z += dz;
    aabb.max.z += dz;

    let blocked_h = dx != want_x || dz != want_z;
    if blocked_h && step_height > 0.0 {
        *aabb = saved;
        let mut up = step_height;
        up = clip_axis(aabb, world, 1, up);
        aabb.min.y += up;
        aabb.max.y += up;

        let mut sdx = want_x;
        sdx = clip_axis(aabb, world, 0, sdx);
        aabb.min.x += sdx;
        aabb.max.x += sdx;

        let mut sdz = want_z;
        sdz = clip_axis(aabb, world, 2, sdz);
        aabb.min.z += sdz;
        aabb.max.z += sdz;

        let mut down = -up;
        down = clip_axis(aabb, world, 1, down);
        aabb.min.y += down;
        aabb.max.y += down;

        let stepped = sdx.abs() + sdz.abs() > dx.abs() + dz.abs() + 1.0e-4;
        if !stepped {
            *aabb = saved;
            aabb.min.x += dx;
            aabb.max.x += dx;
            aabb.min.z += dz;
            aabb.max.z += dz;
            if dx != want_x {
                hit.x = true;
                vel.x = 0.0;
            }
            if dz != want_z {
                hit.z = true;
                vel.z = 0.0;
            }
        }
    } else {
        if dx != want_x {
            hit.x = true;
            vel.x = 0.0;
        }
        if dz != want_z {
            hit.z = true;
            vel.z = 0.0;
        }
    }

    hit
}

fn clip_axis(aabb: &Aabb, world: &impl VoxelSolid, axis: i32, mut delta: f32) -> f32 {
    if delta.abs() < EPSILON {
        return 0.0;
    }
    let moved = aabb.expand(axis_vec(axis, delta));
    for (x, y, z) in voxel_range(moved) {
        if !world.solid(x, y, z) {
            continue;
        }
        let block = Aabb {
            min: Vec3::new(x as f32, y as f32, z as f32),
            max: Vec3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0),
        };
        delta = match axis {
            0 => aabb.clip_x(block, delta),
            1 => aabb.clip_y(block, delta),
            2 => aabb.clip_z(block, delta),
            _ => delta,
        };
    }
    if delta.abs() < EPSILON {
        0.0
    } else {
        delta
    }
}

fn axis_vec(axis: i32, v: f32) -> Vec3 {
    match axis {
        0 => Vec3::new(v, 0.0, 0.0),
        1 => Vec3::new(0.0, v, 0.0),
        _ => Vec3::new(0.0, 0.0, v),
    }
}

fn voxel_range(aabb: Aabb) -> impl Iterator<Item = (i32, i32, i32)> {
    let min_x = aabb.min.x.floor() as i32;
    let min_y = aabb.min.y.floor() as i32;
    let min_z = aabb.min.z.floor() as i32;
    let max_x = aabb.max.x.floor() as i32;
    let max_y = aabb.max.y.floor() as i32;
    let max_z = aabb.max.z.floor() as i32;
    (min_x..=max_x).flat_map(move |x| {
        (min_y..=max_y).flat_map(move |y| (min_z..=max_z).map(move |z| (x, y, z)))
    })
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Collision {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}
