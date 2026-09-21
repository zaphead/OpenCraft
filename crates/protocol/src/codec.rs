use engine_core::{BlockPos, ChunkPos, EntityId, Vec3};
use thiserror::Error;

use crate::msg::{ClientPlay, InventorySnap, OpenKind, ParticleKind, ServerPlay, SoundKind};

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("truncated packet")]
    Truncated,
    #[error("unknown tag {0}")]
    Tag(u8),
}

pub fn encode_packet(msg: &ServerPlay) -> Vec<u8> {
    let mut w = Vec::new();
    encode_server(&mut w, msg);
    w
}

pub fn decode_packet(bytes: &[u8]) -> Result<ServerPlay, CodecError> {
    let mut r = bytes;
    decode_server(&mut r)
}

pub fn encode_client(msg: &ClientPlay) -> Vec<u8> {
    let mut w = Vec::new();
    write_client(&mut w, msg);
    w
}

pub fn decode_client(bytes: &[u8]) -> Result<ClientPlay, CodecError> {
    let mut r = bytes;
    read_client(&mut r)
}

fn u8w(w: &mut Vec<u8>, v: u8) {
    w.push(v);
}
fn u16w(w: &mut Vec<u8>, v: u16) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn u32w(w: &mut Vec<u8>, v: u32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn i32w(w: &mut Vec<u8>, v: i32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn i64w(w: &mut Vec<u8>, v: i64) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn f32w(w: &mut Vec<u8>, v: f32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn boolw(w: &mut Vec<u8>, v: bool) {
    w.push(if v { 1 } else { 0 });
}

fn u8r(r: &mut &[u8]) -> Result<u8, CodecError> {
    if r.is_empty() {
        return Err(CodecError::Truncated);
    }
    let v = r[0];
    *r = &r[1..];
    Ok(v)
}
fn take<const N: usize>(r: &mut &[u8]) -> Result<[u8; N], CodecError> {
    if r.len() < N {
        return Err(CodecError::Truncated);
    }
    let mut a = [0u8; N];
    a.copy_from_slice(&r[..N]);
    *r = &r[N..];
    Ok(a)
}
fn u16r(r: &mut &[u8]) -> Result<u16, CodecError> {
    Ok(u16::from_le_bytes(take(r)?))
}
fn u32r(r: &mut &[u8]) -> Result<u32, CodecError> {
    Ok(u32::from_le_bytes(take(r)?))
}
fn i32r(r: &mut &[u8]) -> Result<i32, CodecError> {
    Ok(i32::from_le_bytes(take(r)?))
}
fn i64r(r: &mut &[u8]) -> Result<i64, CodecError> {
    Ok(i64::from_le_bytes(take(r)?))
}
fn f32r(r: &mut &[u8]) -> Result<f32, CodecError> {
    Ok(f32::from_le_bytes(take(r)?))
}
fn boolr(r: &mut &[u8]) -> Result<bool, CodecError> {
    Ok(u8r(r)? != 0)
}

fn pos_w(w: &mut Vec<u8>, p: BlockPos) {
    i32w(w, p.x);
    i32w(w, p.y);
    i32w(w, p.z);
}
fn pos_r(r: &mut &[u8]) -> Result<BlockPos, CodecError> {
    Ok(BlockPos::new(i32r(r)?, i32r(r)?, i32r(r)?))
}
fn vec_w(w: &mut Vec<u8>, v: Vec3) {
    f32w(w, v.x);
    f32w(w, v.y);
    f32w(w, v.z);
}
fn vec_r(r: &mut &[u8]) -> Result<Vec3, CodecError> {
    Ok(Vec3::new(f32r(r)?, f32r(r)?, f32r(r)?))
}
fn eid_w(w: &mut Vec<u8>, e: EntityId) {
    u32w(w, e.index);
    u32w(w, e.gen);
}
fn eid_r(r: &mut &[u8]) -> Result<EntityId, CodecError> {
    Ok(EntityId::new(u32r(r)?, u32r(r)?))
}

fn stack_w(w: &mut Vec<u8>, s: Option<(u16, u8)>) {
    match s {
        None => u8w(w, 0),
        Some((id, n)) => {
            u8w(w, 1);
            u16w(w, id);
            u8w(w, n);
        }
    }
}
fn stack_r(r: &mut &[u8]) -> Result<Option<(u16, u8)>, CodecError> {
    if boolr(r)? {
        Ok(Some((u16r(r)?, u8r(r)?)))
    } else {
        Ok(None)
    }
}

fn inv_w(w: &mut Vec<u8>, inv: &InventorySnap) {
    u16w(w, inv.slots.len() as u16);
    for s in &inv.slots {
        stack_w(w, *s);
    }
    stack_w(w, inv.cursor);
    u8w(w, inv.selected);
}
fn inv_r(r: &mut &[u8]) -> Result<InventorySnap, CodecError> {
    let n = u16r(r)? as usize;
    let mut slots = Vec::with_capacity(n);
    for _ in 0..n {
        slots.push(stack_r(r)?);
    }
    Ok(InventorySnap {
        slots,
        cursor: stack_r(r)?,
        selected: u8r(r)?,
    })
}

fn write_client(w: &mut Vec<u8>, m: &ClientPlay) {
    match m {
        ClientPlay::TickInput {
            tick,
            yaw,
            pitch,
            forward,
            strafe,
            jump,
            sneak,
            sprint,
        } => {
            u8w(w, 1);
            u32w(w, *tick);
            f32w(w, *yaw);
            f32w(w, *pitch);
            f32w(w, *forward);
            f32w(w, *strafe);
            boolw(w, *jump);
            boolw(w, *sneak);
            boolw(w, *sprint);
        }
        ClientPlay::StartDig { pos } => {
            u8w(w, 2);
            pos_w(w, *pos);
        }
        ClientPlay::CancelDig => u8w(w, 3),
        ClientPlay::Place {
            against,
            face,
            hotbar,
        } => {
            u8w(w, 4);
            pos_w(w, *against);
            u8w(w, *face);
            u8w(w, *hotbar);
        }
        ClientPlay::UseBlock { pos } => {
            u8w(w, 5);
            pos_w(w, *pos);
        }
        ClientPlay::SelectHotbar { slot } => {
            u8w(w, 6);
            u8w(w, *slot);
        }
        ClientPlay::ClickSlot { slot, button, shift } => {
            u8w(w, 7);
            u16w(w, *slot);
            u8w(w, *button);
            boolw(w, *shift);
        }
        ClientPlay::CloseWindow => u8w(w, 8),
        ClientPlay::OpenInventory => u8w(w, 9),
        ClientPlay::PressButton { id } => {
            u8w(w, 10);
            u8w(w, *id);
        }
    }
}

fn read_client(r: &mut &[u8]) -> Result<ClientPlay, CodecError> {
    let tag = u8r(r)?;
    let m = match tag {
        1 => ClientPlay::TickInput {
            tick: u32r(r)?,
            yaw: f32r(r)?,
            pitch: f32r(r)?,
            forward: f32r(r)?,
            strafe: f32r(r)?,
            jump: boolr(r)?,
            sneak: boolr(r)?,
            sprint: boolr(r)?,
        },
        2 => ClientPlay::StartDig { pos: pos_r(r)? },
        3 => ClientPlay::CancelDig,
        4 => ClientPlay::Place {
            against: pos_r(r)?,
            face: u8r(r)?,
            hotbar: u8r(r)?,
        },
        5 => ClientPlay::UseBlock { pos: pos_r(r)? },
        6 => ClientPlay::SelectHotbar { slot: u8r(r)? },
        7 => ClientPlay::ClickSlot {
            slot: u16r(r)?,
            button: u8r(r)?,
            shift: boolr(r)?,
        },
        8 => ClientPlay::CloseWindow,
        9 => ClientPlay::OpenInventory,
        10 => ClientPlay::PressButton { id: u8r(r)? },
        t => return Err(CodecError::Tag(t)),
    };
    Ok(m)
}

fn encode_server(w: &mut Vec<u8>, m: &ServerPlay) {
    match m {
        ServerPlay::JoinGame {
            entity,
            seed,
            tick,
            spawn,
            pos,
            health,
            inventory,
        } => {
            u8w(w, 1);
            eid_w(w, *entity);
            i64w(w, *seed);
            u32w(w, *tick);
            vec_w(w, *spawn);
            vec_w(w, *pos);
            u8w(w, *health);
            inv_w(w, inventory);
        }
        ServerPlay::KeepTick { tick } => {
            u8w(w, 2);
            u32w(w, *tick);
        }
        ServerPlay::Chunk { pos, sections } => {
            u8w(w, 3);
            i32w(w, pos.x);
            i32w(w, pos.z);
            u8w(w, sections.len() as u8);
            for (sy, pal, idx) in sections {
                u8w(w, *sy);
                u16w(w, pal.len() as u16);
                for p in pal {
                    u16w(w, *p);
                }
                u32w(w, idx.len() as u32);
                for i in idx {
                    u16w(w, *i);
                }
            }
        }
        ServerPlay::UnloadChunk { pos } => {
            u8w(w, 4);
            i32w(w, pos.x);
            i32w(w, pos.z);
        }
        ServerPlay::BlockUpdate { pos, id } => {
            u8w(w, 5);
            pos_w(w, *pos);
            u16w(w, *id);
        }
        ServerPlay::PlayerCorrection {
            tick,
            pos,
            vel,
            on_ground,
            health,
            fall_distance,
            ride,
            rooted,
            fx,
            adv,
        } => {
            u8w(w, 6);
            u32w(w, *tick);
            vec_w(w, *pos);
            vec_w(w, *vel);
            boolw(w, *on_ground);
            u8w(w, *health);
            f32w(w, *fall_distance);
            u8w(w, *ride);
            u8w(w, *rooted);
            u8w(w, *fx);
            u16w(w, *adv);
        }
        ServerPlay::EntitySpawn {
            id,
            kind,
            pos,
            item,
            hp,
            state,
            variant,
        } => {
            u8w(w, 7);
            eid_w(w, *id);
            u8w(w, *kind);
            vec_w(w, *pos);
            stack_w(w, *item);
            u8w(w, *hp);
            u8w(w, *state);
            u8w(w, *variant);
        }
        ServerPlay::EntityPos {
            id,
            pos,
            yaw,
            pitch,
            hp,
            state,
        } => {
            u8w(w, 8);
            eid_w(w, *id);
            vec_w(w, *pos);
            f32w(w, *yaw);
            f32w(w, *pitch);
            u8w(w, *hp);
            u8w(w, *state);
        }
        ServerPlay::EntityDespawn { id } => {
            u8w(w, 9);
            eid_w(w, *id);
        }
        ServerPlay::Inventory(inv) => {
            u8w(w, 10);
            inv_w(w, inv);
        }
        ServerPlay::OpenWindow { kind, pos } => {
            u8w(w, 11);
            u8w(
                w,
                match kind {
                    OpenKind::Inventory => 0,
                    OpenKind::CraftingTable => 1,
                    OpenKind::Chest => 2,
                    OpenKind::Etch => 3,
                    OpenKind::Brew => 4,
                    OpenKind::Trade => 5,
                    OpenKind::Vault => 6,
                },
            );
            match pos {
                None => u8w(w, 0),
                Some(p) => {
                    u8w(w, 1);
                    pos_w(w, *p);
                }
            }
        }
        ServerPlay::CloseWindow => u8w(w, 12),
        ServerPlay::Particles { kind, pos, block } => {
            u8w(w, 13);
            u8w(
                w,
                match kind {
                    ParticleKind::Break => 0,
                    ParticleKind::Place => 1,
                    ParticleKind::Hearts => 2,
                    ParticleKind::Steam => 3,
                    ParticleKind::Spore => 4,
                    ParticleKind::Leaf => 5,
                    ParticleKind::Sting => 6,
                    ParticleKind::Wisp => 7,
                },
            );
            vec_w(w, *pos);
            u16w(w, *block);
        }
        ServerPlay::Sound { kind, pos, block } => {
            u8w(w, 14);
            u8w(
                w,
                match kind {
                    SoundKind::Step => 0,
                    SoundKind::Break => 1,
                    SoundKind::Place => 2,
                    SoundKind::Hurt => 3,
                    SoundKind::Pickup => 4,
                    SoundKind::Call => 5,
                    SoundKind::Hum => 6,
                },
            );
            vec_w(w, *pos);
            u16w(w, *block);
        }
        ServerPlay::Hurt { health } => {
            u8w(w, 15);
            u8w(w, *health);
        }
        ServerPlay::Respawned { pos, health, inventory } => {
            u8w(w, 16);
            vec_w(w, *pos);
            u8w(w, *health);
            inv_w(w, inventory);
        }
    }
}

fn decode_server(r: &mut &[u8]) -> Result<ServerPlay, CodecError> {
    let tag = u8r(r)?;
    let m = match tag {
        1 => ServerPlay::JoinGame {
            entity: eid_r(r)?,
            seed: i64r(r)?,
            tick: u32r(r)?,
            spawn: vec_r(r)?,
            pos: vec_r(r)?,
            health: u8r(r)?,
            inventory: inv_r(r)?,
        },
        2 => ServerPlay::KeepTick { tick: u32r(r)? },
        3 => {
            let pos = ChunkPos::new(i32r(r)?, i32r(r)?);
            let n = u8r(r)? as usize;
            let mut sections = Vec::with_capacity(n);
            for _ in 0..n {
                let sy = u8r(r)?;
                let pn = u16r(r)? as usize;
                let mut pal = Vec::with_capacity(pn);
                for _ in 0..pn {
                    pal.push(u16r(r)?);
                }
                let inn = u32r(r)? as usize;
                let mut idx = Vec::with_capacity(inn);
                for _ in 0..inn {
                    idx.push(u16r(r)?);
                }
                sections.push((sy, pal, idx));
            }
            ServerPlay::Chunk { pos, sections }
        }
        4 => ServerPlay::UnloadChunk {
            pos: ChunkPos::new(i32r(r)?, i32r(r)?),
        },
        5 => ServerPlay::BlockUpdate {
            pos: pos_r(r)?,
            id: u16r(r)?,
        },
        6 => ServerPlay::PlayerCorrection {
            tick: u32r(r)?,
            pos: vec_r(r)?,
            vel: vec_r(r)?,
            on_ground: boolr(r)?,
            health: u8r(r)?,
            fall_distance: f32r(r)?,
            ride: u8r(r)?,
            rooted: u8r(r)?,
            fx: u8r(r)?,
            adv: u16r(r)?,
        },
        7 => ServerPlay::EntitySpawn {
            id: eid_r(r)?,
            kind: u8r(r)?,
            pos: vec_r(r)?,
            item: stack_r(r)?,
            hp: u8r(r)?,
            state: u8r(r)?,
            variant: u8r(r)?,
        },
        8 => ServerPlay::EntityPos {
            id: eid_r(r)?,
            pos: vec_r(r)?,
            yaw: f32r(r)?,
            pitch: f32r(r)?,
            hp: u8r(r)?,
            state: u8r(r)?,
        },
        9 => ServerPlay::EntityDespawn { id: eid_r(r)? },
        10 => ServerPlay::Inventory(inv_r(r)?),
        11 => {
            let kind = match u8r(r)? {
                0 => OpenKind::Inventory,
                1 => OpenKind::CraftingTable,
                2 => OpenKind::Chest,
                3 => OpenKind::Etch,
                4 => OpenKind::Brew,
                5 => OpenKind::Trade,
                6 => OpenKind::Vault,
                t => return Err(CodecError::Tag(t)),
            };
            let pos = if boolr(r)? { Some(pos_r(r)?) } else { None };
            ServerPlay::OpenWindow { kind, pos }
        }
        12 => ServerPlay::CloseWindow,
        13 => {
            let kind = match u8r(r)? {
                0 => ParticleKind::Break,
                1 => ParticleKind::Place,
                2 => ParticleKind::Hearts,
                3 => ParticleKind::Steam,
                4 => ParticleKind::Spore,
                5 => ParticleKind::Leaf,
                6 => ParticleKind::Sting,
                7 => ParticleKind::Wisp,
                t => return Err(CodecError::Tag(t)),
            };
            ServerPlay::Particles {
                kind,
                pos: vec_r(r)?,
                block: u16r(r)?,
            }
        }
        14 => {
            let kind = match u8r(r)? {
                0 => SoundKind::Step,
                1 => SoundKind::Break,
                2 => SoundKind::Place,
                3 => SoundKind::Hurt,
                4 => SoundKind::Pickup,
                5 => SoundKind::Call,
                6 => SoundKind::Hum,
                t => return Err(CodecError::Tag(t)),
            };
            ServerPlay::Sound {
                kind,
                pos: vec_r(r)?,
                block: u16r(r)?,
            }
        }
        15 => ServerPlay::Hurt { health: u8r(r)? },
        16 => ServerPlay::Respawned {
            pos: vec_r(r)?,
            health: u8r(r)?,
            inventory: inv_r(r)?,
        },
        t => return Err(CodecError::Tag(t)),
    };
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientPlay;

    #[test]
    fn client_roundtrip() {
        let m = ClientPlay::TickInput {
            tick: 9,
            yaw: 1.2,
            pitch: 0.1,
            forward: 1.0,
            strafe: 0.0,
            jump: true,
            sneak: false,
            sprint: true,
        };
        let b = encode_client(&m);
        assert_eq!(decode_client(&b).expect("roundtrip of a well-formed TickInput"), m);
    }

    #[test]
    fn server_roundtrip_join() {
        let m = ServerPlay::KeepTick { tick: 3 };
        let b = encode_packet(&m);
        assert_eq!(decode_packet(&b).expect("roundtrip of KeepTick"), m);
    }
}
