use crate::num_traits::{Float, Numeric, One, Recip, Sqrt};

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<Float>;
pub type Vec3i = Vec3<isize>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    /// Inner (dot) product
    pub fn dot(self, rhs: Vec3<T>) -> T
    where
        T: Numeric<T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn len_squared(self) -> T
    where
        T: Numeric<T>,
    {
        self.dot(self)
    }

    pub fn len(self) -> T
    where
        T: Numeric<T> + Sqrt,
    {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Vec3<T>
    where
        T: Numeric<T> + Sqrt + Recip,
    {
        self * (self.len().recip())
    }
}

pub fn lerp<T>(v1: Vec3<T>, v2: Vec3<T>, t: T) -> Vec3<T>
where
    T: Numeric<T> + One,
{
    v1 * (T::one() - t) + v2 * t
}

impl<T> std::ops::Neg for Vec3<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        let Vec3 { x, y, z } = self;
        Vec3::new(-x, -y, -z)
    }
}

///
/// Vector addition.
///
impl<T> std::ops::Add for Vec3<T>
where
    T: Numeric<T>,
{
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::AddAssign for Vec3<T>
where
    T: Numeric<T>,
{
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = *self + rhs
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
