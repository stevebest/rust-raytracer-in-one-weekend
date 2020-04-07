use super::Material;

use crate::geo::*;
use crate::hit::*;

pub struct NullMaterial;

impl Material for NullMaterial {
    fn scatter(&self, _ray: &Ray, _hit: &HitStruct<'_>, _attenuation: &mut Vec3f) -> Option<Ray> {
        None
    }
}
