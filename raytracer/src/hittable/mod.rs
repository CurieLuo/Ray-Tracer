pub use crate::bvh::aabb::*;
use crate::{material::Material, utility::*};
pub mod aarect;
pub mod hittable_list;
pub mod medium;
pub mod rect_box;
pub mod sphere;
pub mod transform;
pub mod triangle;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub u: f64,
    pub v: f64,
    // (u,v) is the relative coordinate on the surface
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Point3, mat_ptr: &'a dyn Material, u: f64, v: f64) -> Self {
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
        self.front_face = dot(r.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f64 {
        0.
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
    //pdf_value & random are for light sources
}
