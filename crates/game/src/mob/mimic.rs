use crate::mind::{toward, Sight, Will, AGGRO, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.state == SHUT {
        w.state = SHUT;
        return w;
    }
    let (x, z, yaw) = toward(s, s.player, 0.12);
    w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
    if s.dist < 1.5 { w.hit = 5; }
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
    fn shut_holds_aggro_hits() {
        let shut = think(&sight(SHUT, 1.0));
        assert_eq!(shut.state, SHUT);
        assert_eq!(shut.x, 0.0);
        assert_eq!(shut.z, 0.0);
        assert_eq!(shut.hit, 0);

        let hit = think(&sight(AGGRO, 1.0));
        assert_eq!(hit.state, AGGRO);
        assert_eq!(hit.hit, 5);
    }
}
