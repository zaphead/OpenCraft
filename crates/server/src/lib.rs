//! Authority: 20 TPS sim, embeddable and dedicated.

mod embed;
mod session;
mod sim;

pub use embed::{pace_tick, spawn_embedded, EmbeddedHandle, ServerConfig};
pub use sim::{Sim, VIEW_CHUNKS};
