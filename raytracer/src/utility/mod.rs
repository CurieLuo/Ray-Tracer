pub use super::{ray::*, vec3::*};
pub use std::f64::{consts::PI, INFINITY};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random() -> f64 {
    rand::random::<f64>()
}

pub fn randrange(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
