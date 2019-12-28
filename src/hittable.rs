use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord<'mat, T> {
    pub parameter: T,
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub material: &'mat Material,
}

#[non_exhaustive]
pub enum Hittable<'mat> {
    Sphere(Sphere<'mat>),
    List(Vec<Hittable<'mat>>),
}

impl<'mat> Hittable<'mat> {
    pub fn sphere(center: Vec3<f64>, radius: f64, material: &'mat Material) -> Self {
        Hittable::Sphere(Sphere::new(center, radius, material))
    }

    pub fn list(vec: Vec<Hittable<'mat>>) -> Self {
        Hittable::List(vec)
    }

    pub fn hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        match self {
            Hittable::Sphere(s) => s.hit(ray, t_min, t_max),
            Hittable::List(list) => {
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
