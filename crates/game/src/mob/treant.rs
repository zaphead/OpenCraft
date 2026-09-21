use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.dist < 14.0 {
        let (x, z, yaw) = toward(s, s.player, 0.05);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 3.2 && s.timer == 0 { w.root = 30; w.timer = 50; }
        else { w.timer = s.timer.saturating_sub(1); }
        if s.dist < 2.2 { w.hit = 4; }
        return w;
    }
    w.state = CALM;
    w.timer = s.timer.saturating_sub(1);
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(dist: f32, timer: u16) -> Sight {
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
            timer,
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
    fn close_roots_far_calm() {
        let close = think(&sight(2.0, 0));
        assert_eq!(close.root, 30);
        assert_eq!(close.timer, 50);
        assert_eq!(close.state, AGGRO);

        let far = think(&sight(14.0, 0));
        assert_eq!(far.state, CALM);
        assert_eq!(far.root, 0);
    }
}
