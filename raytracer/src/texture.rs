use crate::{perlin::*, utility::*};
use image::GenericImageView;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct CheckerTexture<T0: Texture, T1: Texture> {
    pub even: T0,
    pub odd: T1,
}
impl<T0: Texture, T1: Texture> CheckerTexture<T0, T1> {
    pub fn new(even: T0, odd: T1) -> Self {
        Self { even, odd }
    }
}
impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_color(c0: Color, c1: Color) -> Self {
        Self::new(SolidColor::new(c0), SolidColor::new(c1))
    }
}
impl<T0: Texture, T1: Texture> Texture for CheckerTexture<T0, T1> {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z + 10. * self.noise.turb7(p)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<[u8; 3]>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = image::open(filename).expect("Failed to open image");
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut data = vec![[0u8; 3]; width * height];
        for (i, (_, _, pixel)) in image.pixels().enumerate() {
            for j in 0..3 {
                data[i][j] = pixel[j];
            }
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: Vec3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        u = clamp(u, 0., 1.);
        v = 1. - clamp(v, 0., 1.);
        // Flip V to image coordinates
        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;
        // Clamp integer mapping, since actual coordinates should be less than 1.
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let index = j * self.width + i;
        Color::new(
            self.data[index][0] as f64,
            self.data[index][1] as f64,
            self.data[index][2] as f64,
        ) / 255.
    }
}
