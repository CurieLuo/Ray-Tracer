use crate::utility::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}
impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}
impl CheckerTexture {
    // pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
    //     Self { even, odd }
    // }
    pub fn new_color(c0: Color, c1: Color) -> Self {
        Self {
            even: Arc::new(SolidColor::new(c0)),
            odd: Arc::new(SolidColor::new(c1)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
