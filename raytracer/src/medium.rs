use crate::{aabb::*, hittable::*, material::*, texture::*, utility::*};

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_color(boundary: Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1. / d,
            phase_function: Arc::new(Isotropic::new_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebugtrue.
        let rec1 = self.boundary.hit(r, NEG_INFINITY, INFINITY);
        rec1.as_ref()?;
        let mut rec1 = rec1.unwrap();
        let rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        rec2.as_ref()?;
        let mut rec2 = rec2.unwrap();
        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);
        rec1.t = rec1.t.max(0.);
        if rec1.t >= rec2.t {
            return None;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        rec1.t += hit_distance / ray_length;
        rec1.p = r.at(rec1.t);

        rec1.mat_ptr = self.phase_function.clone();

        Some(rec1)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic { albedo }
    }

    pub fn new_color(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
