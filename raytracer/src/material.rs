use crate::{hittable::HitRecord, utility::*};

pub trait Material {
    fn scatter(r_in: Ray, rec: HitRecord, attenuation: Color3, scattered: &mut Ray) -> bool;
}
