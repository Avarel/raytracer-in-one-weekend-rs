use crate::ray::Ray;
use crate::vec3::{vec3, Vec3};

pub struct Camera {
    top_left_corner: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    origin: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    _w: Vec3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        // Position of the camera.
        look_from: Vec3<f64>,
        // The position the camera is focusing on.
        look_at: Vec3<f64>,
        // Up reference vector.
        v_up: Vec3<f64>,
        // Vertical
        v_fov: f64,
        // Aspect ratio.
        aspect: f64,
        // Set aperture to 0 to disable depth of field.
        aperture: f64,
        // Focus distance.
        focus_dist: f64,
    ) -> Self {
        let theta = v_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(w).unit();
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

    pub fn get_ray(&self, s: f64, t: f64) -> Ray<f64> {
        let rd = Self::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.top_left_corner + s * self.horizontal - t * self.vertical - self.origin - offset,
        )
    }

    fn random_in_unit_disk() -> Vec3<f64> {
        let theta = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let r = rand::random::<f64>().sqrt();
        vec3(r * theta.cos(), r * theta.sin(), 0.0)
    }
}
