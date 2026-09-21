use crate::mind::{away, toward, wander, Sight, Will, AGGRO, CALM, FLEE};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.dist < 6.0 || s.herd || (s.blood && s.night) {
        let speed = if s.blood && s.night { 0.14 } else { 0.16 };
        let (x, z, yaw) = if s.blood && s.night { toward(s, s.player, speed) } else { away(s, speed) };
        w.x = x; w.z = z; w.yaw = yaw;
        w.state = if s.blood && s.night { AGGRO } else { FLEE };
        if s.blood && s.night && s.dist < 1.2 { w.hit = 1; }
        if s.tick % 12 == 0 { w.jump = 0.42; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.05);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    if s.tick % 30 == 0 { w.jump = 0.3; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(dist: f32, night: bool, blood: bool) -> Sight {
        Sight {
            tick: 1,
            night,
            blood,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            yaw: 0.0,
            hp: 6,
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
    fn close_flees_blood_night_chases() {
        let flee = think(&sight(3.0, false, false));
        assert_eq!(flee.state, FLEE);
        assert!(flee.x < 0.0);

        let chase = think(&sight(8.0, true, true));
        assert_eq!(chase.state, AGGRO);
        assert!(chase.x > 0.0);
    }
}
