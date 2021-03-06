use super::point3::Point3f;
use super::vec3::Vec3f;
use crate::num_traits::Float;

pub struct Ray {
    /// Origin of a ray.
    o: Point3f,
    /// Direction of a ray travel.
    d: Vec3f,
}

impl Ray {
    /// Creates a new ray from origin point `o` and normalized direction
    /// vector `d`.
    ///
    /// ```
    /// use pbrt::geo::{Point3f, Vec3f, Ray};
    ///
    /// let o = Point3f::default();
    /// let d = Vec3f::new(1.0, 0.0, 0.0);
    /// let r = Ray::new(o, d);
    ///
    /// assert_eq!(r.origin(), o);
    /// assert_eq!(r.direction(), d);
    /// ```
    pub fn new(o: Point3f, d: Vec3f) -> Ray {
        Ray { o, d }
    }

    /// Creates a new ray from origin point `o` and a direction vector `d`,
    /// which is not guaranteed to be normalized.
    pub fn new_unnormalized(o: Point3f, d: Vec3f) -> Ray {
        Ray::new(o, d.normalized())
    }

    pub fn origin(&self) -> Point3f {
        self.o
    }

    pub fn direction(&self) -> Vec3f {
        self.d
    }

    pub fn origin_and_direction(&self) -> (Point3f, Vec3f) {
        (self.origin(), self.direction())
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
