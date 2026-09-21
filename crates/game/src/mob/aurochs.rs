use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.ridden { return w; }
    if s.blood && s.night {
        let (x, z, yaw) = toward(s, s.player, 0.12);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.8 { w.hit = 2; }
        return w;
    }
    if s.state == FLEE || s.state == AGGRO && !s.blood {
        let (x, z, yaw) = away(s, 0.14);
        w.x = x; w.z = z; w.yaw = yaw; w.state = if s.dist > 16.0 { CALM } else { FLEE };
        return w;
    }
    let (x, z, yaw) = wander(s, 0.05);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(blood: bool, night: bool, state: u8, dist: f32) -> Sight {
        Sight {
            tick: 0,
            night,
            blood,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            yaw: 0.0,
            hp: 10,
            state,
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
            grass: true,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn blood_night_chase_vs_calm_flee() {
        let chase = think(&sight(true, true, CALM, 1.0));
        assert_eq!(chase.state, AGGRO);
        assert_eq!(chase.hit, 2);
        assert!(chase.x > 0.0);

        let day = think(&sight(false, false, CALM, 1.0));
        assert_eq!(day.state, CALM);
        assert_eq!(day.hit, 0);

        let flee = think(&sight(false, false, FLEE, 8.0));
        assert_eq!(flee.state, FLEE);
        assert!(flee.x < 0.0);

        let calm = think(&sight(false, false, FLEE, 17.0));
        assert_eq!(calm.state, CALM);
    }
}
