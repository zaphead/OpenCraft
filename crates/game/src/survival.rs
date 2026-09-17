pub const MAX_HEALTH: u8 = 20;

/// Java-public: fall damage ≈ fall_distance - 3.
pub fn apply_fall_damage(health: u8, fall_distance: f32) -> u8 {
    if fall_distance <= 3.0 {
        return health;
    }
    let dmg = (fall_distance - 3.0).floor() as u8;
    health.saturating_sub(dmg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_fall_is_safe() {
        assert_eq!(apply_fall_damage(20, 2.5), 20);
    }

    #[test]
    fn long_fall_kills() {
        assert_eq!(apply_fall_damage(20, 24.0), 0);
    }
}
