use crate::vec3::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64, // see camera.rs
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        // let direction = direction.unit();
        Ray {
            origin,
            direction,
            time,
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
