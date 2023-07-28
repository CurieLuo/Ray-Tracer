pub mod ray;
pub mod vec3;

pub use ray::*;
pub use std::{
    f64::{
        consts::{PI, TAU},
        INFINITY, NEG_INFINITY,
    },
    sync::Arc,
};
pub use vec3::*;

pub const TIME0: f64 = 0.;
pub const TIME1: f64 = 1.;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.clamp(min, max)
}

pub fn random() -> f64 {
    rand::random::<f64>()
}
pub fn randrange(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
// random int in [min,max)
pub fn randint(min: i32, max: i32) -> i32 {
    randrange(min as f64, max as f64) as i32
    // rand::thread_rng().gen_range(min..max)
}
