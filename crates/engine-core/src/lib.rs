//! Shared time, IDs, math re-exports, and coordinate types.

pub mod ids;
pub mod pos;
pub mod tick;

pub use glam::{self, DVec3, IVec3, Mat4, Quat, Vec2, Vec3, Vec4};
pub use ids::EntityId;
pub use pos::{BlockPos, ChunkPos, SectionPos};
pub use tick::{partial_tick, TICK_MS, TPS};

/// One paletted cube. World height for this slice is `[MIN_Y, MAX_Y)`.
pub const SECTION_EDGE: i32 = 16;
pub const MIN_Y: i32 = 0;
pub const MAX_Y: i32 = 128;
pub const SECTIONS_PER_CHUNK: i32 = (MAX_Y - MIN_Y) / SECTION_EDGE;

pub type FxHashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type FxHashSet<K> = rustc_hash::FxHashSet<K>;

/// Yaw/pitch (radians, Java-shaped: +X east, +Z south, Y-up) to unit look direction.
/// Single canonical spelling shared by simulation and rendering.
pub fn yaw_pitch_dir(yaw: f32, pitch: f32) -> Vec3 {
    Vec3::new(-yaw.sin() * pitch.cos(), -pitch.sin(), yaw.cos() * pitch.cos())
}
