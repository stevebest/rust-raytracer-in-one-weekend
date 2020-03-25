use super::Material;

use crate::prelude::*;

use crate::geo::{Ray, Vec3f};
use crate::hit::HitStruct;

pub struct Dielectric {
    pub refraction_index: Float, // TODO: should `refraction_index` be a f64?
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitStruct<'_>, attenuation: &mut Vec3f) -> Option<Ray> {
        *attenuation = Vec3f::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().normalized();
        let cos_theta = rec.n.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let scattered = if etai_over_etat * sin_theta > 1.0 {
            reflect(unit_direction, rec.n)
        } else {
            refract(unit_direction, rec.n, etai_over_etat)
        };
        Some(Ray::new(rec.p, scattered))
    }
}

// TODO: move `reflect` to Vec3
fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - n * v.dot(n) * 2.0
}

// TODO: move `refract` to Vec3
fn refract(uv: Vec3f, n: Vec3f, etai_over_etat: Float) -> Vec3f {
    let cos_theta = -uv.dot(n);
    let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
    let r_out_perp = n * -(1.0 - r_out_parallel.len_squared()).sqrt();
    r_out_parallel + r_out_perp
}
