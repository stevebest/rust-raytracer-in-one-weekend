pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub trait Material: std::marker::Sync {
    fn scatter(&self, ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray>;
}
