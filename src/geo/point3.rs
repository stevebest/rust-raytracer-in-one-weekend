use crate::geo::vec3::Vec3;
use crate::geo::{max, min};
use crate::num_traits::{Float, Numeric, Zero};

/// A point in 3-dimensional space.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3f = Point3<Float>;

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }

    /// Returns the origin: (0.0, 0.0, 0.0)
    pub fn origin() -> Point3<T>
    where
        T: Zero,
    {
        Point3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Component-wise minimum
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let p1 = point3(1.0, 2.0, -3.0);
    /// let p2 = point3(-1.0, 3.0, 2.0);
    ///
    /// assert_eq!(p1.min(p2), point3(-1.0, 2.0, -3.0));
    /// ```
    pub fn min(&self, other: Point3<T>) -> Point3<T>
    where
        T: Copy + PartialOrd,
    {
        Point3 {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
            z: min(self.z, other.z),
        }
    }

    /// Component-wise maximum
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let p1 = point3(1.0, 2.0, -3.0);
    /// let p2 = point3(-1.0, 3.0, 2.0);
    ///
    /// assert_eq!(p1.max(p2), point3(1.0, 3.0, 2.0));
    /// ```
    pub fn max(&self, other: Point3<T>) -> Point3<T>
    where
        T: Copy + PartialOrd,
    {
        Point3 {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            z: max(self.z, other.z),
        }
    }
}

impl Point3f {
    pub fn lerp(t: Float, p1: Point3f, p2: Point3f) -> Point3f {
        lerp(t, p1, p2)
    }
}

///
/// Point-vector addition.
///
impl<T> std::ops::Add<Vec3<T>> for Point3<T>
where
    T: Numeric<T>,
{
    type Output = Point3<T>;

    fn add(self, v: Vec3<T>) -> Self::Output {
        Point3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

///
/// Point-point subtraction.
///
impl<T> std::ops::Sub for Point3<T>
where
    T: Numeric<T>,
{
    type Output = Vec3<T>;

    fn sub(self, other: Point3<T>) -> Self::Output {
        let (x, y, z) = (self.x - other.x, self.y - other.y, self.z - other.z);
        Vec3::new(x, y, z)
    }
}

///
/// Point scaling.
///
impl<T> std::ops::Mul<T> for Point3<T>
where
    T: Numeric<T>,
{
    type Output = Point3<T>;

    fn mul(self, s: T) -> Self::Output {
        Point3::new(self.x * s, self.y * s, self.z * s)
    }
}

///
/// Linearly interpolates between two points.
///
/// ```
/// use pbrt::geo::point3::{Point3f, lerp};
///
/// let p1 = Point3f::new(1.0, 0.0, 2.0);
/// let p2 = Point3f::new(3.0, 0.0, -4.0);
///
/// assert_eq!(lerp(0.0, p1, p2), p1);
/// assert_eq!(lerp(1.0, p1, p2), p2);
/// assert_eq!(lerp(0.5, p1, p2), Point3f::new(2.0, 0.0, -1.0));
/// ```
pub fn lerp(t: Float, p1: Point3f, p2: Point3f) -> Point3f {
    // let (x, y, z) = (
    //     crate::geo::lerp(t, p1.x, p2.x),
    //     crate::geo::lerp(t, p1.y, p2.y),
    //     crate::geo::lerp(t, p1.z, p2.z),
    // );
    // Point3f::new(x, y, z)
    p1 + (p2 - p1) * t
}
