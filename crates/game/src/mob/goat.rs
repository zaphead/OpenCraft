use crate::mind::{away, wander, Sight, Will};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    let (x, z, yaw) = if s.dist < 4.0 { away(s, 0.1) } else { wander(s, 0.07) };
    w.x = x; w.z = z; w.yaw = yaw;
    if s.tick % 28 == 0 { w.jump = 0.62; }
    if s.dist < 8.0 && s.tick % 40 == 0 { w.call = true; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mind::CALM;
    use engine_core::Vec3;

    fn sight(tick: u32, dist: f32) -> Sight {
        Sight {
            tick,
            night: false,
            blood: false,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            yaw: 0.0,
            hp: 10,
            state: CALM,
            tame: 0,
            stamina: 0,
            variant: 0,
            timer: 0,
            player: Vec3::new(dist, 0.0, 0.0),
            dist,
            herd: false,
            item: None,
            water: false,
            lava: false,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn close_player_calls_and_jump_tick() {
        // tick 0 hits both jump (%28) and call (%40) intervals
        let w = think(&sight(0, 5.0));
        assert!(w.call);
        assert!(w.jump > 0.5);
    }
}
