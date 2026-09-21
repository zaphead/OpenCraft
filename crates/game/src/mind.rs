//! What a mob wants this tick. Feature files fill `think`; the sim applies it.

use engine_core::Vec3;

pub const CALM: u8 = 0;
pub const FLEE: u8 = 1;
pub const AGGRO: u8 = 2;
pub const BABY: u8 = 3;
pub const SHUT: u8 = 4;
pub const ASLEEP: u8 = 5;

#[derive(Clone, Copy)]
pub struct Sight {
    pub tick: u32,
    pub night: bool,
    pub blood: bool,
    pub pos: Vec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub hp: u8,
    pub state: u8,
    pub tame: u8,
    pub stamina: u8,
    pub variant: u8,
    pub timer: u16,
    pub player: Vec3,
    pub dist: f32,
    pub herd: bool,
    pub item: Option<Vec3>,
    pub water: bool,
    pub lava: bool,
    pub grass: bool,
    pub flower: bool,
    pub ridden: bool,
}

#[derive(Clone, Copy)]
pub struct Will {
    pub x: f32,
    pub z: f32,
    pub yaw: f32,
    pub jump: f32,
    pub state: u8,
    pub tame: u8,
    pub stamina: u8,
    pub timer: u16,
    pub hit: u8,
    pub call: bool,
    pub flower: bool,
    pub boom: bool,
    pub root: u16,
    pub take_item: bool,
    pub spawn_goblin: bool,
    pub fly: bool,
}

impl Will {
    pub fn hold(s: &Sight) -> Self {
        Self {
            x: 0.0,
            z: 0.0,
            yaw: s.yaw,
            jump: 0.0,
            state: s.state,
            tame: s.tame,
            stamina: s.stamina,
            timer: s.timer,
            hit: 0,
            call: false,
            flower: false,
            boom: false,
            root: 0,
            take_item: false,
            spawn_goblin: false,
            fly: false,
        }
    }
}

pub fn toward(s: &Sight, target: Vec3, speed: f32) -> (f32, f32, f32) {
    let dx = target.x - s.pos.x;
    let dz = target.z - s.pos.z;
    let mag = (dx * dx + dz * dz).sqrt().max(0.001);
    let yaw = dz.atan2(-dx);
    (dx / mag * speed, dz / mag * speed, yaw)
}

pub fn away(s: &Sight, speed: f32) -> (f32, f32, f32) {
    let (x, z, yaw) = toward(s, s.player, speed);
    (-x, -z, yaw + std::f32::consts::PI)
}

pub fn wander(s: &Sight, speed: f32) -> (f32, f32, f32) {
    let phase = (s.tick / 40) as f32 + s.pos.x;
    let yaw = phase.sin() * 3.0 + s.pos.z * 0.01;
    (-yaw.sin() * speed, yaw.cos() * speed, yaw)
}
