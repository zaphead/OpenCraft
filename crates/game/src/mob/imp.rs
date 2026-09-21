use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    let (x, z, yaw) = toward(s, s.player, 0.1);
    w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
    if s.dist < 1.4 { w.hit = 2; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(lava: bool) -> Sight {
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
            player: Vec3::new(8.0, 0.0, 0.0),
            dist: 8.0,
            herd: false,
            item: None,
            water: false,
            lava,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn lava_still_chases_aggro() {
        let w = think(&sight(true));
        assert_eq!(w.state, AGGRO);
        assert!(w.x != 0.0 || w.z != 0.0);
    }
}
