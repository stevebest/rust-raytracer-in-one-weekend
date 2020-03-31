use super::Material;

use crate::geo::*;
use crate::hit::*;

pub struct NullMaterial;

impl Material for NullMaterial {
    fn scatter(&self, _ray: &Ray, _hit: &HitStruct<'_>, attenuation: &mut Vec3f) -> Option<Ray> {
        *attenuation = vec3(0.0, 0.0, 0.0);
        Some(Ray::new(point3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)))
    }
}
