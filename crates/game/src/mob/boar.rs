use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.state == AGGRO || (s.blood && s.night) {
        let (x, z, yaw) = toward(s, s.player, 0.13);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.5 { w.hit = 3; }
        if s.dist > 24.0 && !(s.blood && s.night) { w.state = CALM; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.06);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
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
    fn aggro_on_hit_chases_calm_wanders() {
        let calm = think(&sight(CALM, 8.0));
        assert_eq!(calm.state, CALM);
        assert_eq!(calm.hit, 0);
        assert!(calm.x != 0.0 || calm.z != 0.0);

        let aggro = think(&sight(AGGRO, 1.0));
        assert_eq!(aggro.state, AGGRO);
        assert_eq!(aggro.hit, 3);
        assert!(aggro.x != 0.0 || aggro.z != 0.0);
    }
}
