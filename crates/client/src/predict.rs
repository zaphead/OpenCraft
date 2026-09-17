use engine_core::Vec3;
use engine_phys::{tick_body, Body, MoveInput};
use world::World;

pub struct Predict {
    pub body: Body,
    pub prev: Vec3,
    pub health: u8,
    pub local_tick: u32,
}

impl Predict {
    pub fn new(pos: Vec3) -> Self {
        Self {
            body: Body::at(pos),
            prev: pos,
            health: 20,
            local_tick: 0,
        }
    }

    pub fn step(&mut self, input: MoveInput, world: &World) {
        self.prev = self.body.pos;
        self.local_tick = self.local_tick.wrapping_add(1);
        tick_body(&mut self.body, input, world);
    }

    pub fn render_pos(&self, partial: f32) -> Vec3 {
        self.prev.lerp(self.body.pos, partial)
    }

    pub fn correct(&mut self, pos: Vec3, vel: Vec3, on_ground: bool, health: u8, fall: f32) {
        self.health = health;
        self.body.fall_distance = fall;
        let err = self.body.pos.distance(pos);
        if err > 2.0 {
            self.body.pos = pos;
            self.prev = pos;
            self.body.vel = vel;
            self.body.on_ground = on_ground;
        } else if err > 0.05 {
            self.body.pos = self.body.pos.lerp(pos, 0.3);
            self.body.vel = vel;
            self.body.on_ground = on_ground;
        }
    }
}
