pub mod bounds2;
pub mod mat4;
pub mod point2;
pub mod point3;
pub mod ray;
pub mod transform;
pub mod vec2;
pub mod vec3;

pub use bounds2::{Bounds2, Bounds2f};

pub use point2::{Point2, Point2f};
pub use point3::{Point3, Point3f};

pub use vec2::{Vec2, Vec2f};
pub use vec3::{Vec3, Vec3f};

pub use mat4::Mat4;

pub use ray::Ray;

use crate::num_traits::*;

/// Creates a new 3D vector.
pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Numeric<T>,
{
    fn from((x, y, z): (T, T, T)) -> Self {
        vec3(x, y, z)
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Numeric<T>,
{
    fn from([x, y, z]: [T; 3]) -> Self {
        vec3(x, y, z)
    }
}

/// Creates a new 3D point.
pub fn point3<T>(x: T, y: T, z: T) -> Point3<T> {
    Point3::new(x, y, z)
}

impl<T> From<Vec3<T>> for Point3<T>
where
    T: Numeric<T>,
{
    fn from(Vec3 { x, y, z }: Vec3<T>) -> Self {
        point3(x, y, z)
    }
}

impl<T> From<(T, T, T)> for Point3<T>
where
    T: Numeric<T>,
{
    fn from((x, y, z): (T, T, T)) -> Self {
        point3(x, y, z)
    }
}

impl<T> From<[T; 3]> for Point3<T>
where
    T: Numeric<T>,
{
    fn from([x, y, z]: [T; 3]) -> Self {
        point3(x, y, z)
    }
}

///
/// Linearly interpolates between two values.
///
/// ```
/// use pbrt::geo::lerp;
///
/// let a = 1.0;
/// let b = 5.0;
/// let t = 0.25;
///
/// assert_eq!(lerp(t, a, b), 2.0);
/// ```
pub fn lerp(t: Float, v1: Float, v2: Float) -> Float {
    (1.0 - t) * v1 + t * v2
}
