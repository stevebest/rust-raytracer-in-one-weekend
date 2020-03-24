use crate::geo::vec3::Vec3;
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
