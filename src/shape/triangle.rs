use crate::prelude::*;

use crate::geo::*;

use crate::shape::Shape;

pub struct Triangle {
    positions: [Point3f; 3],
}

impl Triangle {
    pub fn intersection(&self, ray: &Ray) -> Option<Point3f> {
        moller_trumbore(ray, self)
    }
}

fn moller_trumbore(ray: &Ray, triangle: &Triangle) -> Option<Point3f> {
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
        // Some(ray.origin() + ray.direction() * t)
        Some(v0 + (e1 * u + e2 * v))
    } else {
        None
    }
}

impl Shape for Triangle {}

#[cfg(test)]
mod test {

    use super::*;
    use crate::geo::*;

    #[test]
    fn triangle_ray_intersection() {
        let t = Triangle {
            positions: [
                point3(1.0, 0.0, 0.0),
                point3(0.0, 1.0, 0.0),
                point3(0.0, 0.0, 0.0),
            ],
        };

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 2.0, -3.0));
        assert!(t.intersection(&r).is_some());
        assert!((t.intersection(&r).unwrap() - point3(1.0 / 3.0, 2.0 / 3.0, 0.0)).len() < EPSILON);

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, 3.0));
        assert!(t.intersection(&r).is_none());

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, 0.0));
        assert!(t.intersection(&r).is_none());

        let r = Ray::new(point3(0.0, 0.0, 1.0), vec3(1.0, 1.0, -1.0));
        assert!(t.intersection(&r).is_none());
    }
}
