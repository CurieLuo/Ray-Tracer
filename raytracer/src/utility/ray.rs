use crate::vec3::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub a: Point3,
    pub b: Vec3,
    pub t: f64,
}

impl Ray {
    pub fn new(a: Point3, b: Vec3, t: f64) -> Self {
        Ray { a, b, t }
    }

    pub fn origin(&self) -> Point3 {
        self.a
    }
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    pub fn time(&self) -> f64 {
        self.t
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.a + self.b * t
    }
}
