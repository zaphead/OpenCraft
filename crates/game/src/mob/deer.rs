use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    let scared = s.herd || s.dist < 9.0 || s.state == FLEE;
    if s.blood && s.night {
        let (x, z, yaw) = toward(s, s.player, 0.14);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.6 { w.hit = 2; }
        return w;
    }
    if scared {
        let (x, z, yaw) = away(s, 0.18);
        w.x = x; w.z = z; w.yaw = yaw;
        w.state = if s.dist > 20.0 && !s.herd { CALM } else { FLEE };
        return w;
    }
    let (x, z, yaw) = wander(s, 0.06);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(dist: f32, herd: bool, blood: bool, night: bool) -> Sight {
        Sight {
            tick: 0,
            night,
            blood,
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
            herd,
            item: None,
            water: false,
            lava: false,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn herd_flees_at_large_dist() {
        let w = think(&sight(40.0, true, false, false));
        assert_eq!(w.state, FLEE);
        assert!(w.x < 0.0);
    }

    #[test]
    fn blood_night_chases() {
        let w = think(&sight(5.0, false, true, true));
        assert_eq!(w.state, AGGRO);
        assert!(w.x > 0.0);
    }
}
