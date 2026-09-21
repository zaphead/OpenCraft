use crate::mind::{wander, Sight, Will};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.tick > 0 && s.tick % 35 == 0 {
        let (x, z, yaw) = wander(s, 0.16);
        w.x = x;
        w.z = z;
        w.yaw = yaw;
        w.jump = 0.38;
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mind::CALM;
    use engine_core::Vec3;

    fn sight(tick: u32) -> Sight {
        Sight {
            tick,
            night: false,
            blood: false,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            yaw: 0.0,
            hp: 4,
            state: CALM,
            tame: 0,
            stamina: 0,
            variant: 0,
            timer: 0,
            player: Vec3::ZERO,
            dist: 20.0,
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
    fn sits_then_hops() {
        let hold = think(&sight(0));
        assert_eq!(hold.x, 0.0);
        assert_eq!(hold.jump, 0.0);

        let hop = think(&sight(35));
        assert!(hop.jump > 0.0);
        assert!((hop.jump - 0.38).abs() < 0.001);
    }
}
