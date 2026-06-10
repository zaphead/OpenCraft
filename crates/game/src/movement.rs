use glam::{Vec2, Vec3};

use crate::axes::{horizontal_forward, horizontal_right, UP};
use crate::play_mode::PlayMode;

pub const BASE_WALK_SPEED: f32 = 5.1;
pub const GROUND_ACCEL: f32 = 90.0;
pub const MOUSE_SENSITIVITY: f32 = 0.0012;
pub const SURVIVAL_SPRINT_MULT: f32 = 1.5;
pub const SPECTATOR_SPEED_SCALE: f32 = 2.0;
pub const SPECTATOR_SPRINT_MULT: f32 = 3.0;
/// Lower = more ice slide when input is released.
pub const SPECTATOR_DRAG: f32 = 1.6;
/// Max steering speed added along input while airborne (does not cut carried ground speed).
pub const AIR_CONTROL_SPEED: f32 = 1.75;
pub const AIR_ACCEL: f32 = 35.0;
/// Passive bleed on lateral speed while airborne — very light so sprint carry coasts through jumps.
pub const AIR_DRAG: f32 = 0.165;

#[derive(Debug, Clone, Copy)]
pub struct LocomotionConfig {
    pub speed_scale: f32,
    pub sprint_multiplier: f32,
    pub fly: bool,
}

impl LocomotionConfig {
    pub fn for_mode(mode: PlayMode) -> Self {
        match mode {
            PlayMode::Survival => Self {
                speed_scale: 1.0,
                sprint_multiplier: SURVIVAL_SPRINT_MULT,
                fly: false,
            },
            PlayMode::Spectator => Self {
                speed_scale: SPECTATOR_SPEED_SCALE,
                sprint_multiplier: SPECTATOR_SPRINT_MULT,
                fly: true,
            },
        }
    }
}

pub fn max_speed(config: LocomotionConfig, sprint: bool) -> f32 {
    BASE_WALK_SPEED * config.speed_scale * if sprint { config.sprint_multiplier } else { 1.0 }
}

pub fn wish_direction_horizontal(yaw: f32, move_axis: Vec2) -> Vec3 {
    let forward = horizontal_forward(yaw);
    let right = horizontal_right(yaw);
    (forward * move_axis.y + right * move_axis.x).normalize_or_zero()
}

pub fn wish_direction_fly(yaw: f32, move_axis: Vec2, vertical_axis: f32) -> Vec3 {
    let horizontal = wish_direction_horizontal(yaw, move_axis);
    let vertical = UP * vertical_axis;
    (horizontal + vertical).normalize_or_zero()
}

/// Shared acceleration toward a target velocity (survival ground + spectator thrust).
pub fn accelerate_toward(current: Vec3, target: Vec3, delta: f32) -> Vec3 {
    accelerate_toward_step(current, target, GROUND_ACCEL * delta)
}

fn accelerate_toward_step(current: Vec3, target: Vec3, max_step: f32) -> Vec3 {
    let delta_v = target - current;
    let dist = delta_v.length();
    if dist > max_step && dist > 0.0 {
        current + delta_v / dist * max_step
    } else {
        target
    }
}

pub fn accelerate_horizontal(current: Vec2, target: Vec2, max_step: f32) -> Vec2 {
    let delta_v = target - current;
    let dist = delta_v.length();
    if dist > max_step && dist > 0.0 {
        current + delta_v / dist * max_step
    } else {
        target
    }
}

pub fn apply_horizontal_drag(velocity: Vec2, drag: f32, delta: f32) -> Vec2 {
    velocity * (-drag * delta).exp()
}

/// Spectator coast decay when movement input is released.
pub fn apply_ice_drag(velocity: Vec3, delta: f32) -> Vec3 {
    velocity * (-SPECTATOR_DRAG * delta).exp()
}
