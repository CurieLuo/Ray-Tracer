#![allow(dead_code, unused_imports)]
use crate::{material::*, onb::Onb};

#[derive(Clone, Copy)]
pub struct Generic<TDiff: Texture, TSpec: Texture, TEmit: Texture, TRough: Texture> {
    pub diffuse: TDiff,
    pub specular: TSpec,
    pub emit: TEmit,
    pub rough: TRough, //0..1000, specular exponent
    pub alpha: f64,
    pub optical_density: f64,
    //TODO Color/Scalar->Texture; more params
}

impl<TDiff: Texture, TSpec: Texture, TEmit: Texture, TRough: Texture>
    Generic<TDiff, TSpec, TEmit, TRough>
{
    pub fn new(
        diffuse: TDiff,
        specular: TSpec,
        emit: TEmit,
        rough: TRough,
        alpha: f64,
        optical_density: f64,
    ) -> Self {
        Self {
            diffuse,
            specular,
            emit,
            rough,
            alpha,
            optical_density,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl<TDiff: Texture, TSpec: Texture, TEmit: Texture, TRough: Texture> Material
    for Generic<TDiff, TSpec, TEmit, TRough>
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let (u, v, p, normal) = (rec.u, rec.v, rec.p, rec.normal);
        if random() >= self.alpha {
            let refraction_ratio = if rec.front_face {
                1. / self.optical_density
            } else {
                self.optical_density
            };
            let cos_theta = dot(-r_in.direction, normal).min(1.);
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            let cannot_refract = refraction_ratio * sin_theta > 1.;
            let is_reflected =
                cannot_refract || random() < Self::reflectance(cos_theta, refraction_ratio);
            return if !is_reflected {
                let scatter_direction = refract(r_in.direction, normal, refraction_ratio);
                let scattered = Ray::new(p, scatter_direction, r_in.time);
                Some(ScatterRecord::new(scattered, Color::new(1., 1., 1.)))
            } else {
                let scatter_direction = reflect(r_in.direction, normal);
                let scattered = Ray::new(p, scatter_direction, r_in.time);
                Some(ScatterRecord::new(scattered, self.specular.value(u, v, p)))
            };
        } // refract
          //TODO how to distribute between diffuse and specular?
        let rough = self.rough.value(u, v, p)[0];
        if random() < rough {
            let attenuation = self.diffuse.value(u, v, p);
            let mut scatter_direction = normal + random_unit_vector();
            // Catch degenerate scatter direction
            if scatter_direction.near_zero() {
                scatter_direction = normal;
            }
            let scattered = Ray::new(p, scatter_direction.unit(), r_in.time);
            return Some(ScatterRecord::new(scattered, attenuation));
        }
        let reflected = reflect(r_in.direction, rec.normal);
        let scattered = Ray::new(rec.p, reflected.unit(), r_in.time);
        Some(ScatterRecord::new(scattered, self.specular.value(u, v, p)))
    }
    fn _scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<_ScatterRecord> {
        let (u, v, p, normal) = (rec.u, rec.v, rec.p, rec.normal);
        if random() >= self.alpha {
            let refraction_ratio = if rec.front_face {
                1. / self.optical_density
            } else {
                self.optical_density
            };
            let cos_theta = dot(-r_in.direction, normal).min(1.);
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            let cannot_refract = refraction_ratio * sin_theta > 1.;
            let is_reflected =
                cannot_refract || random() < Self::reflectance(cos_theta, refraction_ratio);
            return if !is_reflected {
                let scatter_direction = refract(r_in.direction, normal, refraction_ratio);
                let scattered = Ray::new(p, scatter_direction, r_in.time);
                Some(_ScatterRecord::new(scattered, Color::new(1., 1., 1.), None))
            } else {
                let scatter_direction = reflect(r_in.direction, normal);
                let scattered = Ray::new(p, scatter_direction, r_in.time);
                Some(_ScatterRecord::new(
                    scattered,
                    self.specular.value(u, v, p),
                    None,
                ))
            };
        } // refract
        let pdf_ptr = Box::new(CosinePdf::new(rec.normal));
        let rough = self.rough.value(u, v, p)[0];
        if random() < rough {
            let attenuation = self.diffuse.value(u, v, p);
            return Some(_ScatterRecord::new(
                Ray::default(),
                attenuation,
                Some(pdf_ptr),
            ));
        }
        let reflected = reflect(r_in.direction, rec.normal);
        let scattered = Ray::new(rec.p, reflected.unit(), r_in.time);
        Some(_ScatterRecord::new(
            scattered,
            self.specular.value(u, v, p),
            None,
        ))
    }
    fn _scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, scattered.direction);
        cosine.max(0.) / PI
    }
    fn emitted(&self, rec: &HitRecord) -> Color {
        // TODO need front_face?
        // self.emit.value(rec.u, rec.v, rec.p)
        // const BRIGHTNESS: f64 = 5.;
        if rec.front_face {
            self.emit.value(rec.u, rec.v, rec.p) //* BRIGHTNESS
        } else {
            Color::default()
        }
    }
}
