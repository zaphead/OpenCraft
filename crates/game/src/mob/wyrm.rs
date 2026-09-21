use crate::mind::{toward, Sight, Will, AGGRO, CALM};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.dist < 24.0 {
        let (x, z, yaw) = toward(s, s.player, 0.1);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 2.4 { w.hit = 6; }
        return w;
    }
    w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(dist: f32) -> Sight {
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
    fn near_aggro_far_calm_no_hit() {
        let near = think(&sight(10.0));
        assert_eq!(near.state, AGGRO);

        let far = think(&sight(40.0));
        assert_eq!(far.state, CALM);
        assert_eq!(far.hit, 0);
    }
}
