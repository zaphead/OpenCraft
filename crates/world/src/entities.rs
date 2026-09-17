use engine_core::{EntityId, Vec3};
use engine_phys::Aabb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Item,
}

#[derive(Clone, Debug)]
pub struct Entities {
    gens: Vec<u32>,
    free: Vec<u32>,
    pub kind: Vec<EntityKind>,
    pub pos: Vec<Vec3>,
    pub vel: Vec<Vec3>,
    pub yaw: Vec<f32>,
    pub pitch: Vec<f32>,
    pub item: Vec<Option<(u16, u8)>>,
    pub age: Vec<u32>,
    pub pickup_delay: Vec<u32>,
    alive: Vec<bool>,
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}

impl Entities {
    pub fn new() -> Self {
        Self {
            gens: Vec::new(),
            free: Vec::new(),
            kind: Vec::new(),
            pos: Vec::new(),
            vel: Vec::new(),
            yaw: Vec::new(),
            pitch: Vec::new(),
            item: Vec::new(),
            age: Vec::new(),
            pickup_delay: Vec::new(),
            alive: Vec::new(),
        }
    }

    pub fn spawn(&mut self, kind: EntityKind, pos: Vec3) -> EntityId {
        let item = None;
        if let Some(index) = self.free.pop() {
            let gen = self.gens[index as usize].wrapping_add(1).max(1);
            self.gens[index as usize] = gen;
            let i = index as usize;
            self.kind[i] = kind;
            self.pos[i] = pos;
            self.vel[i] = Vec3::ZERO;
            self.yaw[i] = 0.0;
            self.pitch[i] = 0.0;
            self.item[i] = item;
            self.age[i] = 0;
            self.pickup_delay[i] = 0;
            self.alive[i] = true;
            EntityId::new(index, gen)
        } else {
            let index = self.gens.len() as u32;
            self.gens.push(1);
            self.kind.push(kind);
            self.pos.push(pos);
            self.vel.push(Vec3::ZERO);
            self.yaw.push(0.0);
            self.pitch.push(0.0);
            self.item.push(item);
            self.age.push(0);
            self.pickup_delay.push(0);
            self.alive.push(true);
            EntityId::new(index, 1)
        }
    }

    pub fn despawn(&mut self, id: EntityId) {
        let i = id.index as usize;
        if i >= self.gens.len() || self.gens[i] != id.gen || !self.alive[i] {
            return;
        }
        self.alive[i] = false;
        self.free.push(id.index);
    }

    pub fn get(&self, id: EntityId) -> Option<usize> {
        let i = id.index as usize;
        if i < self.gens.len() && self.gens[i] == id.gen && self.alive[i] {
            Some(i)
        } else {
            None
        }
    }

    pub fn id_at(&self, index: usize) -> EntityId {
        EntityId::new(index as u32, self.gens[index])
    }

    pub fn iter_alive(&self) -> impl Iterator<Item = (EntityId, usize)> + '_ {
        self.alive.iter().enumerate().filter_map(|(i, a)| {
            if *a {
                Some((self.id_at(i), i))
            } else {
                None
            }
        })
    }

    pub fn aabb_at(&self, i: usize) -> Aabb {
        match self.kind[i] {
            EntityKind::Player => {
                let p = self.pos[i];
                Aabb {
                    min: Vec3::new(p.x - 0.3, p.y, p.z - 0.3),
                    max: Vec3::new(p.x + 0.3, p.y + 1.8, p.z + 0.3),
                }
            }
            EntityKind::Item => {
                let p = self.pos[i];
                Aabb {
                    min: p,
                    max: p + Vec3::splat(0.25),
                }
            }
        }
    }
}
