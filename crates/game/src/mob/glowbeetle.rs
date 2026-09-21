use crate::mind::{wander, Sight, Will};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    w.fly = true;
    let (x, z, yaw) = wander(s, 0.04);
    w.x = x; w.z = z; w.yaw = yaw;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mind::CALM;
    use engine_core::Vec3;

    fn sight() -> Sight {
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
            lava: false,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn flies_with_small_horizontal_wish() {
        let w = think(&sight());
        assert!(w.fly);
        let speed = (w.x * w.x + w.z * w.z).sqrt();
        assert!(speed > 0.0 && speed <= 0.05);
    }
}
