use crate::prelude::*;

use crate::geo::*;
use crate::hit::*;
use crate::material::Material;
// use crate::shape::Shape;

pub struct Triangle<'a> {
    pub positions: [Point3f; 3],
    pub material: &'a dyn Material,
}

impl Hit for Triangle<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitStruct<'_>> {
        if let Some(intersection) = self.intersection(ray) {
            let Intersection { p, t, n, .. } = intersection;
            if t > t_min && t < t_max {
                Some(HitStruct::new(t, p, ray, n, self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Triangle<'_> {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        moller_trumbore(ray, self)
    }
}

struct Intersection {
    /// Point of intersection.
    p: Point3f,
    /// Normal at the point of intersection.
    n: Vec3f,
    /// Time of intersection.
    t: Float,
    /// UV coordinates.
    uv: (Float, Float),
}

fn moller_trumbore(ray: &Ray, triangle: &Triangle) -> Option<Intersection> {
    let [v0, v1, v2] = triangle.positions;
    let e1 = v1 - v0;
    let e2 = v2 - v0;

    let h = ray.direction().cross(&e2);

    let a = e1.dot(h);
    if a > -EPSILON && a < EPSILON {
        // ray is parallel to triangle
        return None;
    }

    let f = 1.0 / a;

    let s = ray.origin() - v0;
    let u = f * s.dot(h);
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.cross(&e1);
    let v = f * ray.direction().dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * e2.dot(q);
    if t > EPSILON {
        // let p = Some(ray.origin() + ray.direction() * t)
        let p = v0 + (e1 * u + e2 * v);
        let n = e1.cross(&e2).normalized();
        Some(Intersection {
            p,
            t,
            n,
            uv: (u, v),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::material::null::NullMaterial;

    #[test]
    fn triangle_ray_intersection() {
        let t = Triangle {
            positions: [
                point3(0.0, 0.0, 0.0),
                point3(1.0, 0.0, 0.0),
                point3(0.0, 1.0, 0.0),
            ],
            material: &NullMaterial,
        };

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 2.0, -3.0));
        let i = t.intersection(&r);
        assert!(t.intersection(&r).is_some());
        let i = i.unwrap();
        assert!((i.p - point3(1.0 / 3.0, 2.0 / 3.0, 0.0)).len() < EPSILON);
        assert_eq!(i.n, vec3(0.0, 0.0, 1.0));
        assert!(r.direction().dot(i.n) < 0.0);

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, 3.0));
        assert!(t.intersection(&r).is_none());

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, 0.0));
        assert!(t.intersection(&r).is_none());

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, -1.0));
        assert!(t.intersection(&r).is_none());
    }
}
