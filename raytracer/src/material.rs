use crate::{hittable::HitRecord, utility::*};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Copy, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Metal {
            albedo,
            fuzz: f.min(1.),
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction().unit(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;

        dot(scattered.direction(), rec.normal) > 0.
    }
}

#[derive(Clone, Copy, Default)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let refracted = refract(r_in.direction().unit(), rec.normal, refraction_ratio);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
