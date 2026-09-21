//! OpenCraft rules: blocks, items, craft, gather, and the living world.

pub mod biome;
pub mod biomes;
pub mod blocks;
mod clock;
mod craft;
mod inventory;
mod items;
mod life;
pub mod mind;
pub mod mob;
mod mining;
pub mod pieces;
mod stamp;
mod survival;
mod worldgen;

pub use biome::{biome_at, Biome};
pub use blocks::{face_offset, is_cross, is_flora, is_leaf, is_log, is_solid};
pub use clock::{at_secs, at_tick, Phase, Season, DAY_TICKS};
pub use craft::craft_result;
pub use inventory::{
    click_slot, collect_into, drop_all, etch_tool, layout, slot_at, window_len, SlotMap, Window, WindowKind,
    HOTBAR,
};
pub use items::{as_block, food_hearts, is_block_item, is_potion, item_max, ItemId};
pub use life::{
    boss_name, brew, drops, etched, initial_stamina, initial_state, max_hp, residents, spawner_kind,
    ADV_DRINK, ADV_ETCH, ADV_HIDE, ADV_MOON, ADV_RIDE, ADV_TREE, ADV_WAKE, ADV_WYRM,
};
pub use world::Stack;
pub use mining::{break_ticks, harvest_drop, tool_for};
pub use survival::{apply_fall_damage, MAX_HEALTH};
pub use worldgen::{generate_chunk, spawn_pos, surface_y};
