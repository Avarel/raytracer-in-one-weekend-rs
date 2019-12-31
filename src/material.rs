use crate::model::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// The result of calculations of a ray hitting and bouncing off a
/// material with a certain direction and some attenuation.
///
/// # Note
/// Both `scattered` and `attenuation` can be zero.
pub struct Scatter {
    /// The direction of the bounced ray of light.
    pub scattered: Ray,
    /// The vector representing the RGB emitted
    /// after a bounce on the material.
    pub attenuation: Vec3,
}

impl Scatter {
    pub const ZERO: Scatter = Scatter {
        scattered: Ray::ZERO,
        attenuation: Vec3::ZERO,
    };
}

// Material enum so we can avoid dynamic dispatch.
/// Material enumeration.
#[derive(Debug)]
#[non_exhaustive]
pub enum Material<'mat> {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Combined {
        scatterer: &'mat Material<'mat>,
        emitter: &'mat Material<'mat>,
    },
}

impl Material<'_> {
    /// Convenience method to construct a lambertian reflective
    /// or matte material.
    pub fn lambertian(albedo: Vec3) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    /// Convenience method to construct a reflective or metal material.
    pub fn metal(albedo: Vec3, fuzz: f32) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }

    /// Convenience method to construct a dielectric or glass material.
    pub fn dielectric(ref_idx: f32) -> Self {
        Self::Dielectric(Dielectric::new(ref_idx))
    }

    /// Convenience method to construct a diffuse light material.
    pub fn diffuse_light(emittance: Vec3) -> Self {
        Self::DiffuseLight(DiffuseLight::new(emittance))
    }

    /// Process an incoming ray and return an option indicating if that ray
    /// has been scattered or completely absorbed.
    pub fn scatter(&self, r_in: Ray, rec: &Hit) -> Scatter {
        match self {
            Material::Lambertian(mat) => mat.scatter(r_in, rec),
            Material::Metal(mat) => mat.scatter(r_in, rec),
            Material::Dielectric(mat) => mat.scatter(r_in, rec),
            Material::Combined { scatterer, .. } => scatterer.scatter(r_in, rec),
            _ => Scatter::ZERO,
        }
    }

    /// Get what the material emits.
    ///
    /// # Assumptions
    /// This method assumes that the ray has already hit the object with
    /// this material.
    pub fn emit(&self, rec: Hit) -> Vec3 {
        match self {
            Material::DiffuseLight(mat) => mat.emit(rec),
            Material::Combined { emitter, .. } => emitter.emit(rec),
            _ => Vec3::ZERO,
        }
    }
}

/// Lambertian reflective or matte material.
#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, _: Ray, rec: &Hit) -> Scatter {
        let target = rec.point + rec.normal + rand::random::<Vec3>();
        let scattered = Ray::new(rec.point, target - rec.point);
        Scatter {
            scattered,
            attenuation: self.albedo,
        }
    }
}

/// Reflective or metal material.
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
        let target = r_in
            .direction /*.normalize()*/
            .reflect(rec.normal);
        let scattered = Ray::new(rec.point, target + rand::random::<Vec3>() * self.fuzz);
        if scattered.direction.dot(rec.normal) > 0.0 {
            Scatter {
                scattered,
                attenuation: self.albedo,
            }
        } else {
            Scatter::ZERO
        }
    }
}

/// Dielectric or glass-like material.
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
                Ray::new(rec.point, r_in.direction.reflect(rec.normal))
            } else {
                Ray::new(rec.point, refract_result.unwrap_or_default())
            },
            attenuation: Vec3::ONE,
        }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn refract(v: Vec3, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = v.normalize();
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
