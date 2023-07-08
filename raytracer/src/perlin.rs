use crate::{texture::*, utility::*};
use rand::prelude::SliceRandom;

pub struct Perlin {
    ranfloat: Box<[f64]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranfloat = vec![0.; Self::POINT_COUNT];
        for x in &mut ranfloat {
            *x = random();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat: ranfloat.into_boxed_slice(),
            perm_x: perm_x.into_boxed_slice(),
            perm_y: perm_y.into_boxed_slice(),
            perm_z: perm_z.into_boxed_slice(),
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; Self::POINT_COUNT];
        for (i, x) in p.iter_mut().enumerate() {
            *x = i;
        }
        p.shuffle(&mut rand::thread_rng());
        p
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        u = u * u * (3. - 2. * u);
        v = v * v * (3. - 2. * v);
        w = w * w * (3. - 2. * w);
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in &mut cij.iter_mut().enumerate() {
                    *cijk = self.ranfloat[self.perm_x[(i + di as i32) as usize & 255]
                        ^ self.perm_y[(j + dj as i32) as usize & 255]
                        ^ self.perm_z[(k + dk as i32) as usize & 255]];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    accum += (i as f64 * u + (1. - i as f64) * (1. - u))
                        * (j as f64 * v + (1. - j as f64) * (1. - v))
                        * (k as f64 * w + (1. - k as f64) * (1. - w))
                        * cijk;
                }
            }
        }

        accum
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1., 1., 1.) * self.noise.noise(p)
    }
}
