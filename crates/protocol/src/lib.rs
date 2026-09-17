//! Custom protocol: message enums + one binary codec. Not Mojang packets.

mod codec;
mod msg;

pub use codec::{decode_client, decode_packet, encode_client, encode_packet, CodecError};
pub use msg::{
    ClientPlay, InventorySnap, OpenKind, ParticleKind, ServerPlay, SoundKind, PROTOCOL_VERSION,
};

pub const MAX_PACKET: usize = 1 << 22;
