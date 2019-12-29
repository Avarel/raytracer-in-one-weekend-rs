use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

// If this is returned, then it means that the ray of light hit the object
// for some parameter at a point. The normal and material of the object
// is also returned.
pub struct Hit<'mat, T> {
    pub parameter: T,
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub material: &'mat Material,
}

// 3D model enumeration to avoid dynamic dispatch.
#[non_exhaustive]
pub enum Model<'mat> {
    Sphere(Sphere<'mat>),
    List(Vec<Model<'mat>>),
}

impl<'mat> Model<'mat> {
    // Convenience method to construct a sphere.
    pub fn sphere(center: Vec3<f64>, radius: f64, material: &'mat Material) -> Self {
        Model::Sphere(Sphere::new(center, radius, material))
    }

    // Convenience method to construct a list of models.
    pub fn list(vec: Vec<Model<'mat>>) -> Self {
        Model::List(vec)
    }

    // Test if the ray of light hits the object(s) within a certain parameter range.
    pub fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<Hit<f64>> {
        match self {
            Model::Sphere(s) => s.hit(ray, t_min, t_max),
            Model::List(list) => {
                let mut hit_record = None;

                let mut closest_so_far = t_max;

                for hittable in list {
                    if let Some(hit) = hittable.hit(ray, t_min, closest_so_far) {
                        closest_so_far = hit.parameter;
                        hit_record = Some(hit);
                    }
                }

                hit_record
            }
        }
    }
}

// A very round boy.
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

    pub fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<Hit<'mat, f64>> {
        // Quadratic formula this boy.
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let parameter = (-b - discriminant.sqrt()) / a;

            if t_min < parameter && parameter < t_max {
                let point = ray.point_at_parameter(parameter);

                return Some(Hit {
                    parameter,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: &self.material
                })
            }

            let parameter = (-b + discriminant.sqrt()) / a;

            if t_min < parameter && parameter < t_max {
                let point = ray.point_at_parameter(parameter);

                return Some(Hit {
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