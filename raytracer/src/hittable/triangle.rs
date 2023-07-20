#![allow(dead_code)]
use crate::{hittable::*, texture::*};
use ndarray::array;

#[derive(Clone, Copy)]
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
            min[i] = f64::min(f64::min(a[i], b[i]), c[i]) - 0.0001;
            max[i] = f64::max(f64::max(a[i], b[i]), c[i]) + 0.0001;
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

#[derive(Clone)]
pub struct TriangleWithNormalMapping<M: Material> {
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
    pub na: Vec3,
    pub nab: Vec3,
    pub nac: Vec3,
    //normal information stored in vn items
    pub nmap: NormalTexture,
    pub tangent: Vec3,
}
impl<M: Material> TriangleWithNormalMapping<M> {
    pub fn new(
        (a, b, c): (Point3, Point3, Point3),
        mat: M,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
        (na, nb, nc): (Vec3, Vec3, Vec3),
        nmap: NormalTexture,
    ) -> Self {
        let ab = b - a;
        let ac = c - a;
        let normal_ = cross(ab, ac);
        let n = normal_.unit();
        let det = normal_.length();
        let mut min = Point3::default();
        let mut max = Point3::default();
        for i in 0..3 {
            min[i] = f64::min(f64::min(a[i], b[i]), c[i]) - 0.0001;
            max[i] = f64::max(f64::max(a[i], b[i]), c[i]) + 0.0001;
        }
        let uva = Vec3::new(ua, va, 0.);
        let uvab = Vec3::new(ub, vb, 0.) - uva;
        let uvac = Vec3::new(uc, vc, 0.) - uva;
        let tb = 1. / (uvab.x * uvac.y - uvac.x * uvab.y)
            * array![[uvac.y, -uvab.y], [-uvac.x, uvab.x]]
                .dot(&array![[ab.x, ab.y, ab.z], [ac.x, ac.y, ac.z]]);
        let tangent = &tb.row(0);
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
            na,
            nab: nb - na,
            nac: nc - na,
            nmap,
            tangent: Vec3::from_array(tangent), //Vec3::new(tangent[0], tangent[1], tangent[2]),
        }
    }
}
impl<M: Material> Hittable for TriangleWithNormalMapping<M> {
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
            let uv: Vec3 = self.uva + u * self.uvab + v * self.uvac;
            let mut rec = HitRecord::new(t, p, &self.mat, uv.x, uv.y);
            let normal = (self.na + u * self.nab + v * self.nac).unit();
            let mut tangent = self.tangent;
            tangent = (tangent - dot(tangent, normal) * normal).unit();
            let bitangent = cross(normal, tangent).unit();
            let tbn = array![
                [tangent[0], tangent[1], tangent[2]],
                [bitangent[0], bitangent[1], bitangent[2]],
                [normal[0], normal[1], normal[2]],
            ];
            let normal = self.nmap.value(u, v, p) * 2. - Vec3::new(1., 1., 1.);
            let normal = array![normal[0], normal[1], normal[2]];
            let normal = tbn.dot(&normal);
            let normal = Vec3::from_array(&normal);
            rec.set_face_normal(r, normal);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
