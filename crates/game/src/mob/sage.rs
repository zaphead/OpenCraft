use crate::mind::{away, toward, wander, Sight, Will, AGGRO, ASLEEP, CALM, FLEE, SHUT};

pub fn think(s: &Sight) -> Will {
    let mut w = Will::hold(s);
    let (x, z, yaw) = wander(s, 0.03);
    w.x = x; w.z = z; w.yaw = yaw; w.state = CALM;
    w
}
