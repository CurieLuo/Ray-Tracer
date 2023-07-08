use crate::{texture::*, utility::*};
use rand::prelude::SliceRandom;

pub struct Perlin {
    ranvec: Box<[Vec3]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranvec = vec![Vec3::default(); Self::POINT_COUNT];
        for x in &mut ranvec {
            *x = Vec3::randrange(-1., 1.).unit();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec: ranvec.into_boxed_slice(),
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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in &mut cij.iter_mut().enumerate() {
                    *cijk = self.ranvec[self.perm_x[(i + di as i32) as usize & 255]
                        ^ self.perm_y[(j + dj as i32) as usize & 255]
                        ^ self.perm_z[(k + dk as i32) as usize & 255]];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1. - i as f64) * (1. - u))
                        * (j as f64 * vv + (1. - j as f64) * (1. - v))
                        * (k as f64 * ww + (1. - k as f64) * (1. - w))
                        * dot(*cijk, weight_v);
                }
            }
        }

        accum
    }

    pub fn turb(&self, mut p: Point3, depth: i32) -> f64 {
        let mut accum = 0.;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.;
        }

        accum.abs()
    }
    pub fn turb7(&self, p: Point3) -> f64 {
        self.turb(p, 7)
    }
}

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
