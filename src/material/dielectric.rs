use super::Material;

use crate::prelude::*;

use crate::geo::*;
use crate::hit::HitStruct;

pub struct Dielectric {
    pub refraction_index: Float, // TODO: should `refraction_index` be a f64?
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitStruct<'_>, attenuation: &mut Vec3f) -> Option<Ray> {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        *attenuation = vec3(1.0, 1.0, 1.0);
        // ray direction, normalized
        let d = r_in.direction().normalized();

        let (etai_over_etat, n) = if rec.front_face {
            (self.refraction_index.recip(), rec.n.normalized())
        } else {
            (self.refraction_index, -rec.n.normalized())
        };

        let cos_theta = Vec3f::dot(-d, n).min(1.0).max(-1.0);

        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            // Total internal reflection
            reflect(d, n)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if rng.gen::<Float>() < reflect_prob {
                reflect(d, n)
            } else {
                refract(d, n, etai_over_etat)
            }
        };

        Some(Ray::new(rec.p, scattered))
    }
}

// FIXME Float -> f64?
fn schlick(cosine: Float, refraction_index: Float) -> Float {
    assert!(cosine >= -1.0, "schlick: cosine < -1.0, = {}", cosine);
    assert!(cosine <= 1.0, "schlick: cosine > 1.0, = {}", cosine);

    // ```cpp
    // auto r0 = (1-ref_idx) / (1+ref_idx);
    // r0 = r0*r0;
    // return r0 + (1-r0)*pow((1 - cosine),5);
    // ```

    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)

    // let c = 1.0 - cosine;
    // let c2 = c * c;
    // let c5 = c2 * c2 * c; // c.powf(5.0)
}

// TODO: move `reflect` to Vec3
fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - n * v.dot(n) * 2.0
}

// TODO: move `refract` to Vec3
fn refract(uv: Vec3f, n: Vec3f, etai_over_etat: Float) -> Vec3f {
    assert!(!uv.has_nans(), "refract: uv has NaNs: {:?}", uv);
    assert!(!n.has_nans(), "refract: uv has NaNs: {:?}", n);

    let cos_theta = -uv.dot(n);
    assert!(
        cos_theta >= -1.0 && cos_theta <= 1.0,
        "refract: cos_theta = {}",
        cos_theta
    );

    let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
    assert!(
        !r_out_parallel.has_nans(),
        "refract: r_out_parallel has NaNs: uv = {:?}, n = {:?}, cos_theta = {}, etai_over_etat = {}, r_out_parallel = {:?}",
        uv,
        n,
        cos_theta,
        etai_over_etat,
        r_out_parallel,
    );

    let r_out_perp = n * -((1.0 - r_out_parallel.len_squared().min(1.0)).sqrt());
    assert!(
        !r_out_perp.has_nans(),
        "refract: r_out_perp = {:?}; r_out_parallel = {:?}, r_out_parallel.len_squared() = {}",
        r_out_perp,
        r_out_parallel,
        r_out_parallel.len_squared(),
    );

    let r_out = r_out_parallel + r_out_perp;

    r_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refract_front() {
        let eta = 1.0;
        let eta_prime = 2.0;

        let u = vec3(1.0, 2.0, -1.0).normalized();
        let n = vec3(0.0, 0.0, 1.0);

        let w = refract(u, n, eta / eta_prime);

        let cos_theta = -u.dot(n);
        let cos_theta_prime = -w.dot(n);

        assert!(
            snells_law((eta, cos_theta), (eta_prime, cos_theta_prime)),
            "Snell's law"
        );
        assert!(w.z < 0.0, "w.z = {}", w.z);

        // Refract out
        let n = vec3(0.0, 0.0, 1.0);

        let w1 = refract(w, n, eta_prime / eta);

        let cos_theta = -w.dot(n);
        let cos_theta_prime = -w1.dot(n);

        assert!(
            snells_law((eta_prime, cos_theta), (eta, cos_theta_prime)),
            "Snell's law"
        );
        // assert!(w.z < 0.0, "w.z = {}", w.z);
        assert!((u.dot(w1) - 1.0).abs() < EPSILON);
    }

    fn snells_law((eta, cos_theta): (f32, f32), (eta_prime, cos_theta_prime): (f32, f32)) -> bool {
        let sin_theta = pyth(cos_theta);
        let sin_theta_prime = pyth(cos_theta_prime);

        (eta * sin_theta - eta_prime * sin_theta_prime).abs() < EPSILON
    }

    fn pyth(x: f32) -> f32 {
        (1.0 - x * x).sqrt()
    }
}
