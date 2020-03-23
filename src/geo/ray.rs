use super::point3::Point3f;
use super::vec3::Vec3f;
use crate::num_traits::Float;

pub struct Ray {
    /// Origin of a ray.
    pub o: Point3f,
    /// Direction of a ray travel.
    pub d: Vec3f,
}

impl Ray {
    /// ```
    /// use pbrt::geo::{Point3f, Vec3f, Ray};
    ///
    /// let o = Point3f::default();
    /// let d = Vec3f::new(1.0, 0.0, 0.0);
    /// let r = Ray::new(o, d);
    ///
    /// assert_eq!(r.o, o);
    /// assert_eq!(r.d, d);
    /// ```
    pub fn new(o: Point3f, d: Vec3f) -> Ray {
        Ray { o, d }
    }

    ///
    /// Calculates a position along the ray at time `t`.
    ///
    /// ```
    /// use pbrt::geo::{Point3f, Vec3f, Ray};
    ///
    /// let o = Point3f::default();
    /// let d = Vec3f::new(1.0, 0.0, 0.0);
    /// let r = Ray::new(o, d);
    ///
    /// assert_eq!(r.eval(5.0), Point3f::new(5.0, 0.0, 0.0));
    /// ```
    pub fn eval(&self, t: Float) -> Point3f {
        self.o + self.d * t
    }
}
