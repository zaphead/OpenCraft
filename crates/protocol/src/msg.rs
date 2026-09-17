use engine_core::{BlockPos, ChunkPos, EntityId, Vec3};

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub enum ClientPlay {
    TickInput {
        tick: u32,
        yaw: f32,
        pitch: f32,
        forward: f32,
        strafe: f32,
        jump: bool,
        sneak: bool,
        sprint: bool,
    },
    StartDig {
        pos: BlockPos,
    },
    CancelDig,
    Place {
        against: BlockPos,
        face: u8,
        hotbar: u8,
    },
    UseBlock {
        pos: BlockPos,
    },
    SelectHotbar {
        slot: u8,
    },
    ClickSlot {
        slot: u16,
        button: u8,
        shift: bool,
    },
    CloseWindow,
    OpenInventory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenKind {
    Inventory,
    CraftingTable,
    Chest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleKind {
    Break,
    Place,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundKind {
    Step,
    Break,
    Place,
    Hurt,
    Pickup,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InventorySnap {
    pub slots: Vec<Option<(u16, u8)>>,
    pub cursor: Option<(u16, u8)>,
    pub selected: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServerPlay {
    JoinGame {
        entity: EntityId,
        seed: i64,
        tick: u32,
        spawn: Vec3,
        pos: Vec3,
        health: u8,
        inventory: InventorySnap,
    },
    KeepTick {
        tick: u32,
    },
    Chunk {
        pos: ChunkPos,
        sections: Vec<(u8, Vec<u16>, Vec<u16>)>,
    },
    UnloadChunk {
        pos: ChunkPos,
    },
    BlockUpdate {
        pos: BlockPos,
        id: u16,
    },
    PlayerCorrection {
        tick: u32,
        pos: Vec3,
        vel: Vec3,
        on_ground: bool,
        health: u8,
        fall_distance: f32,
    },
    EntitySpawn {
        id: EntityId,
        kind: u8,
        pos: Vec3,
        item: Option<(u16, u8)>,
    },
    EntityPos {
        id: EntityId,
        pos: Vec3,
        yaw: f32,
        pitch: f32,
    },
    EntityDespawn {
        id: EntityId,
    },
    Inventory(InventorySnap),
    OpenWindow {
        kind: OpenKind,
        pos: Option<BlockPos>,
    },
    CloseWindow,
    Particles {
        kind: ParticleKind,
        pos: Vec3,
        block: u16,
    },
    Sound {
        kind: SoundKind,
        pos: Vec3,
        block: u16,
    },
    Hurt {
        health: u8,
    },
    Respawned {
        pos: Vec3,
        health: u8,
        inventory: InventorySnap,
    },
}
