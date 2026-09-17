use engine_core::{BlockPos, Vec3};

pub struct RayHit {
    pub pos: BlockPos,
    pub before: BlockPos,
    pub dist: f32,
    pub face: u8,
}

/// DDA voxel walk. `face`: 0=-x 1=+x 2=-y 3=+y 4=-z 5=+z.
pub fn hit_voxel(
    origin: Vec3,
    dir: Vec3,
    max_dist: f32,
    solid: impl Fn(BlockPos) -> bool,
) -> Option<RayHit> {
    let dir = {
        let l = dir.length();
        if l < 1e-8 {
            return None;
        }
        dir / l
    };
    let mut x = origin.x.floor() as i32;
    let mut y = origin.y.floor() as i32;
    let mut z = origin.z.floor() as i32;
    let step_x = if dir.x < 0.0 { -1 } else { 1 };
    let step_y = if dir.y < 0.0 { -1 } else { 1 };
    let step_z = if dir.z < 0.0 { -1 } else { 1 };
    let t_delta_x = if dir.x.abs() < 1e-8 { f32::INFINITY } else { (1.0 / dir.x).abs() };
    let t_delta_y = if dir.y.abs() < 1e-8 { f32::INFINITY } else { (1.0 / dir.y).abs() };
    let t_delta_z = if dir.z.abs() < 1e-8 { f32::INFINITY } else { (1.0 / dir.z).abs() };
    let mut t_max_x = intbound(origin.x, dir.x);
    let mut t_max_y = intbound(origin.y, dir.y);
    let mut t_max_z = intbound(origin.z, dir.z);
    let mut dist = 0.0;
    let mut face = 0u8;
    let mut prev = BlockPos::new(x, y, z);
    for _ in 0..256 {
        let pos = BlockPos::new(x, y, z);
        if solid(pos) {
            return Some(RayHit {
                pos,
                before: prev,
                dist,
                face,
            });
        }
        prev = pos;
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                dist = t_max_x;
                if dist > max_dist {
                    return None;
                }
                x += step_x;
                t_max_x += t_delta_x;
                face = if step_x > 0 { 0 } else { 1 };
            } else {
                dist = t_max_z;
                if dist > max_dist {
                    return None;
                }
                z += step_z;
                t_max_z += t_delta_z;
                face = if step_z > 0 { 4 } else { 5 };
            }
        } else if t_max_y < t_max_z {
            dist = t_max_y;
            if dist > max_dist {
                return None;
            }
            y += step_y;
            t_max_y += t_delta_y;
            face = if step_y > 0 { 2 } else { 3 };
        } else {
            dist = t_max_z;
            if dist > max_dist {
                return None;
            }
            z += step_z;
            t_max_z += t_delta_z;
            face = if step_z > 0 { 4 } else { 5 };
        }
    }
    None
}

fn intbound(s: f32, ds: f32) -> f32 {
    if ds.abs() < 1e-8 {
        return f32::INFINITY;
    }
    if ds > 0.0 {
        ((s.floor() + 1.0) - s) / ds
    } else {
        (s - s.floor()) / -ds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_block_ahead() {
        let hit = hit_voxel(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), 8.0, |p| {
            p.z == 3
        });
        let hit = hit.expect("DDA must hit the solid voxel at z=3");
        assert_eq!(hit.pos.z, 3);
        assert_eq!(hit.before.z, 2);
    }
}
