use crate::utility::*;

#[derive(Clone, Copy, Default)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
} //axis-aligned bounding box

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }
}

impl Aabb {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_da = 1. / r.direction.get(a);
            let t0 = (self.min.get(a) - r.origin.get(a)) * inv_da;
            let t1 = (self.max.get(a) - r.origin.get(a)) * inv_da;
            let (t0, t1) = if inv_da < 0. { (t1, t0) } else { (t0, t1) };
            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Point3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z),
    );
    let big = Point3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z),
    );
    Aabb::new(small, big)
}
