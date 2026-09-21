use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    let (x, z, yaw) = toward(s, s.player, 0.045);
    w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
    if s.dist < 1.6 { w.hit = 4; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(dist: f32) -> Sight {
        Sight {
            tick: 0,
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
    fn slow_chase_hits_hard_up_close() {
        let w = think(&sight(1.0));
        let speed = (w.x * w.x + w.z * w.z).sqrt();
        assert!(speed < 0.06);
        assert_eq!(w.hit, 4);
    }
}
