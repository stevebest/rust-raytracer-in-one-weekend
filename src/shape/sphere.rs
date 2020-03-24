use crate::prelude::*;

use crate::geo::*;
use crate::hit::*;
use crate::material::*;

pub struct Sphere<'a> {
    pub center: Point3f,
    pub radius: Float,
    pub material: &'a dyn Material,
}

impl Hit for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitStruct> {
        let material = self.material;
        let oc = ray.origin() - self.center;

        let a = ray.direction().len_squared();
        let b = oc.dot(ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.eval(t);
                let n = (p - self.center) * self.radius.recip();
                return Some(HitStruct { t, p, n, material });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.eval(t);
                let n = (p - self.center) * self.radius.recip();
                return Some(HitStruct { t, p, n, material });
            }
        }

        return None;
    }
}
