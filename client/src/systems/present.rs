use engine_core::SystemContext;
use engine_render::{extract_render_scene, Renderer, RenderWorld};
use game::{local_player_entity, ActiveDebugWorld, ActivePlayMode, Velocity};
use glam::Vec3;

use crate::systems::hud::format_debug_hud;

pub struct ClientRenderer(pub Renderer);

pub fn present_frame_system(ctx: &mut SystemContext<'_>) {
    let play_mode = ctx.resources.get::<ActivePlayMode>().map(|mode| mode.0);
    let debug_world = ctx.resources.get::<ActiveDebugWorld>().map(|active| active.0);
    let velocity = local_player_entity(ctx)
        .and_then(|entity| ctx.world.get::<&Velocity>(entity).ok())
        .map(|velocity| velocity.0)
        .unwrap_or(Vec3::ZERO);

    let presented = ctx
        .resources
        .with_pair::<RenderWorld, ClientRenderer, _>(|world, renderer| {
            if !world.ready {
                return false;
            }
            if world.opaque.vertices.is_empty() && world.cutout.vertices.is_empty() {
                log::debug!("present skipped: zero meshes in RenderWorld");
                return false;
            }

            let hud_text = format_debug_hud(&world.camera, play_mode, debug_world, velocity);
            renderer.0.sync_meshes(
                world.mesh_generation,
                &world.opaque,
                &world.cutout,
            );
            let scene = extract_render_scene(
                world.camera,
                Default::default(),
                Default::default(),
                world.animation_tick,
                Vec::new(),
                world.target_block,
            );
            if let Err(error) = renderer.0.render(&scene, Some(&hud_text)) {
                log::warn!("render error: {error:?}");
            }
            true
        })
        .unwrap_or(false);
    if !presented {
        return;
    }
}
