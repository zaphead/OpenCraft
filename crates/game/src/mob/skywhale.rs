use crate::mind::{wander, Sight, Will};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    w.fly = true;
    let (x, z, yaw) = wander(s, 0.08);
    w.x = x; w.z = z; w.yaw = yaw;
    if s.tick % 200 == 0 { w.spawn_goblin = true; }
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
            hp: 40,
            state: CALM,
            tame: 0,
            stamina: 0,
            variant: 0,
            timer: 0,
            player: Vec3::new(32.0, 0.0, 0.0),
            dist: 32.0,
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
    fn tick0_spawns_goblin_and_flies_never_hits() {
        let w0 = think(&sight(0));
        assert!(w0.spawn_goblin);
        assert!(w0.fly);
        assert_eq!(w0.hit, 0);

        let w1 = think(&sight(1));
        assert!(!w1.spawn_goblin);
        assert!(w1.fly);
        assert_eq!(w1.hit, 0);
    }
}
