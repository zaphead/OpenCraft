use engine_render::Camera;
use game::{DebugWorldKind, PlayMode};

pub fn format_debug_hud(
    camera: &Camera,
    mode: Option<PlayMode>,
    world: Option<DebugWorldKind>,
) -> String {
    let mode_line = mode.map(PlayMode::label).unwrap_or("");
    let world_line = world.map(DebugWorldKind::label).unwrap_or("");
    let pos = camera.position;
    let yaw_deg = camera.yaw.to_degrees();
    let pitch_deg = camera.pitch.to_degrees();
    let forward = camera.forward();

    format!(
        "{mode_line}\n\
         {world_line}\n\
         POS\n\
         X {:>7.1}\n\
         Y {:>7.1}\n\
         Z {:>7.1}\n\
         ROT\n\
         YAW {:>6.1}\n\
         PIT {:>6.1}\n\
         DIR\n\
         X {:>6.2}\n\
         Y {:>6.2}\n\
         Z {:>6.2}",
        pos.x,
        pos.y,
        pos.z,
        yaw_deg,
        pitch_deg,
        forward.x,
        forward.y,
        forward.z,
    )
}
