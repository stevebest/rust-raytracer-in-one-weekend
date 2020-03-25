use super::Material;

use crate::prelude::*;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Dielectric {}

impl Material for Dielectric {
    fn scatter(&self, _: &Ray, _: &HitStruct<'_>, _: &mut Vec3f) -> Option<Ray> {
        unimplemented!()
    }
}
