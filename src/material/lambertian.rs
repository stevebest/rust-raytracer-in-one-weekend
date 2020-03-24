use super::Material;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Lambertian {
    // TODO: albedo is a spectrum, not a vector.
    pub albedo: Vec3f,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray> {
        let target = hit.p + hit.n + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        *attenuation = self.albedo;
        Some(scattered)
    }
}

/// Unbiased random direction
fn random_in_unit_sphere() -> Vec3f {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3f::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 + Vec3f::new(-1.0, -1.0, -1.0);
        if v.len_squared() <= 1.0 {
            return v;
        }
    }
}
