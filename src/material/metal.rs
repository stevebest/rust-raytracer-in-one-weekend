use super::Material;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Metal {
    // TODO: albedo is a spectrum, not a vector.
    pub albedo: Vec3f,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitStruct, attenuation: &mut Vec3f) -> Option<Ray> {
        let reflected = reflect(ray.direction().normalized(), hit.n);
        let scattered = Ray::new(hit.p, reflected);
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
