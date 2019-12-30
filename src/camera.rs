use crate::ray::Ray;
use ultraviolet::vec::Vec3;

pub struct Camera {
    top_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        // Position of the camera.
        look_from: Vec3,
        // The position the camera is focusing on.
        look_at: Vec3,
        // Up reference vector.
        v_up: Vec3,
        // Vertical
        v_fov: f32,
        // Aspect ratio.
        aspect: f32,
        // Set aperture to 0 to disable depth of field.
        aperture: f32,
        // Focus distance.
        focus_dist: f32,
    ) -> Self {
        let theta = v_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalized();
        let u = v_up.cross(w).normalized();
        let v = w.cross(u);

        Self {
            lens_radius: aperture / 2.0,
            origin: look_from,
            _w: w,
            u,
            v,
            top_left_corner: look_from - u * half_width * focus_dist + v * half_height * focus_dist
                - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Self::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.top_left_corner + s * self.horizontal - t * self.vertical - self.origin - offset,
        )
    }

    fn random_in_unit_disk() -> Vec3 {
        let theta = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let r = rand::random::<f32>().sqrt();
        Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
    }
}
