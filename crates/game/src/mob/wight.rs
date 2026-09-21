use crate::mind::{toward, Sight, Will, AGGRO, CALM};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.dist < 6.0 || s.state == AGGRO {
        let (x, z, yaw) = toward(s, s.player, 0.14);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.5 { w.hit = 4; }
        if s.dist > 22.0 { w.state = CALM; }
        return w;
    }
    w.state = CALM;
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
    fn calm_holds_at_ten_chases_at_four() {
        let hold = think(&sight(CALM, 10.0));
        assert_eq!(hold.state, CALM);
        assert_eq!(hold.x, 0.0);
        assert_eq!(hold.z, 0.0);

        let chase = think(&sight(CALM, 4.0));
        assert_eq!(chase.state, AGGRO);
        assert!(chase.x > 0.0 || chase.z != 0.0);
    }
}
