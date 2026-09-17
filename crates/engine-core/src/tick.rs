pub const TICK_MS: u64 = 50;
pub const TPS: u32 = 20;

/// `(now - last_tick) / 50ms`, clamped to `[0, 1]`.
pub fn partial_tick(now_ms: u64, last_tick_ms: u64) -> f32 {
    let dt = now_ms.saturating_sub(last_tick_ms) as f32 / TICK_MS as f32;
    dt.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_tick_clamps() {
        assert_eq!(partial_tick(0, 0), 0.0);
        assert_eq!(partial_tick(25, 0), 0.5);
        assert_eq!(partial_tick(50, 0), 1.0);
        assert_eq!(partial_tick(80, 0), 1.0);
    }
}
