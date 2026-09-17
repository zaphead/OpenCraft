use engine_core::Vec3;

use crate::aabb::Aabb;
use crate::collide::{move_colliding, VoxelSolid};

pub const PLAYER_WIDTH: f32 = 0.6;
pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_SNEAK_HEIGHT: f32 = 1.5;
pub const PLAYER_EYE: f32 = 1.62;
pub const PLAYER_SNEAK_EYE: f32 = 1.27;
const GRAVITY: f32 = 0.08;
const AIR_DRAG: f32 = 0.98;
const HORIZ_DRAG: f32 = 0.91;
const SLIP: f32 = 0.6;
const JUMP: f32 = 0.42;
const STEP: f32 = 0.6;
const AIR_ACCEL: f32 = 0.02;
const GROUND_ACCEL: f32 = 0.1;
const SNEAK_MUL: f32 = 0.3;
const SPRINT_MUL: f32 = 1.3;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
    pub sneaking: bool,
    pub sprinting: bool,
    pub fall_distance: f32,
}

impl Body {
    pub fn at(pos: Vec3) -> Self {
        Self {
            pos,
            vel: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: false,
            sneaking: false,
            sprinting: false,
            fall_distance: 0.0,
        }
    }

    pub fn height(self) -> f32 {
        if self.sneaking {
            PLAYER_SNEAK_HEIGHT
        } else {
            PLAYER_HEIGHT
        }
    }

    pub fn eye_height(self) -> f32 {
        if self.sneaking {
            PLAYER_SNEAK_EYE
        } else {
            PLAYER_EYE
        }
    }

    pub fn aabb(self) -> Aabb {
        let h = self.height();
        let hw = PLAYER_WIDTH * 0.5;
        Aabb {
            min: Vec3::new(self.pos.x - hw, self.pos.y, self.pos.z - hw),
            max: Vec3::new(self.pos.x + hw, self.pos.y + h, self.pos.z + hw),
        }
    }

    pub fn eye(self) -> Vec3 {
        Vec3::new(self.pos.x, self.pos.y + self.eye_height(), self.pos.z)
    }

    pub fn look_dir(self) -> Vec3 {
        engine_core::yaw_pitch_dir(self.yaw, self.pitch)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MoveInput {
    pub forward: f32,
    pub strafe: f32,
    pub jump: bool,
    pub sneak: bool,
    pub sprint: bool,
    pub yaw: f32,
    pub pitch: f32,
}

pub fn tick_body(body: &mut Body, input: MoveInput, world: &impl VoxelSolid) {
    body.yaw = input.yaw;
    body.pitch = input.pitch.clamp(-1.535, 1.535);
    body.sneaking = input.sneak;
    body.sprinting = input.sprint && !input.sneak && input.forward > 0.0;

    let mut accel = if body.on_ground { GROUND_ACCEL } else { AIR_ACCEL };
    if body.sprinting && body.on_ground {
        accel *= SPRINT_MUL;
    }
    if body.sneaking {
        accel *= SNEAK_MUL;
    }

    let (sin, cos) = (body.yaw.sin(), body.yaw.cos());
    let mut wish_x = input.strafe * cos - input.forward * sin;
    let mut wish_z = input.forward * cos + input.strafe * sin;
    let mag = (wish_x * wish_x + wish_z * wish_z).sqrt();
    if mag > 1.0 {
        wish_x /= mag;
        wish_z /= mag;
    }
    body.vel.x += wish_x * accel;
    body.vel.z += wish_z * accel;

    if input.jump && body.on_ground {
        body.vel.y = JUMP;
        if body.sprinting {
            body.vel.x -= sin * 0.2;
            body.vel.z += cos * 0.2;
        }
        body.on_ground = false;
    }

    if body.sneaking && body.on_ground {
        sneak_edge(body, world);
    }

    let mut box_ = body.aabb();
    let mut vel = body.vel;
    let before_y = box_.min.y;
    let hit = move_colliding(&mut box_, &mut vel, world, STEP);
    body.pos.x = (box_.min.x + box_.max.x) * 0.5;
    body.pos.y = box_.min.y;
    body.pos.z = (box_.min.z + box_.max.z) * 0.5;
    body.vel = vel;

    if hit.y && before_y > body.pos.y {
        body.on_ground = true;
    } else if !hit.y {
        body.on_ground = false;
    }
    if body.vel.y < 0.0 && !body.on_ground {
        body.fall_distance += before_y - body.pos.y;
    }
    if body.on_ground {
        // fall_distance consumed by caller via take_landed_fall
    }

    body.vel.y -= GRAVITY;
    body.vel.y *= AIR_DRAG;
    let mut drag = HORIZ_DRAG;
    if body.on_ground {
        drag *= SLIP;
    }
    body.vel.x *= drag;
    body.vel.z *= drag;
}

fn sneak_edge(body: &mut Body, world: &impl VoxelSolid) {
    let probe = body.aabb().translate(Vec3::new(body.vel.x, -0.05, body.vel.z));
    let feet_y = (body.pos.y - 0.001).floor() as i32;
    let x = body.pos.x.floor() as i32;
    let z = body.pos.z.floor() as i32;
    let dest_x = probe.min.x.floor() as i32;
    let dest_z = probe.min.z.floor() as i32;
    let dest_x2 = (probe.max.x - 0.001).floor() as i32;
    let dest_z2 = (probe.max.z - 0.001).floor() as i32;
    let mut supported = false;
    for ix in dest_x..=dest_x2 {
        for iz in dest_z..=dest_z2 {
            if world.solid(ix, feet_y, iz) {
                supported = true;
            }
        }
    }
    if !supported && world.solid(x, feet_y, z) {
        body.vel.x = 0.0;
        body.vel.z = 0.0;
    }
}

pub fn take_landed_fall(body: &mut Body) -> f32 {
    if !body.on_ground {
        return 0.0;
    }
    let d = body.fall_distance;
    body.fall_distance = 0.0;
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VoxelSolid;

    struct Flat;

    impl VoxelSolid for Flat {
        fn solid(&self, _x: i32, y: i32, _z: i32) -> bool {
            y < 0
        }
    }

    #[test]
    fn gravity_lands_on_floor() {
        let mut b = Body::at(Vec3::new(0.5, 5.0, 0.5));
        for _ in 0..40 {
            tick_body(&mut b, MoveInput::default(), &Flat);
        }
        assert!(b.on_ground);
        assert!(b.pos.y > -0.01 && b.pos.y < 0.05);
    }

    #[test]
    fn jump_leaves_ground() {
        let mut b = Body::at(Vec3::new(0.5, 0.0, 0.5));
        b.on_ground = true;
        let mut input = MoveInput::default();
        input.jump = true;
        tick_body(&mut b, input, &Flat);
        assert!(!b.on_ground);
        assert!(b.vel.y > 0.2);
    }
}
