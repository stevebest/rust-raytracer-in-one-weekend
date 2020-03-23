use crate::num_traits::Float;

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<Float>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

///
/// Vector-scalar multiplication.
///
impl<T> std::ops::Mul<T> for Vec3<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;
    /// ```
    /// use pbrt::geo::vec3::Vec3;
    /// let v = Vec3::new(1.0, 2.0, -1.0);
    /// assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, -2.0));
    /// ```
    fn mul(self, s: T) -> Self::Output {
        mul(self, s)
    }
}

fn mul<T>(v: Vec3<T>, s: T) -> Vec3<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    Vec3::new(v.x * s, v.y * s, v.z * s)
}
