use engine_core::{BlockPos, EntityId};
use engine_phys::{Body, MoveInput};
use game::{slot_at, window_len, Stack, Window, MAX_HEALTH};

pub struct Player {
    pub entity: EntityId,
    pub body: Body,
    pub health: u8,
    pub hotbar: [Stack; 9],
    pub main: [Stack; 27],
    pub selected: u8,
    pub window: Option<Window>,
    pub chest_pos: Option<BlockPos>,
    pub digging: Option<(BlockPos, u32)>,
    pub last_tick: u32,
    pub known: engine_core::FxHashSet<engine_core::ChunkPos>,
    pub spawn: engine_core::Vec3,
    pub last_on_ground: bool,
    pub step_accum: f32,
    pub input: MoveInput,
    pub mount: Option<EntityId>,
    pub rooted: u16,
    pub fire: u16,
    pub glow: u16,
    pub swift: u16,
    pub adv: u16,
    pub hurt_cd: u8,
}

impl Player {
    pub fn new(entity: EntityId, spawn: engine_core::Vec3) -> Self {
        Self {
            entity,
            body: Body::at(spawn),
            health: MAX_HEALTH,
            hotbar: [None; 9],
            main: [None; 27],
            selected: 0,
            window: None,
            chest_pos: None,
            digging: None,
            last_tick: 0,
            known: engine_core::FxHashSet::default(),
            spawn,
            last_on_ground: true,
            step_accum: 0.0,
            input: MoveInput::default(),
            mount: None,
            rooted: 0,
            fire: 0,
            glow: 0,
            swift: 0,
            adv: 0,
            hurt_cd: 0,
        }
    }

    pub fn held(&self) -> Option<u16> {
        self.hotbar[self.selected as usize].map(|(id, _)| id)
    }
}

pub fn snap_inv(p: &Player) -> protocol::InventorySnap {
    let mut slots = Vec::new();
    if let Some(w) = &p.window {
        for i in 0..window_len(w.kind) {
            slots.push(slot_at(w, i as u16));
        }
        protocol::InventorySnap {
            slots,
            cursor: w.cursor,
            selected: p.selected,
        }
    } else {
        slots.extend_from_slice(&p.hotbar);
        slots.extend_from_slice(&p.main);
        protocol::InventorySnap {
            slots,
            cursor: None,
            selected: p.selected,
        }
    }
}
