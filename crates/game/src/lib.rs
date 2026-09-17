//! OpenCraft rules for the core slice: blocks, items, craft, gather, survival-lite.

pub mod blocks;
mod craft;
mod inventory;
mod items;
mod mining;
mod survival;
mod worldgen;

pub use blocks::{face_offset, is_solid};
pub use craft::craft_result;
pub use inventory::{
    click_slot, collect_into, drop_all, layout, slot_at, window_len, SlotMap, Window, WindowKind, HOTBAR,
};
pub use items::{as_block, is_block_item, item_max, ItemId};
pub use world::Stack;
pub use mining::{break_ticks, harvest_drop, tool_for};
pub use survival::{apply_fall_damage, MAX_HEALTH};
pub use worldgen::{generate_chunk, spawn_pos, surface_y};
