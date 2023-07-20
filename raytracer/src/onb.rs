use crate::utility::*;

#[derive(Copy, Clone, Default)]
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
} // orthonormal basis

impl Onb {
    pub fn local3(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u + b * self.v + c * self.w
    }
    pub fn local(&self, a: Vec3) -> Vec3 {
        self.local3(a.x, a.y, a.z)
    }
    pub fn new(n: Vec3) -> Self {
        let w = n.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        let v = cross(w, a);
        let u = cross(w, v);
        Self { u, v, w }
    }
}
