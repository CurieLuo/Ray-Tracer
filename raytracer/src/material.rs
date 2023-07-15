use crate::{hittable::HitRecord, pdf::*, texture::*, utility::*};

#[derive(Clone)]
pub struct ScatterRecord {
    pub scattered: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
}
impl ScatterRecord {
    pub fn new(
        scattered: Ray,
        is_specular: bool,
        attenuation: Color,
        pdf_ptr: Option<Arc<dyn Pdf>>,
    ) -> Self {
        Self {
            scattered,
            is_specular,
            attenuation,
            pdf_ptr,
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }
    fn emitted(&self, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}
impl<T: Texture> Lambertian<T> {
    pub fn new_texture(albedo: T) -> Self {
        Self { albedo }
    }
}
impl Lambertian<SolidColor> {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: SolidColor::new(a),
        }
    }
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf_ptr = Arc::new(CosinePdf::new(rec.normal));

        Some(ScatterRecord::new(
            Ray::default(),
            false,
            attenuation,
            Some(pdf_ptr),
        ))
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, scattered.direction().unit());
        cosine.max(0.) / PI
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}
impl DiffuseLight<SolidColor> {
    pub fn new_color(c: Color) -> Self {
        Self::new(SolidColor::new(c))
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::new(0., 0., 0.)
        }
    }
}

#[derive(Clone)]
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction().unit(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );

        Some(ScatterRecord::new(scattered, true, self.albedo, None))
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit();
        let cos_theta = dot(-unit_direction, rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };
        let scattered = Ray::new(rec.p, direction, r_in.time());
        Some(ScatterRecord::new(
            scattered,
            true,
            Color::new(1., 1., 1.),
            None,
        ))
    }
}
