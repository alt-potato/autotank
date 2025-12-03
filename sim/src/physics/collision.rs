use crate::util::math::{Vec2};

/// An axis-aligned bounding box (AABB).
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    /// Creates a new AABB. 
    /// 
    /// Normalizes the min and max vectors so that `min.x <= max.x` and `min.y <= max.y`.
    pub fn new(min: Vec2, max: Vec2) -> Self {
        AABB { 
            min: Vec2::new(min.x.min(max.x), min.y.min(max.y)), 
            max: Vec2::new(min.x.max(max.x), min.y.max(max.y)) 
        }
    }

    /// Creates a new AABB with the given center and size.
    pub fn new_from_size(center: Vec2, size: Vec2) -> Self {
        AABB {
            min: Vec2::new(center.x - size.x / 2.0, center.y - size.y / 2.0),
            max: Vec2::new(center.x + size.x / 2.0, center.y + size.y / 2.0),
        }
    }
}
