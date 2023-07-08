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
        for (i, x) in p.iter_mut().enumerate().take(Self::POINT_COUNT) {
            *x = i;
        }
        p.shuffle(&mut rand::thread_rng());
        p
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4. * p.x) as i32 & 255;
        let j = (4. * p.y) as i32 & 255;
        let k = (4. * p.z) as i32 & 255;
        self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
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
