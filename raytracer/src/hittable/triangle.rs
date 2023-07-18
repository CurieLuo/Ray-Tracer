use crate::hittable::*;

#[derive(Clone)]
pub struct Triangle<M: Material> {
    pub a: Point3,
    pub n: Vec3,
    pub pb: Vec3,
    pub pc: Vec3,
    // pb / pc: perpendicular to ab / ac
    pub mat: M,
    pub bbox: Aabb,
    pub uva: Vec3,
    pub uvab: Vec3,
    pub uvac: Vec3,
}
impl<M: Material> Triangle<M> {
    pub fn new(
        a: Point3,
        b: Point3,
        c: Point3,
        mat: M,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
    ) -> Self {
        let ab = b - a;
        let ac = c - a;
        let normal_ = cross(ab, ac);
        let n = normal_.unit();
        let det = normal_.length();
        let mut min = Point3::default();
        let mut max = Point3::default();
        for i in 0..3 {
            *min.at(i) = f64::min(f64::min(a.get(i), b.get(i)), c.get(i)) - 0.0001;
            *max.at(i) = f64::max(f64::max(a.get(i), b.get(i)), c.get(i)) + 0.0001;
        }
        let uva = Vec3::new(ua, va, 0.);
        let uvab = Vec3::new(ub, vb, 0.) - uva;
        let uvac = Vec3::new(uc, vc, 0.) - uva;

        Self {
            a,
            n,
            pb: cross(n, ab) / det,
            pc: cross(ac, n) / det,
            mat,
            bbox: Aabb::new(min, max),
            uva,
            uvab,
            uvac,
        }
    }
}
impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = dot(self.a - r.origin, self.n) / dot(r.direction, self.n);
        if t < t_min || t_max < t {
            return None;
        }
        let p = r.at(t);
        let ap = p - self.a;
        let u = dot(ap, self.pc);
        let v = dot(ap, self.pb);
        // P = A + uAB + vAC
        if u >= 0. && v >= 0. && u + v <= 1. {
            let uv = self.uva + u * self.uvab + v * self.uvac;
            let mut rec = HitRecord::new(t, p, &self.mat, uv.x, uv.y);
            rec.set_face_normal(r, self.n);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
