use engine_core::SystemContext;
use engine_render::{extract_render_scene, Renderer, RenderWorld};
use game::ActivePlayMode;

use crate::systems::hud::format_debug_hud;

pub struct ClientRenderer(pub Renderer);

pub fn present_frame_system(ctx: &mut SystemContext<'_>) {
    let snapshot = ctx.resources.get::<RenderWorld>().and_then(|world| {
        if world.ready {
            Some((
                world.camera,
                world.opaque.clone(),
                world.cutout.clone(),
                world.animation_tick,
            ))
        } else {
            None
        }
    });
    let Some((camera, opaque, cutout, animation_tick)) = snapshot else {
        return;
    };
    if opaque.vertices.is_empty() && cutout.vertices.is_empty() {
        log::debug!("present skipped: zero meshes in RenderWorld");
        return;
    }

    let play_mode = ctx.resources.get::<ActivePlayMode>().map(|mode| mode.0);
    let hud_text = format_debug_hud(&camera, play_mode);
    let hud_buffer = hud_text;

    let Some(renderer) = ctx.resources.get_mut::<ClientRenderer>() else {
        log::warn!("present skipped: ClientRenderer missing");
        return;
    };

    let scene = extract_render_scene(camera, opaque, cutout, animation_tick, Vec::new());
    renderer.0.upload_meshes(&scene.opaque, &scene.cutout);
    if let Err(error) = renderer.0.render(&scene, Some(&hud_buffer)) {
        log::warn!("render error: {error:?}");
    }
}
