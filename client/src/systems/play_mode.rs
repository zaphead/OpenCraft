use engine_core::SystemContext;
use game::{
    local_player_entity, player_spawn_center_z_at, spawn_net_player, ActiveDebugWorld, WorldSeed,
    PLAYER_SPAWN_PITCH,
    ActivePlayMode, LocalPlayerId, NetworkClient, PlayMode, Transform, Velocity,
    PLAYER_EYE_OFFSET_Z,
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
                camera.position = transform.position + Vec3::new(0.0, 0.0, PLAYER_EYE_OFFSET_Z);
                camera.yaw = transform.yaw;
                camera.pitch = transform.pitch;
                if let Ok(velocity) = ctx.world.get::<&Velocity>(entity) {
                    camera.velocity = velocity.0;
                }
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
    let world = ctx
        .resources
        .get::<ActiveDebugWorld>()
        .map(|active| active.0)
        .unwrap_or_default();
    let seed = ctx
        .resources
        .get::<WorldSeed>()
        .map(|seed| seed.0)
        .unwrap_or(0);
    let yaw = ctx
        .resources
        .get::<SpectatorCamera>()
        .map(|camera| camera.yaw)
        .unwrap_or(0.0);
    let spawn_position = Vec3::new(0.5, 0.5, player_spawn_center_z_at(0, 0, world, seed));

    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.id = Some(0);
        local.spawned = false;
    }

    spawn_net_player(ctx, 0, Some((spawn_position, yaw, PLAYER_SPAWN_PITCH)));

    if let Some(local) = ctx.resources.get_mut::<LocalPlayerId>() {
        local.id = Some(0);
        local.spawned = true;
    }
}
