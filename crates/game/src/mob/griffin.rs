use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.ridden { return w; }
    w.fly = true;
    let (x, z, yaw) = wander(s, 0.1);
    w.x = x; w.z = z; w.yaw = yaw;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(ridden: bool) -> Sight {
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
            player: Vec3::ZERO,
            dist: 8.0,
            herd: false,
            item: None,
            water: false,
            lava: false,
            grass: false,
            flower: false,
            ridden,
        }
    }

    #[test]
    fn unridden_flies_ridden_holds() {
        let free = think(&sight(false));
        assert!(free.fly);

        let mount = think(&sight(true));
        assert_eq!(mount.x, 0.0);
        assert_eq!(mount.z, 0.0);
    }
}
