pub mod dielectric;
pub mod lambertian;
pub mod metal;

/// Null material, useful for replacing missing materials and for unit tests.
pub mod null;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;
pub use null::*;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub trait Material: std::marker::Sync {
    fn scatter(&self, ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray>;
}
