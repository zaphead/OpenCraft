use engine_assets::BlockRegistry;
use engine_core::SystemContext;
use engine_world::{BlockPos, WorldMutationQueue};

use crate::components::{TerrainGeneration, WorldInitialized};

pub const WORLD_RADIUS: i32 = 64;
const COLUMNS_PER_TICK: i32 = 16;

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
    let Some(air) = registry.id_by_name("air") else {
        return;
    };
    let Some(stone) = registry.id_by_name("stone") else {
        return;
    };
    let Some(dirt) = registry.id_by_name("dirt") else {
        return;
    };
    let Some(grass) = registry.id_by_name("grass") else {
        return;
    };

    let total_columns = WORLD_RADIUS * 2;
    let end_column = (progress.next_column + COLUMNS_PER_TICK).min(total_columns);

    let Some(queue) = ctx.resources.get_mut::<WorldMutationQueue>() else {
        return;
    };

    for column in progress.next_column..end_column {
        let x = -WORLD_RADIUS + column;
        for z in -WORLD_RADIUS..WORLD_RADIUS {
            let surface = terrain_height(x, z);
            for y in 0..=surface + 2 {
                let block = if y > surface {
                    air
                } else if y == surface {
                    grass
                } else if y >= surface - 3 {
                    dirt
                } else {
                    stone
                };
                if block != air {
                    queue.set_block(BlockPos::new(x, y, z), block);
                }
            }
        }
    }

    let complete = end_column >= total_columns;
    if let Some(progress) = ctx.resources.get_mut::<TerrainGeneration>() {
        progress.next_column = end_column;
        progress.complete = complete;
    }
    if complete {
        if let Some(flag) = ctx.resources.get_mut::<WorldInitialized>() {
            flag.0 = true;
        }
    }
}

fn terrain_height(x: i32, z: i32) -> i32 {
    let xf = x as f32 * 0.05;
    let zf = z as f32 * 0.05;
    let hills = (xf.sin() * zf.cos()) * 6.0 + (xf * 0.5).cos() * (zf * 0.5).sin() * 4.0;
    14 + hills.round() as i32
}

pub fn spawn_height(x: i32, z: i32) -> i32 {
    terrain_height(x, z) + 2
}
