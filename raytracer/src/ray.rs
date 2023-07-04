use crate::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    // pub fn origin(&self) -> Vec3 {
    //     self.a
    // }
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    // pub fn at(&self, t: f64) -> Vec3 {
    //     self.a + self.b * t
    // }
}
