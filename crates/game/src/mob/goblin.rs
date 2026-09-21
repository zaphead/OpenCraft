use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.night || s.blood {
        let (x, z, yaw) = toward(s, s.player, 0.11);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 12.0 && s.tick % 30 == 0 { w.call = true; }
        if s.dist < 1.5 { w.hit = 3; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.04);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(night: bool, dist: f32) -> Sight {
        Sight {
            tick: 0,
            night,
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
    fn day_wanders_calm_night_chases() {
        let day = think(&sight(false, 5.0));
        assert_eq!(day.state, CALM);
        assert_eq!(day.hit, 0);

        let night = think(&sight(true, 5.0));
        assert_eq!(night.state, AGGRO);
        assert!(night.x > 0.0);
    }
}
