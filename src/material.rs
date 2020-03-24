pub mod lambertian;
pub mod metal;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray>;
}