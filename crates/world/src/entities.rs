use engine_core::{EntityId, Vec3};
use engine_phys::Aabb;

/// Mobs are more rows of this enum. Discriminant is the protocol kind byte.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityKind {
    Player = 0,
    Item = 1,
    Aurochs = 2,
    Boar = 3,
    Goat = 4,
    Rabbit = 5,
    Deer = 6,
    Fox = 7,
    Songbird = 8,
    Crow = 9,
    Fish = 10,
    Frog = 11,
    Glowbeetle = 12,
    Bee = 13,
    Horse = 14,
    Griffin = 15,
    Goblin = 16,
    Husk = 17,
    Wraith = 18,
    Wight = 19,
    Imp = 20,
    Hopper = 21,
    Mimic = 22,
    Sentinel = 23,
    Treant = 24,
    Skywhale = 25,
    Wyrm = 26,
    Sage = 27,
    Parrot = 28,
    Projectile = 29,
}

impl EntityKind {
    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0 => Self::Player,
            1 => Self::Item,
            2 => Self::Aurochs,
            3 => Self::Boar,
            4 => Self::Goat,
            5 => Self::Rabbit,
            6 => Self::Deer,
            7 => Self::Fox,
            8 => Self::Songbird,
            9 => Self::Crow,
            10 => Self::Fish,
            11 => Self::Frog,
            12 => Self::Glowbeetle,
            13 => Self::Bee,
            14 => Self::Horse,
            15 => Self::Griffin,
            16 => Self::Goblin,
            17 => Self::Husk,
            18 => Self::Wraith,
            19 => Self::Wight,
            20 => Self::Imp,
            21 => Self::Hopper,
            22 => Self::Mimic,
            23 => Self::Sentinel,
            24 => Self::Treant,
            25 => Self::Skywhale,
            26 => Self::Wyrm,
            27 => Self::Sage,
            28 => Self::Parrot,
            29 => Self::Projectile,
            _ => return None,
        })
    }

    pub fn code(self) -> u8 {
        self as u8
    }

    /// Half-width and height of the collision box.
    pub fn box_size(self) -> (f32, f32) {
        match self {
            Self::Player => (0.3, 1.8),
            Self::Item => (0.125, 0.25),
            Self::Aurochs => (0.7, 1.4),
            Self::Boar => (0.45, 0.8),
            Self::Goat => (0.35, 0.9),
            Self::Rabbit => (0.2, 0.4),
            Self::Deer => (0.4, 1.4),
            Self::Fox => (0.3, 0.5),
            Self::Songbird | Self::Crow | Self::Parrot => (0.15, 0.3),
            Self::Fish => (0.2, 0.2),
            Self::Frog => (0.2, 0.3),
            Self::Glowbeetle | Self::Bee => (0.12, 0.2),
            Self::Horse => (0.6, 1.5),
            Self::Griffin => (0.6, 1.1),
            Self::Goblin => (0.25, 1.2),
            Self::Husk | Self::Wight => (0.3, 1.8),
            Self::Wraith => (0.3, 1.9),
            Self::Imp => (0.2, 0.6),
            Self::Hopper => (0.3, 0.6),
            Self::Mimic => (0.45, 0.9),
            Self::Sentinel => (0.4, 2.0),
            Self::Treant => (0.7, 3.0),
            Self::Skywhale => (2.0, 1.5),
            Self::Wyrm => (1.0, 1.2),
            Self::Sage => (0.3, 1.8),
            Self::Projectile => (0.1, 0.2),
        }
    }
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
    pub hp: Vec<u8>,
    pub state: Vec<u8>,
    pub tame: Vec<u8>,
    pub stamina: Vec<u8>,
    pub variant: Vec<u8>,
    pub timer: Vec<u16>,
    pub link: Vec<u32>,
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
            hp: Vec::new(),
            state: Vec::new(),
            tame: Vec::new(),
            stamina: Vec::new(),
            variant: Vec::new(),
            timer: Vec::new(),
            link: Vec::new(),
            alive: Vec::new(),
        }
    }

    pub fn spawn(&mut self, kind: EntityKind, pos: Vec3) -> EntityId {
        if let Some(index) = self.free.pop() {
            let gen = self.gens[index as usize].wrapping_add(1).max(1);
            self.gens[index as usize] = gen;
            let i = index as usize;
            self.write_new(i, kind, pos);
            EntityId::new(index, gen)
        } else {
            let index = self.gens.len() as u32;
            self.gens.push(1);
            self.kind.push(kind);
            self.pos.push(pos);
            self.vel.push(Vec3::ZERO);
            self.yaw.push(0.0);
            self.pitch.push(0.0);
            self.item.push(None);
            self.age.push(0);
            self.pickup_delay.push(0);
            self.hp.push(0);
            self.state.push(0);
            self.tame.push(0);
            self.stamina.push(0);
            self.variant.push(0);
            self.timer.push(0);
            self.link.push(0);
            self.alive.push(true);
            EntityId::new(index, 1)
        }
    }

    fn write_new(&mut self, i: usize, kind: EntityKind, pos: Vec3) {
        self.kind[i] = kind;
        self.pos[i] = pos;
        self.vel[i] = Vec3::ZERO;
        self.yaw[i] = 0.0;
        self.pitch[i] = 0.0;
        self.item[i] = None;
        self.age[i] = 0;
        self.pickup_delay[i] = 0;
        self.hp[i] = 0;
        self.state[i] = 0;
        self.tame[i] = 0;
        self.stamina[i] = 0;
        self.variant[i] = 0;
        self.timer[i] = 0;
        self.link[i] = 0;
        self.alive[i] = true;
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
        let (hw, h) = self.kind[i].box_size();
        let p = self.pos[i];
        let h = if self.state[i] == 3 { h * 0.5 } else { h };
        Aabb {
            min: Vec3::new(p.x - hw, p.y, p.z - hw),
            max: Vec3::new(p.x + hw, p.y + h, p.z + hw),
        }
    }
}
