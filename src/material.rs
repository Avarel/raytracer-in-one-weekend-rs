use crate::model::Hit;
use crate::ray::Ray;
use ultraviolet::vec::Vec3;

// If this is returned, then it means that the light bounced off the
// material with a certain direction and some attenuation.
pub struct Scatter {
    // The direction of the bounced ray of light.
    pub scattered: Ray,
    // The vector representing the RGB emitted
    // after a bounce on the material.
    pub attenuation: Vec3,
}

impl Scatter {
    pub fn zero() -> Self {
        Self {
            scattered: Ray::zero(),
            attenuation: Vec3::zero(),
        }
    }
}

// Material enum, using this so we can avoid dynamic dispatch.
#[derive(Debug)]
#[non_exhaustive]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Material {
    // Convenience method to construct a lambertian reflectance
    // or matte material.
    pub fn lambertian(albedo: Vec3) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    // Convenience method to construct a reflective or metal material.
    pub fn metal(albedo: Vec3, fuzz: f32) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }

    // Convenience method to construct a dielectric or glass material.
    pub fn dielectric(ref_idx: f32) -> Self {
        Self::Dielectric(Dielectric::new(ref_idx))
    }

    // Convenience method to construct a diffuse light material.
    pub fn diffuse_light(emittance: Vec3) -> Self {
        Self::DiffuseLight(DiffuseLight::new(emittance))
    }

    // Process an incoming ray and return an option indicating if that ray
    // has been scattered or completely absorbed.
    pub fn scatter(&self, r_in: Ray, rec: &Hit) -> Scatter {
        match self {
            Material::Lambertian(mat) => mat.scatter(r_in, rec),
            Material::Metal(mat) => mat.scatter(r_in, rec),
            Material::Dielectric(mat) => mat.scatter(r_in, rec),
            _ => Scatter::zero(),
        }
    }

    // Get what the material emits.
    // This method assumes that the ray has already hit the object with
    // this material.
    pub fn emit(&self, rec: Hit) -> Vec3 {
        match self {
            Material::DiffuseLight(mat) => mat.emit(rec),
            _ => Vec3::zero()
        }
    }
}

// Generate a random point *inside* a unit sphere.
fn random_in_unit_sphere() -> Vec3 {
    let u = rand::random::<f32>();
    let v = rand::random::<f32>();
    let theta = u * 2.0 * std::f32::consts::PI;
    let phi = (2.0 * v - 1.0).acos();
    let r = rand::random::<f32>().cbrt();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let x = r * sin_phi * cos_theta;
    let y = r * sin_phi * sin_theta;
    let z = r * cos_phi;
    Vec3::new(x, y, z)
}

// Return a reflected direction given a normal direction on the object.
fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - v.dot(normal) * normal * 2.0
}

// Lambertian reflective or matte material.
#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, _: Ray, rec: &Hit) -> Scatter {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.point, target - rec.point);
        Scatter {
            scattered,
            attenuation: self.albedo,
        }
    }
}

// Reflective or metal material.
#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn scatter(&self, r_in: Ray, rec: &Hit) -> Scatter {
        let target = reflect(r_in.direction.normalized(), rec.normal);
        let scattered = Ray::new(rec.point, target + random_in_unit_sphere() * self.fuzz);
        if scattered.direction.dot(rec.normal) > 0.0 {
            Scatter {
                scattered,
                attenuation: self.albedo,
            }
        } else {
            Scatter::zero()
        }
    }
}

// Dielectric or glass-like material.
#[derive(Debug)]
pub struct Dielectric {
    // Refraction index
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }

    pub fn scatter(&self, r_in: Ray, rec: &Hit) -> Scatter {
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            let _cosine = r_in.direction.dot(rec.normal) / r_in.direction.mag();
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - _cosine * _cosine)).sqrt();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(rec.normal) / r_in.direction.mag();
        }

        let refract_result = Self::refract(r_in.direction, outward_normal, ni_over_nt);

        let reflect_probability = if refract_result.is_some() {
            Self::schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        Scatter {
            scattered: if rand::random::<f32>() < reflect_probability {
                Ray::new(rec.point, reflect(r_in.direction, rec.normal))
            } else {
                Ray::new(rec.point, refract_result.unwrap_or_default())
            },
            attenuation: Vec3::one(),
        }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn refract(v: Vec3, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = v.normalized();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
        } else {
            None
        }
    }
}

// Diffuse light-emitting material.
#[derive(Debug)]
pub struct DiffuseLight {
    emittance: Vec3,
}

impl DiffuseLight {
    pub fn new(emittance: Vec3) -> Self {
        Self { emittance }
    }

    pub fn emit(&self, _: Hit) -> Vec3 {
        self.emittance
    }
}
