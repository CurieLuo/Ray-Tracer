pub use super::{ray::*, vec3::*};
// use rand::prelude::*;
pub use std::{
    f64::{consts::PI, INFINITY, NEG_INFINITY},
    sync::Arc,
};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn random() -> f64 {
    rand::random::<f64>()
}
pub fn randrange(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
// // random int in [min,max)
// pub fn randint(min: i32, max: i32) -> i32 {
//     // randrange(min as f64, max as f64) as i32
//     rand::thread_rng().gen_range(min..max)
// }
