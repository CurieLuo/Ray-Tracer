#![allow(dead_code, unused_imports)]
use ndarray::{array, Array2};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::utility::*;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    ///! bug: divide by 0
    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn get(&self, i: i32) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
    pub fn at(&mut self, i: i32) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }

    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }
    pub fn randrange(min: f64, max: f64) -> Self {
        Self::new(
            randrange(min, max),
            randrange(min, max),
            randrange(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn to_tuple(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
    pub fn from_array<T: Index<usize, Output = f64>>(arr: &T) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-dot(uv, n)).min(1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1. - r_out_perp.length_squared()).abs().sqrt()) * n;
    (r_out_perp + r_out_parallel).unit()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::randrange(-1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = randrange(0., TAU);
    let z = randrange(-1., 1.);
    let r = (1. - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
    //method 2: Normal Distribution
    //method 3: random_in_unit_sphere().unit()
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random();
    let r2 = random();
    let z = (1. - r2).sqrt();
    let phi = TAU * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random();
    let r2 = random();
    let z = 1. + r2 * ((1. - radius * radius / distance_squared).sqrt() - 1.);
    let phi = TAU * r1;
    let x = phi.cos() * (1. - z * z).sqrt();
    let y = phi.sin() * (1. - z * z).sqrt();
    Vec3::new(x, y, z)
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(randrange(-1., 1.), randrange(-1., 1.), 0.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl Mul<Vec3> for Array2<f64> {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from_array(&self.dot(&array![rhs[0], rhs[1], rhs[2]]))
    }
}
pub fn matmul(lhs: &Array2<f64>, rhs: Vec3) -> Vec3 {
    Vec3::from_array(&lhs.dot(&array![rhs[0], rhs[1], rhs[2]]))
}
