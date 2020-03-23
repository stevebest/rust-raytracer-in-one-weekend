use crate::num_traits::Float;
use crate::num_traits::{Abs, Numeric, Recip, Sqrt};

/// Two-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2f = Vec2<Float>;
pub type Vec2i = Vec2<isize>;

pub enum Dim {
    X,
    Y,
}

impl<T> Vec2<T> {
    /// Constructs a vector with given `x` and `y` components.
    ///
    /// Alternatively, a vector could be constructed from a tuple.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(1.0, v.x);
    /// assert_eq!(2.0, v.y);
    /// ```
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    ///
    /// Dot product.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let a = Vec2::new(1, 3);
    /// let b = Vec2::new(2, -1);
    /// assert_eq!(a.dot(b), -1);
    ///
    /// let a = Vec2::new(1.0, 3.0);
    /// let b = Vec2::new(2.0, -1.0);
    /// assert_eq!(a.dot(b), -1.0);
    /// ```
    pub fn dot(self, other: Vec2<T>) -> T
    where
        T: Numeric<T>,
    {
        self.x * other.x + self.y * other.y
    }

    ///
    /// Absolute value of a dot product.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let a = Vec2::new(1.0, 3.0);
    /// let b = Vec2::new(2.0, -1.0);
    /// assert_eq!(a.abs_dot(b), 1.0);
    /// ```
    pub fn abs_dot(self, other: Vec2<T>) -> T
    where
        T: Numeric<T> + Abs,
    {
        self.dot(other).abs()
    }

    ///
    /// Squared length of a vector.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let v = Vec2::new(-3.0, 4.0);
    /// assert_eq!(v.len_squared(), 25.0);
    /// ```
    pub fn len_squared(self) -> T
    where
        T: Numeric<T>,
    {
        self.dot(self.clone())
    }

    ///
    /// Euclidean length of a vector.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let v = Vec2::new(-3.0, 4.0);
    /// assert_eq!(v.len(), 5.0);
    /// ```
    pub fn len(self) -> T
    where
        T: Numeric<T> + Sqrt,
    {
        self.len_squared().sqrt()
    }

    ///
    /// Creates and returns a vector collinear to `self` of length `1`.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// use std::f32::EPSILON;
    ///
    /// let v = Vec2::new(-3.0f32, 4.0);
    /// let n = v.normalized();
    /// assert!((n.x - (-0.6)).abs() < EPSILON);
    /// assert!((n.y - 0.8).abs() < EPSILON);
    /// ```
    pub fn normalized(self) -> Vec2<T>
    where
        T: Numeric<T> + Sqrt + Recip,
    {
        self * self.len().recip()
    }

    ///
    /// Smallest coordinate value.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let v = Vec2::new(1, 2);
    /// assert_eq!(v.min_component(), 1);
    ///
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(v.min_component(), 1.0);
    /// ```
    pub fn min_component(self) -> T
    where
        T: PartialOrd,
    {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }

    ///
    /// Largest coordinate value.
    ///
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    ///
    /// let v = Vec2::new(1, 2);
    /// assert_eq!(v.max_component(), 2);
    ///
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(v.max_component(), 2.0);
    /// ```
    pub fn max_component(self) -> T
    where
        T: PartialOrd,
    {
        if self.x >= self.y {
            self.x
        } else {
            self.y
        }
    }

    ///
    /// ```
    /// use pbrt::geo::vec2::{Vec2, Dim};
    ///
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(v.permute(Dim::Y, Dim::X), Vec2::new(2.0, 1.0));
    /// ```
    pub fn permute(self, x: Dim, y: Dim) -> Self
    where
        T: Copy,
    {
        Vec2::new(self[x], self[y])
    }
}

impl Vec2<Float> {
    pub fn reflect(self, n: Vec2<Float>) -> Vec2<Float> {
        // assert!(n.is_normalized(), "Vec3::reflect must use normalized `n`");
        self - n * 2.0 * (self.dot(n))
    }
}

///
/// Allows constructing a vector from a tuple.
///
impl<T> From<(T, T)> for Vec2<T> {
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let t = (1.0, 2.0);
    /// let v = Vec2::from(t);
    /// assert_eq!(1.0, v.x);
    /// assert_eq!(2.0, v.y);
    /// ```
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        let (x, y) = tuple;
        Self::new(x, y)
    }
}

///
/// Allows indexing into a vector using the name of the dimension.
///
impl<T> std::ops::Index<Dim> for Vec2<T> {
    type Output = T;

    /// ```
    /// use pbrt::geo::vec2::{Vec2, Dim};
    /// let v = Vec2::<f32>::new(1.0, 2.0);
    /// let mut d = Dim::X;
    /// assert_eq!(1.0, v[d]);
    /// d = Dim::Y;
    /// assert_eq!(2.0, v[d]);
    /// ```
    fn index(&self, dim: Dim) -> &Self::Output {
        match dim {
            Dim::X => &self.x,
            Dim::Y => &self.y,
        }
    }
}

///
/// Mutable indexing using a dimension name.
///
impl<T> std::ops::IndexMut<Dim> for Vec2<T> {
    fn index_mut(&mut self, dim: Dim) -> &mut Self::Output {
        match dim {
            Dim::X => &mut self.x,
            Dim::Y => &mut self.y,
        }
    }
}

///
/// Vector addition.
///
impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let a = Vec2::new(1.0, 2.0);
    /// let b = Vec2::new(2.0, -1.0);
    /// assert_eq!(a + b, Vec2::new(3.0, 1.0));
    /// ```
    fn add(self, other: Vec2<T>) -> Self::Output {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> std::ops::AddAssign for Vec2<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let mut a = Vec2::new(1.0, 2.0);
    /// let b = Vec2::new(2.0, -1.0);
    /// a += b;
    /// assert_eq!(a, Vec2::new(3.0, 1.0));
    /// ```
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

///
/// Vector subtraction.
///
impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let a = Vec2::new(1.0, 2.0);
    /// let b = Vec2::new(2.0, -1.0);
    /// assert_eq!(a - b, Vec2::new(-1.0, 3.0));
    /// ```
    fn sub(self, other: Vec2<T>) -> Self::Output {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

///
/// Vector-scalar multiplication.
///
impl<T> std::ops::Mul<T> for Vec2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(v * 2.0, Vec2::new(2.0, 4.0));
    /// ```
    fn mul(self, s: T) -> Self::Output {
        mul(self, s)
    }
}

fn mul<T>(v: Vec2<T>, s: T) -> Vec2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    Vec2::new(v.x * s, v.y * s)
}

macro_rules! impl_mul {
    ($t:ty) => {
        impl std::ops::Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self, v: Vec2<$t>) -> Self::Output {
                mul(v, self)
            }
        }
    };
}

impl_mul!(f32);
impl_mul!(f64);
impl_mul!(isize);

impl<T> std::ops::Div<T> for Vec2<T>
where
    T: Recip + std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let v = Vec2::new(1.0, 2.0);
    /// assert_eq!(v / 2.0, Vec2::new(0.5, 1.0));
    /// ```
    fn div(self, divisor: T) -> Self::Output {
        let s = divisor.recip();
        self * s
    }
}

impl<T> std::ops::Neg for Vec2<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Vec2<T>;
    /// ```
    /// use pbrt::geo::vec2::Vec2;
    /// let v = Vec2::new(-1.0, 2.0);
    /// assert_eq!(-v, Vec2::new(1.0, -2.0));
    /// ```
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}
