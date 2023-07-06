use crate::utility::*;

#[derive(Clone, Copy, Default)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov /* vertical field-of-view in degrees */: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0.0, viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
