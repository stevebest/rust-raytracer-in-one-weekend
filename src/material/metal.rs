use super::Material;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Metal;

impl Material for Metal {
    fn scatter(&self, _ray: &Ray, _hit: &HitStruct, _attenuation: &mut Vec3f) -> Option<Ray> {
        None
    }
}
