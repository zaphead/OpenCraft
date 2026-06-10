use engine_core::SystemContext;
use game::{
    local_player_entity, player_spawn_center_z_at, spawn_net_player, ActivePlayMode, LocalPlayerId,
    NetworkClient, PlayMode, Transform,
};
use glam::Vec3;

use crate::systems::input::PendingWinitInput;
use crate::systems::spectator::SpectatorCamera;

pub fn toggle_play_mode_system(ctx: &mut SystemContext<'_>) {
    if ctx.resources.get::<NetworkClient>().is_some() {
        return;
    }
    let toggle = ctx
        .resources
        .get::<PendingWinitInput>()
        .map(|pending| pending.0.toggle_play_mode)
        .unwrap_or(false);
    if !toggle {
        return;
    }

    let Some(mode) = ctx.resources.get::<ActivePlayMode>().copied() else {
        return;
    };

    match mode.0 {
        PlayMode::Survival => enter_spectator(ctx),
        PlayMode::Spectator => enter_survival(ctx),
    }

    if let Some(active) = ctx.resources.get_mut::<ActivePlayMode>() {
        active.0 = match mode.0 {
            PlayMode::Survival => PlayMode::Spectator,
            PlayMode::Spectator => PlayMode::Survival,
        };
    }
}

fn enter_spectator(ctx: &mut SystemContext<'_>) {
    if let Some(entity) = local_player_entity(ctx) {
        if let Ok(transform) = ctx.world.get::<&Transform>(entity) {
            if let Some(camera) = ctx.resources.get_mut::<SpectatorCamera>() {
                camera.position = transform.position + Vec3::new(0.0, 0.0, 0.62);
                camera.yaw = transform.yaw;
                camera.pitch = transform.pitch;
            }
        }
        let _ = ctx.world.despawn(entity);
    }

    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.spawned = false;
        local.id = None;
    }
}

fn enter_survival(ctx: &mut SystemContext<'_>) {
    let (position, yaw, pitch) = ctx
        .resources
        .get::<SpectatorCamera>()
        .map(|camera| (camera.position, camera.yaw, camera.pitch))
        .unwrap_or((Vec3::new(0.5, 0.5, player_spawn_center_z_at(0, 0)), 0.0, -0.2));

    let column_x = position.x.floor() as i32;
    let column_y = position.y.floor() as i32;
    let spawn_z = player_spawn_center_z_at(column_x, column_y);
    let spawn_position = Vec3::new(position.x, position.y, spawn_z);

    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.id = Some(0);
        local.spawned = false;
    }

    spawn_net_player(ctx, 0, Some((spawn_position, yaw, pitch)));

    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.id = Some(0);
        local.spawned = true;
    }
}
