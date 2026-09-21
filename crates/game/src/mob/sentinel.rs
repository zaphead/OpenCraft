use crate::mind::{toward, Sight, Will, AGGRO, ASLEEP, CALM};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.state == ASLEEP || s.state == CALM {
        w.state = ASLEEP;
        return w;
    }
    let (x, z, yaw) = toward(s, s.player, 0.08);
    w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
    if s.dist < 1.8 { w.hit = 5; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(state: u8, dist: f32) -> Sight {
        Sight {
            tick: 0,
            night: false,
            blood: false,
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
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn asleep_holds_aggro_chases() {
        let sleep = think(&sight(ASLEEP, 5.0));
        assert_eq!(sleep.state, ASLEEP);
        assert_eq!(sleep.x, 0.0);
        assert_eq!(sleep.z, 0.0);
        assert_eq!(sleep.hit, 0);

        let chase = think(&sight(AGGRO, 5.0));
        assert_eq!(chase.state, AGGRO);
        assert!(chase.x > 0.0);
    }
}
