use glam::Vec3;

/// World up axis (Z-up convention).
pub const UP: Vec3 = Vec3::Z;

/// Horizontal forward on the XY plane from yaw (radians). Yaw 0 → +Y.
pub fn horizontal_forward(yaw: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    Vec3::new(sy, cy, 0.0)
}

/// Horizontal right on the XY plane, perpendicular to [`horizontal_forward`].
pub fn horizontal_right(yaw: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    Vec3::new(cy, -sy, 0.0)
}

/// Full view direction including pitch. Matches `engine_render::Camera::forward`.
pub fn view_forward(yaw: f32, pitch: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    Vec3::new(sy * cp, cy * cp, sp).normalize()
}

/// Survival player AABB half-extents (Z-up, 2 blocks tall).
pub const PLAYER_HALF_EXTENTS: Vec3 = Vec3::new(0.28, 0.28, 0.95);

/// Eye offset above the player collider center (matches survival camera).
pub const PLAYER_EYE_OFFSET_Z: f32 = 0.62;

/// Offset below collider center for grounded checks.
pub fn grounded_probe_offset(half_height: f32) -> Vec3 {
    Vec3::new(0.0, 0.0, -(half_height + 0.05))
}
