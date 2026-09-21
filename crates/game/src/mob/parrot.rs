use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    w.fly = true;
    let (x, z, yaw) = wander(s, 0.09);
    w.x = x; w.z = z; w.yaw = yaw;
    if s.dist < 16.0 && s.tick % 45 == 0 { w.call = true; }
    w
}
