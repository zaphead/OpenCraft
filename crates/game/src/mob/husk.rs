use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if !s.night || s.blood {
        let (x, z, yaw) = toward(s, s.player, 0.09);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.5 { w.hit = 3; }
        return w;
    }
    w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(night: bool, blood: bool, dist: f32) -> Sight {
        Sight {
            tick: 1,
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
    fn night_calm_day_chases() {
        let night = think(&sight(true, false, 4.0));
        assert_eq!(night.state, CALM);

        let day = think(&sight(false, false, 4.0));
        assert_eq!(day.state, AGGRO);
        assert!(day.x > 0.0);
    }
}
