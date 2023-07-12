use crate::{hittable::HitRecord, texture::*, utility::*};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    // pub fn new_texture(albedo: Arc<dyn Texture>) -> Self {
    //     Self { albedo }
    // }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let scatter_direction = random_in_hemisphere(rec.normal);
        *scattered = Ray::new(rec.p, scatter_direction.unit(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        *pdf = 0.5 / PI;

        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, scattered.direction());
        if cosine < 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
    pub fn new_color(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

// #[derive(Clone)]
// pub struct Metal {
//     pub albedo: Color,
//     pub fuzz: f64,
// }
// impl Metal {
//     pub fn new(albedo: Color, f: f64) -> Self {
//         Metal {
//             albedo,
//             fuzz: f.min(1.),
//         }
//     }
// }
// impl Material for Metal {
//     fn scatter(
//         &self,
//         r_in: &Ray,
//         rec: &HitRecord,
//         attenuation: &mut Color,
//         scattered: &mut Ray,
//     ) -> bool {
//         let reflected = reflect(r_in.direction().unit(), rec.normal);
//         *scattered = Ray::new(
//             rec.p,
//             reflected + self.fuzz * random_in_unit_sphere(),
//             r_in.time(),
//         );
//         *attenuation = self.albedo;

//         dot(scattered.direction(), rec.normal) > 0.
//     }
// }

// #[derive(Clone)]
// pub struct Dielectric {
//     ir: f64, // Index of Refraction
// }
// impl Dielectric {
//     pub fn new(ir: f64) -> Self {
//         Self { ir }
//     }
//     fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
//         // Use Schlick's approximation for reflectance.
//         let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
//         r0 + (1. - r0) * (1. - cosine).powi(5)
//     }
// }
// impl Material for Dielectric {
//     fn scatter(
//         &self,
//         r_in: &Ray,
//         rec: &HitRecord,
//         attenuation: &mut Color,
//         scattered: &mut Ray,
//     ) -> bool {
//         *attenuation = Color::new(1., 1., 1.);
//         let refraction_ratio = if rec.front_face {
//             1. / self.ir
//         } else {
//             self.ir
//         };
//         let unit_direction = r_in.direction().unit();
//         let cos_theta = dot(-unit_direction, rec.normal).min(1.);
//         let sin_theta = (1. - cos_theta * cos_theta).sqrt();
//         let cannot_refract = refraction_ratio * sin_theta > 1.;
//         let direction =
//             if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
//                 reflect(unit_direction, rec.normal)
//             } else {
//                 refract(unit_direction, rec.normal, refraction_ratio)
//             };
//         *scattered = Ray::new(rec.p, direction, r_in.time());
//         true
//     }
// }
