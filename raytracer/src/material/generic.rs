#![allow(dead_code, unused_imports)]
use crate::{hittable::*, material::*, pdf::onb::*, pdf::CosPDF, texture::*, utility::*};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let (u, v, p, normal) = (rec.u, rec.v, rec.p, rec.normal);
        if random() >= self.alpha {
            let refraction_ratio = if rec.front_face {
                1. / self.optical_density
            } else {
                self.optical_density
            };
            let cos_theta = dot(&-r_in.direction().unit(), &normal).min(1.);
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            let cannot_refract = refraction_ratio * sin_theta > 1.;
            let is_reflected =
                cannot_refract || random() < Self::reflectance(cos_theta, refraction_ratio);
            return if !is_reflected {
                let scatter_direction = refract(r_in.direction_borrow(), &normal, refraction_ratio);
                srec.scattered = Ray::new(&p, &scatter_direction, r_in.time());
                srec.attenuation = Color::grayscale(1.);
                // srec.pdf_ptr = None;
                true
            } else {
                let scatter_direction = reflect(r_in.direction_borrow(), &normal);
                srec.scattered = Ray::new(&p, &scatter_direction.unit(), r_in.time());
                srec.attenuation = self.specular.value(u, v, &p);
                // srec.pdf_ptr = None;
                true
            };
        } // refract
        let pdf_ptr = Box::new(CosPDF::new(&rec.normal));
        let rough = self.rough.value(u, v, &p)[0];
        if random() < rough {
            srec.attenuation = self.diffuse.value(u, v, &p);
            srec.pdf_ptr = Some(pdf_ptr);
            return true;
        }
        let reflected = reflect(r_in.direction_borrow(), &rec.normal);
        srec.scattered = Ray::new(&rec.p, &reflected.unit(), r_in.time());
        srec.attenuation = self.specular.value(u, v, &p);
        // srec.pdf_ptr = None;
        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &scattered.direction_borrow().unit());
        cosine.max(0.) / PI
    }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        // TODO need front_face?
        // self.emit.value(u, v, p)
        // const BRIGHTNESS: f64 = 5.;
        if rec.front_face {
            self.emit.value(u, v, p) //* BRIGHTNESS
        } else {
            Color::default()
        }
    }
}
