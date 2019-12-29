use crate::vec3::Vec3;

// A ray with an origin and direction vector.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const ZERO: Ray = Ray {
        origin: Vec3::ZERO,
        direction: Vec3::ZERO,
    };

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at_parameter(self, parameter: f64) -> Vec3 {
        self.origin + self.direction * parameter
    }
}