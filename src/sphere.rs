use crate::material::Material;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere<'mat> {
    center: Vec3<f64>,
    radius: f64,
    material: &'mat Material,
}

impl<'mat> Sphere<'mat> {
    pub fn new(center: Vec3<f64>, radius: f64, material: &'mat Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<'mat, f64>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let parameter = (-b - discriminant.sqrt()) / a;

            if t_min < parameter && parameter < t_max {
                let point = ray.point_at_parameter(parameter);

                return Some(HitRecord {
                    parameter,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material
                })
            }

            let parameter = (-b + discriminant.sqrt()) / a;

            if t_min < parameter && parameter < t_max {
                let point = ray.point_at_parameter(parameter);

                return Some(HitRecord {
                    parameter,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material
                })
            }
        }

        None
    }
}
