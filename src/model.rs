use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct Hit<'mat, T> {
    pub parameter: T,
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub material: &'mat Material,
}

#[non_exhaustive]
pub enum Model<'mat> {
    Sphere(Sphere<'mat>),
    List(Vec<Model<'mat>>),
}

impl<'mat> Model<'mat> {
    pub fn sphere(center: Vec3<f64>, radius: f64, material: &'mat Material) -> Self {
        Model::Sphere(Sphere::new(center, radius, material))
    }

    pub fn list(vec: Vec<Model<'mat>>) -> Self {
        Model::List(vec)
    }

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
