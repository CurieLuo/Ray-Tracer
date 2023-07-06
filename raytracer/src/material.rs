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
            fuzz: if f < 1. { f } else { 1. },
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
