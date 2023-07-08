use crate::{aabb::*, material::Material, utility::*};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, mat_ptr: Arc<dyn Material>, u: f64, v: f64) -> Self {
        Self {
            t,
            p,
            normal: Vec3::default(),
            mat_ptr,
            u,
            v,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
