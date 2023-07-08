use crate::utility::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
} //axis-aligned bounding box

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn mn(&self) -> Point3 {
        self.minimum
    }
    pub fn mx(&self) -> Point3 {
        self.maximum
    }
}

impl Aabb {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_da = 1. / r.direction().get(a);
            let t0 = (self.minimum.get(a) - r.origin().get(a)) * inv_da;
            let t1 = (self.maximum.get(a) - r.origin().get(a)) * inv_da;
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
        f64::min(box0.mn().x, box1.mn().x),
        f64::min(box0.mn().y, box1.mn().y),
        f64::min(box0.mn().z, box1.mn().z),
    );
    let big = Point3::new(
        f64::max(box0.mx().x, box1.mx().x),
        f64::max(box0.mx().y, box1.mx().y),
        f64::max(box0.mx().z, box1.mx().z),
    );
    Aabb::new(small, big)
}
