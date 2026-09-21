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

/// The one body, mounted. Sneak dismounts before this tick; glide does not hover.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Ride {
    #[default]
    Foot,
    Horse,
    Griffin,
}

/// What the feet are standing in. Lava and springs are still just blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Fluid {
    #[default]
    None,
    Water,
    Lava,
    Spring,
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
    pub ride: Ride,
    pub fluid: Fluid,
    pub rooted: bool,
    pub swift: bool,
}

pub fn tick_body(body: &mut Body, input: MoveInput, world: &impl VoxelSolid) {
    body.yaw = input.yaw;
    body.pitch = input.pitch.clamp(-1.535, 1.535);
    body.sneaking = input.sneak && input.ride == Ride::Foot;
    body.sprinting = input.sprint && !input.sneak && input.forward > 0.0;

    // A treant holds the body still. Fall distance stays so a later landing
    // still hurts; the root itself is not a landing.
    if input.rooted {
        body.vel = Vec3::ZERO;
        body.sprinting = false;
        return;
    }

    let mut accel = if body.on_ground { GROUND_ACCEL } else { AIR_ACCEL };
    if input.ride == Ride::Horse {
        accel *= 1.55;
    }
    if input.swift {
        accel *= 1.35;
    }
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
    let vy_before = vel.y;
    let before_y = box_.min.y;
    let hit = move_colliding(&mut box_, &mut vel, world, STEP);
    body.pos.x = (box_.min.x + box_.max.x) * 0.5;
    body.pos.y = box_.min.y;
    body.pos.z = (box_.min.z + box_.max.z) * 0.5;
    body.vel = vel;

    // Grounded means contact below while falling or resting, not measured
    // downward travel: a resting body clips zero distance every tick, and the
    // old displacement check latched it airborne forever (no friction, no
    // sneak grip, no jump refresh on stairs — the whole "slidey" feel).
    if hit.y && vy_before <= 0.0 {
        body.on_ground = true;
    } else if !hit.y {
        body.on_ground = if vy_before <= 0.0 {
            supported_below(&box_, world)
        } else {
            false
        };
    } else {
        // Rising into a ceiling: contact, but airborne. Never stale-grounded.
        body.on_ground = false;
    }
    if body.vel.y < 0.0 && !body.on_ground {
        body.fall_distance += before_y - body.pos.y;
    }
    if body.on_ground {
        // fall_distance consumed by caller via take_landed_fall
    }

    let mut gravity = GRAVITY;
    let mut y_drag = AIR_DRAG;
    match input.fluid {
        Fluid::Water | Fluid::Spring => {
            gravity = 0.02;
            y_drag = 0.8;
            if input.jump {
                body.vel.y += 0.06;
            }
        }
        Fluid::Lava => {
            gravity = 0.03;
            y_drag = 0.8;
            if input.jump {
                body.vel.y += 0.04;
            }
        }
        Fluid::None => {}
    }
    // Griffin: forward and down. Never a hover — vertical speed stays a sink
    // unless this tick's jump just left the ground.
    if input.ride == Ride::Griffin && !(input.jump && body.on_ground) {
        let (sin, cos) = (body.yaw.sin(), body.yaw.cos());
        let glide = input.forward.max(0.0);
        body.vel.x += -sin * glide * 0.06;
        body.vel.z += cos * glide * 0.06;
        if body.vel.y > -0.08 {
            body.vel.y = -0.08;
        }
        gravity = 0.01;
    }
    body.vel.y -= gravity;
    body.vel.y *= y_drag;
    let mut drag = HORIZ_DRAG;
    if input.fluid != Fluid::None {
        drag *= 0.85;
    }
    if body.on_ground {
        drag *= SLIP;
    }
    body.vel.x *= drag;
    body.vel.z *= drag;
}

/// Solid ground under any corner of this box? Only consulted when the Y
/// sweep clipped nothing, to tell resting contact from thin air.
fn supported_below(box_: &Aabb, world: &impl VoxelSolid) -> bool {
    let feet_y = (box_.min.y - 1e-3).floor() as i32;
    let min_x = box_.min.x.floor() as i32;
    let min_z = box_.min.z.floor() as i32;
    let max_x = (box_.max.x - 1e-4).floor() as i32;
    let max_z = (box_.max.z - 1e-4).floor() as i32;
    for x in min_x..=max_x {
        for z in min_z..=max_z {
            if world.solid(x, feet_y, z) {
                return true;
            }
        }
    }
    false
}

fn sneak_edge(body: &mut Body, world: &impl VoxelSolid) {
    // Java-like lip grip: stepping the body center past support while the
    // center still stands on ground cancels the move. Center-based, because a
    // footprint check stays "supported" on the trailing corner a full block
    // past the lip, which is exactly how bodies used to stroll into the void.
    let feet_y = (body.pos.y - 0.001).floor() as i32;
    let cur_x = body.pos.x.floor() as i32;
    let cur_z = body.pos.z.floor() as i32;
    let dest_x = (body.pos.x + body.vel.x).floor() as i32;
    let dest_z = (body.pos.z + body.vel.z).floor() as i32;
    if world.solid(cur_x, feet_y, cur_z) && !world.solid(dest_x, feet_y, dest_z) {
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

    struct Blocks {
        floor: i32,
        lid_y: Option<i32>,
        steps: bool,
    }

    impl VoxelSolid for Blocks {
        fn solid(&self, x: i32, y: i32, z: i32) -> bool {
            if self.steps {
                // Staircase rising one block per step along +x from x=0.
                let top = self.floor + (x.max(0).min(8)) as i32;
                if z == 0 && x >= -4 && x <= 9 && y < top && y >= top - 6 {
                    return true;
                }
                return false;
            }
            if y < self.floor {
                return true;
            }
            if self.lid_y.is_some_and(|l| y == l && x >= -2 && x <= 2 && z >= -2 && z <= 2) {
                return true;
            }
            false
        }
    }

    fn free_of_rock(b: &Body, w: &Blocks) {
        let a = b.aabb();
        let shrink = 1e-3;
        for x in (a.min.x.floor() as i32)..=((a.max.x - shrink).floor() as i32) {
            for y in (a.min.y.floor() as i32)..=((a.max.y - shrink).floor() as i32) {
                for z in (a.min.z.floor() as i32)..=((a.max.z - shrink).floor() as i32) {
                    if !w.solid(x, y, z) {
                        continue;
                    }
                    let block = Aabb {
                        min: Vec3::new(x as f32, y as f32, z as f32),
                        max: Vec3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0),
                    };
                    assert!(!a.intersects(block), "wedged into ({x}, {y}, {z})");
                }
            }
        }
    }

    #[test]
    fn sneak_toggle_is_exact() {
        // Sneak is the key, nothing latches: alternating every tick tracks
        // input exactly and the body never touches rock.
        let w = Blocks { floor: 0, lid_y: None, steps: false };
        let mut b = Body::at(Vec3::new(0.5, 0.0, 0.5));
        b.on_ground = true;
        for t in 0..40 {
            let mut input = MoveInput::default();
            input.sneak = t % 2 == 0;
            tick_body(&mut b, input, &w);
            assert_eq!(b.sneaking, input.sneak, "sneak stuck at tick {t}");
            free_of_rock(&b, &w);
        }
    }

    #[test]
    fn unsneak_in_open_stands_up() {
        let w = Blocks { floor: 0, lid_y: None, steps: false };
        let mut b = Body::at(Vec3::new(0.5, 0.0, 0.5));
        b.on_ground = true;
        let mut sneak = MoveInput::default();
        sneak.sneak = true;
        tick_body(&mut b, sneak, &w);
        assert!(b.sneaking);
        tick_body(&mut b, MoveInput::default(), &w);
        assert!(!b.sneaking);
        assert_eq!(b.height(), PLAYER_HEIGHT);
    }

    #[test]
    fn ground_friction_stops_the_slide() {
        // Sprint cut to idle: Java-like drag must kill the glide, no ice rink.
        let w = Blocks { floor: 0, lid_y: None, steps: false };
        let mut b = Body::at(Vec3::new(0.5, 0.0, 0.5));
        b.on_ground = true;
        b.vel.x = 0.5;
        for _ in 0..20 {
            tick_body(&mut b, MoveInput::default(), &w);
        }
        assert!(b.vel.x.abs() < 0.005, "still skating at {}", b.vel.x);
    }

    #[test]
    fn staircase_jump_never_wedges() {
        let w = Blocks { floor: 0, lid_y: None, steps: true };
        let mut b = Body::at(Vec3::new(-3.5, 0.0, 0.5));
        b.on_ground = true;
        // Sprint-jump up eight full blocks: step assist plus jumps must never
        // leave the body intersecting terrain.
        for t in 0..400 {
            let mut input = MoveInput::default();
            input.forward = 1.0;
            input.sprint = true;
            input.yaw = -std::f32::consts::FRAC_PI_2;
            if b.on_ground && t % 12 == 0 {
                input.jump = true;
            }
            tick_body(&mut b, input, &w);
            free_of_rock(&b, &w);
        }
        assert!(b.pos.x > 4.0, "stuck at the stairs at x={}", b.pos.x);
    }

    #[test]
    fn sneak_holds_the_edge() {
        // Island floor top at y=0 for x in -4..=2, void past x=2.
        struct Island;
        impl VoxelSolid for Island {
            fn solid(&self, x: i32, y: i32, _z: i32) -> bool {
                y < 0 && x <= 2
            }
        }
        let w = Island;
        let mut b = Body::at(Vec3::new(1.5, 0.0, 0.5));
        b.on_ground = true;
        // Sneaking toward the void must hold position at the lip.
        for _ in 0..60 {
            let mut input = MoveInput::default();
            input.sneak = true;
            input.forward = 1.0;
            input.yaw = -std::f32::consts::FRAC_PI_2;
            tick_body(&mut b, input, &w);
        }
        assert!(b.pos.x < 3.2, "sneak strolled off at x={}", b.pos.x);
        assert!(b.pos.y > -0.5, "sneak fell off");
    }
}
