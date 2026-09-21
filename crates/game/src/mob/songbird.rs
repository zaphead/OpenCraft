use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    w.fly = true;
    let (x, z, yaw) = wander(s, 0.09);
    w.x = x; w.z = z; w.yaw = yaw;
    if s.dist < 18.0 && s.tick % 50 == 0 { w.call = true; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(tick: u32, dist: f32) -> Sight {
        Sight {
            tick,
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
    fn flies_and_calls_when_close_on_interval() {
        let w = think(&sight(0, 10.0));
        assert!(w.fly);
        assert!(w.call);
    }
}
