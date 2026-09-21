use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.water {
        let (x, z, yaw) = wander(s, 0.06);
        w.x = x; w.z = z; w.yaw = yaw; w.fly = true;
    } else if s.tick % 15 == 0 {
        w.jump = 0.3;
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(water: bool) -> Sight {
        Sight {
            tick: 1,
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
            dist: 10.0,
            herd: false,
            item: None,
            water,
            lava: false,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn water_sets_fly_dry_does_not() {
        assert!(think(&sight(true)).fly);
        assert!(!think(&sight(false)).fly);
    }
}
