use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, vec3};

#[derive(Debug)]
#[non_exhaustive]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vec3<f64>) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    pub fn metal(albedo: Vec3<f64>, fuzz: f64) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(ref_idx: f64) -> Self {
        Self::Dielectric(Dielectric::new(ref_idx))
    }

    // (scattered ray, attenuation direction)
    pub fn scatter(
        &self,
        r_in: Ray<f64>,
        rec: HitRecord<'_, f64>,
    ) -> Option<(Ray<f64>, Vec3<f64>)> {
        match self {
            Material::Lambertian(mat) => mat.scatter(r_in, rec),
            Material::Metal(mat) => mat.scatter(r_in, rec),
            Material::Dielectric(mat) => mat.scatter(r_in, rec),
        }
    }
}

fn random_in_unit_sphere() -> Vec3<f64> {
    let mut position: Vec3<f64>;

    position = vec3(rand::random(), rand::random(), rand::random()) * 2.0 - vec3(1.0, 1.0, 1.0);
    while position.squared_length() >= 1.0 {
        position = vec3(rand::random(), rand::random(), rand::random()) * 2.0 - vec3(1.0, 1.0, 1.0);
    }

    position
}

fn reflect(v: Vec3<f64>, normal: Vec3<f64>) -> Vec3<f64> {
    v - v.dot(normal) * normal * 2.0
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vec3<f64>) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, _: Ray<f64>, rec: HitRecord<'_, f64>) -> Option<(Ray<f64>, Vec3<f64>)> {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.point, target - rec.point);
        Some((scattered, self.albedo))
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3<f64>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn scatter(&self, r_in: Ray<f64>, rec: HitRecord<'_, f64>) -> Option<(Ray<f64>, Vec3<f64>)> {
        let target = reflect(r_in.direction.unit(), rec.normal);
        let scattered = Ray::new(rec.point, target + random_in_unit_sphere() * self.fuzz);
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    // Refraction index
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    pub fn scatter(&self, r_in: Ray<f64>, rec: HitRecord<'_, f64>) -> Option<(Ray<f64>, Vec3<f64>)> {
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            let _cosine = r_in.direction.dot(rec.normal) / r_in.direction.length();
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - _cosine * _cosine)).sqrt();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(rec.normal) / r_in.direction.length();
        }

        let refract_result = Self::refract(r_in.direction, outward_normal, ni_over_nt);

        let reflect_probability = if refract_result.is_some() {
            Self::schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        Some((if rand::random::<f64>() < reflect_probability {
            Ray::new(rec.point, reflect(r_in.direction, rec.normal))
        } else {
            Ray::new(rec.point, refract_result.unwrap_or_default())
        }, vec3(1.0, 1.0, 1.0)))
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn refract(v: Vec3<f64>, normal: Vec3<f64>, ni_over_nt: f64) -> Option<Vec3<f64>> {
        let uv = v.unit();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
        } else {
            None
        }
    }
}
