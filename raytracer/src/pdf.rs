use crate::{hittable::Hittable, onb::*, utility::*};

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
} // probability density function

#[derive(Clone, Copy)]
pub struct CosinePdf {
    pub uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = dot(direction, self.uvw.w);
        cosine.max(0.) / PI
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}

#[derive(Clone, Copy)]
pub struct HittablePdf<'a> {
    pub o: Point3,
    pub ptr: &'a dyn Hittable,
}
impl<'a> HittablePdf<'a> {
    pub fn _new(ptr: &'a dyn Hittable, o: Point3) -> Self {
        Self { o, ptr }
    }
}
impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

#[derive(Clone, Copy)]
pub struct MixturePdf<'a> {
    pub p0: &'a dyn Pdf,
    pub p1: &'a dyn Pdf,
    pub wt0: f64,
}
impl<'a> MixturePdf<'a> {
    pub fn _new(p0: &'a dyn Pdf, p1: &'a dyn Pdf, wt0: f64) -> Self {
        Self { p0, p1, wt0 }
    }
}
impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        self.wt0 * self.p0.value(direction) + (1. - self.wt0) * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random() < self.wt0 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
