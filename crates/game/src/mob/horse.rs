use crate::mind::{toward, wander, Sight, Will, AGGRO, CALM};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if s.ridden {
        let sprint = s.vel.length() > 0.12;
        if sprint { w.stamina = s.stamina.saturating_sub(1); }
        else { w.stamina = (s.stamina + 1).min(40); }
        return w;
    }
    w.stamina = (s.stamina + 1).min(40);
    if s.blood && s.night {
        let (x, z, yaw) = toward(s, s.player, 0.16);
        w.x = x; w.z = z; w.yaw = yaw; w.state = AGGRO;
        if s.dist < 1.8 { w.hit = 2; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.08);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn ridden(stamina: u8, vel: Vec3) -> Sight {
        Sight {
            tick: 0,
            night: false,
            blood: false,
            pos: Vec3::ZERO,
            vel,
            yaw: 0.0,
            hp: 10,
            state: CALM,
            tame: 0,
            stamina,
            variant: 0,
            timer: 0,
            player: Vec3::ZERO,
            dist: 0.0,
            herd: false,
            item: None,
            water: false,
            lava: false,
            grass: false,
            flower: false,
            ridden: true,
        }
    }

    #[test]
    fn ridden_sprint_drains_idle_refills() {
        let drain = think(&ridden(20, Vec3::new(0.2, 0.0, 0.0)));
        assert_eq!(drain.stamina, 19);

        let fill = think(&ridden(20, Vec3::ZERO));
        assert_eq!(fill.stamina, 21);
    }
}
