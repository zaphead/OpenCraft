use crate::mind::{toward, wander, Sight, Will, AGGRO, CALM};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    w.fly = true;
    w.timer = s.timer.saturating_sub(1);
    if s.state == AGGRO || (s.blood && s.night) {
        let (x, z, yaw) = toward(s, s.player, 0.12);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.4 && w.timer == 0 { w.hit = 2; w.timer = 20; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.07);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    if s.flower && s.grass && w.timer == 0 { w.flower = true; w.timer = 100; }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(state: u8, dist: f32, timer: u16, flower: bool, grass: bool) -> Sight {
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
            timer,
            player: Vec3::new(dist, 0.0, 0.0),
            dist,
            herd: false,
            item: None,
            water: false,
            lava: false,
            grass,
            flower,
            ridden: false,
        }
    }

    #[test]
    fn calm_plants_flower_aggro_stings() {
        let plant = think(&sight(CALM, 8.0, 0, true, true));
        assert!(plant.flower);
        assert_eq!(plant.timer, 100);
        assert_eq!(plant.hit, 0);

        let sting = think(&sight(AGGRO, 1.0, 0, false, false));
        assert_eq!(sting.state, AGGRO);
        assert_eq!(sting.hit, 2);
        assert_eq!(sting.timer, 20);
    }
}
