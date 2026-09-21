use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    if !(s.night || s.blood) {
        w.state = CALM;
        return w;
    }
    if let Some(item) = s.item {
        let (x, z, yaw) = toward(s, item, 0.12);
        w.x = x; w.z = z; w.yaw = yaw;
        if s.pos.distance(item) < 1.1 { w.take_item = true; }
        return w;
    }
    let (x, z, yaw) = wander(s, 0.08);
    w.x = x; w.z = z; w.yaw = yaw;
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec3;

    fn sight(night: bool, item: Option<Vec3>) -> Sight {
        Sight {
            tick: 0,
            night,
            blood: false,
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            yaw: 0.0,
            hp: 8,
            state: CALM,
            tame: 0,
            stamina: 0,
            variant: 0,
            timer: 0,
            player: Vec3::ZERO,
            dist: 10.0,
            herd: false,
            item,
            water: false,
            lava: false,
            grass: false,
            flower: false,
            ridden: false,
        }
    }

    #[test]
    fn day_holds_night_takes_nearby_item() {
        let day = think(&sight(false, Some(Vec3::new(0.5, 0.0, 0.0))));
        assert_eq!(day.state, CALM);
        assert_eq!(day.x, 0.0);
        assert_eq!(day.z, 0.0);
        assert!(!day.take_item);

        let night = think(&sight(true, Some(Vec3::new(0.5, 0.0, 0.0))));
        assert!(night.take_item);
    }
}
