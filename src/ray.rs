use ultraviolet::vec::Vec3;

// A ray with an origin and direction vector.
#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn zero() -> Self {
        Self {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn mag(self) -> f32 {
        self.direction.mag()
    }

    pub fn point_at_parameter(self, parameter: f32) -> Vec3 {
        self.origin + self.direction * parameter
    }
}