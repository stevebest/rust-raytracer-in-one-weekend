use super::Material;

use crate::prelude::*;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Metal {
    // TODO: albedo is a spectrum, not a vector.
    pub albedo: Vec3f,
    pub roughness: Float,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray> {
        let reflected = reflect(ray.direction().normalized(), hit.n);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere() * self.roughness);
        *attenuation = self.albedo;
        if reflected.dot(hit.n) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - n * v.dot(n) * 2.0
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
