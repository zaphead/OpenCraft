use engine_core::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn from_pos_size(pos: Vec3, size: Vec3) -> Self {
        Self {
            min: pos,
            max: pos + size,
        }
    }

    pub fn expand(self, delta: Vec3) -> Self {
        let mut min = self.min;
        let mut max = self.max;
        if delta.x < 0.0 {
            min.x += delta.x;
        } else {
            max.x += delta.x;
        }
        if delta.y < 0.0 {
            min.y += delta.y;
        } else {
            max.y += delta.y;
        }
        if delta.z < 0.0 {
            min.z += delta.z;
        } else {
            max.z += delta.z;
        }
        Self { min, max }
    }

    pub fn grow(self, s: f32) -> Self {
        Self {
            min: self.min - Vec3::splat(s),
            max: self.max + Vec3::splat(s),
        }
    }

    pub fn translate(self, d: Vec3) -> Self {
        Self {
            min: self.min + d,
            max: self.max + d,
        }
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn clip_y(self, other: Self, mut dy: f32) -> f32 {
        if self.max.x <= other.min.x || self.min.x >= other.max.x || self.max.z <= other.min.z || self.min.z >= other.max.z
        {
            return dy;
        }
        if dy > 0.0 && self.max.y <= other.min.y {
            let gap = other.min.y - self.max.y;
            if gap < dy {
                dy = gap;
            }
        } else if dy < 0.0 && self.min.y >= other.max.y {
            let gap = other.max.y - self.min.y;
            if gap > dy {
                dy = gap;
            }
        }
        dy
    }

    pub fn clip_x(self, other: Self, mut dx: f32) -> f32 {
        if self.max.y <= other.min.y || self.min.y >= other.max.y || self.max.z <= other.min.z || self.min.z >= other.max.z
        {
            return dx;
        }
        if dx > 0.0 && self.max.x <= other.min.x {
            let gap = other.min.x - self.max.x;
            if gap < dx {
                dx = gap;
            }
        } else if dx < 0.0 && self.min.x >= other.max.x {
            let gap = other.max.x - self.min.x;
            if gap > dx {
                dx = gap;
            }
        }
        dx
    }

    pub fn clip_z(self, other: Self, mut dz: f32) -> f32 {
        if self.max.x <= other.min.x || self.min.x >= other.max.x || self.max.y <= other.min.y || self.min.y >= other.max.y
        {
            return dz;
        }
        if dz > 0.0 && self.max.z <= other.min.z {
            let gap = other.min.z - self.max.z;
            if gap < dz {
                dz = gap;
            }
        } else if dz < 0.0 && self.min.z >= other.max.z {
            let gap = other.max.z - self.min.z;
            if gap > dz {
                dz = gap;
            }
        }
        dz
    }
}
