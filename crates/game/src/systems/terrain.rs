use engine_assets::BlockRegistry;
use engine_core::SystemContext;
use engine_world::{BlockPos, WorldMutationQueue};

use crate::components::TerrainGeneration;
use crate::debug_world::{ActiveDebugWorld, DebugWorldKind};

pub const WORLD_RADIUS: i32 = 16;
pub const ROLLING_HILLS_RADIUS: i32 = 64;
pub const DEBUG_BLOCK_Z: i32 = 0;
pub const DEBUG_BLOCK_SPACING: i32 = 7;
/// Legacy flat-plane reference for the three-block debug layout.
pub const GRASS_PLANE_Z: i32 = DEBUG_BLOCK_Z;

const PLAYER_HALF_HEIGHT: f32 = 1.0;
const TERRAIN_SEED: u32 = 0xC0FFEE;
const BASE_HEIGHT: i32 = 4;
const HEIGHT_AMPLITUDE: i32 = 6;
const NOISE_SCALE: f32 = 0.045;

pub fn rolling_hills_max_surface_z() -> i32 {
    BASE_HEIGHT + HEIGHT_AMPLITUDE
}

pub fn generate_terrain_system(ctx: &mut SystemContext<'_>) {
    let Some(progress) = ctx.resources.get::<TerrainGeneration>().copied() else {
        return;
    };
    if progress.complete {
        return;
    }

    let Some(registry) = ctx.resources.get::<BlockRegistry>() else {
        return;
    };
    let Some(grass) = registry.id_by_name("grass") else {
        return;
    };
    let Some(dirt) = registry.id_by_name("dirt") else {
        return;
    };
    let Some(stone) = registry.id_by_name("stone") else {
        return;
    };

    let world = ctx
        .resources
        .get::<ActiveDebugWorld>()
        .map(|active| active.0)
        .unwrap_or(DebugWorldKind::ThreeBlocks);

    let Some(queue) = ctx.resources.get_mut::<WorldMutationQueue>() else {
        return;
    };

    match world {
        DebugWorldKind::ThreeBlocks => generate_three_blocks(queue, grass, dirt, stone),
        DebugWorldKind::RollingHills => {
            generate_rolling_hills(queue, grass, dirt, stone);
        }
    }

    if let Some(progress) = ctx.resources.get_mut::<TerrainGeneration>() {
        progress.complete = true;
    }
}

fn generate_three_blocks(
    queue: &mut WorldMutationQueue,
    grass: engine_world::BlockId,
    dirt: engine_world::BlockId,
    stone: engine_world::BlockId,
) {
    let debug_blocks = [
        (BlockPos::new(0, 0, DEBUG_BLOCK_Z), grass),
        (BlockPos::new(DEBUG_BLOCK_SPACING, 0, DEBUG_BLOCK_Z), dirt),
        (
            BlockPos::new(DEBUG_BLOCK_SPACING * 2, 0, DEBUG_BLOCK_Z),
            stone,
        ),
    ];

    for (pos, block) in debug_blocks {
        queue.set_block(pos, block);
    }
}

fn generate_rolling_hills(
    queue: &mut WorldMutationQueue,
    grass: engine_world::BlockId,
    dirt: engine_world::BlockId,
    stone: engine_world::BlockId,
) {
    for x in -ROLLING_HILLS_RADIUS..ROLLING_HILLS_RADIUS {
        for y in -ROLLING_HILLS_RADIUS..ROLLING_HILLS_RADIUS {
            let surface = terrain_surface_z(x, y, DebugWorldKind::RollingHills);
            for z in 0..=surface {
                let block = if z == surface {
                    grass
                } else if z >= surface - 3 {
                    dirt
                } else {
                    stone
                };
                queue.set_block(BlockPos::new(x, y, z), block);
            }
        }
    }
}

pub fn terrain_surface_z(x: i32, y: i32, world: DebugWorldKind) -> i32 {
    match world {
        DebugWorldKind::ThreeBlocks => {
            if y == 0
                && (x == 0 || x == DEBUG_BLOCK_SPACING || x == DEBUG_BLOCK_SPACING * 2)
            {
                DEBUG_BLOCK_Z
            } else {
                DEBUG_BLOCK_Z - 1
            }
        }
        DebugWorldKind::RollingHills => {
            let n = rolling_hills_noise(x as f32, y as f32);
            BASE_HEIGHT + (n * HEIGHT_AMPLITUDE as f32).round() as i32
        }
    }
}

pub fn player_spawn_center_z_at(x: i32, y: i32, world: DebugWorldKind) -> f32 {
    terrain_surface_z(x, y, world) as f32 + 1.0 + PLAYER_HALF_HEIGHT
}

pub fn player_spawn_center_z(world: DebugWorldKind) -> f32 {
    player_spawn_center_z_at(0, 0, world)
}

fn rolling_hills_noise(x: f32, y: f32) -> f32 {
    let mut sum = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut norm = 0.0;

    for _ in 0..4 {
        let sample = value_noise_2d(x * NOISE_SCALE * frequency, y * NOISE_SCALE * frequency);
        sum += sample * amplitude;
        norm += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    (sum / norm).clamp(-1.0, 1.0)
}

fn value_noise_2d(x: f32, y: f32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    let tx = smoothstep(x - x0 as f32);
    let ty = smoothstep(y - y0 as f32);

    let n00 = hash_to_unit(x0, y0);
    let n10 = hash_to_unit(x1, y0);
    let n01 = hash_to_unit(x0, y1);
    let n11 = hash_to_unit(x1, y1);

    let nx0 = lerp(n00, n10, tx);
    let nx1 = lerp(n01, n11, tx);
    lerp(nx0, nx1, ty) * 2.0 - 1.0
}

fn hash_to_unit(x: i32, y: i32) -> f32 {
    let mut n = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(TERRAIN_SEED as i32) as u32;
    n ^= n >> 13;
    n = n.wrapping_mul(1274126177);
    (n & 0xFFFF) as f32 / 65535.0
}

fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
