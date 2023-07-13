use crate::{hittable::Hittable, onb::*, utility::*};

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
} // probability density function

pub struct CosinePdf {
    pub uvw: Onb,
}
// impl CosinePdf {
//     pub fn new(w: Vec3) -> Self {
//         Self { uvw: Onb::new(w) }
//     }
// }
impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = dot(direction.unit(), self.uvw.w);
        (cosine / PI).max(0.)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}

pub struct HittablePdf {
    o: Point3,
    ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn new(ptr: Arc<dyn Hittable>, o: Point3) -> Self {
        Self { o, ptr }
    }
}
impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}
