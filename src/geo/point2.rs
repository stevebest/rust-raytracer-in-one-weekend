use super::vec2::Vec2;
use crate::num_traits::Float;
use crate::num_traits::{Abs, Numeric};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2f = Vec2<Float>;
pub type Point2i = Vec2<isize>;

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x, y }
    }

    pub fn abs(self) -> Point2<T>
    where
        T: Numeric<T> + Abs,
    {
        Point2::new(self.x.abs(), self.y.abs())
    }

    ///
    /// Component-wise minimum
    ///
    /// ```
    /// use pbrt::geo::Point2;
    ///
    /// let p1 = Point2::new(1, 5);
    /// let p2 = Point2::new(3, 2);
    ///
    /// assert_eq!(p1.min(p2), Point2::new(1, 2));
    /// ```
    pub fn min(self, other: Point2<T>) -> Point2<T>
    where
        T: PartialOrd,
    {
        let x = if self.x <= other.x { self.x } else { other.x };
        let y = if self.y <= other.y { self.y } else { other.y };
        Point2::new(x, y)
    }

    pub fn max(self, other: Point2<T>) -> Point2<T>
    where
        T: PartialOrd,
    {
        let x = if self.x >= other.x { self.x } else { other.x };
        let y = if self.y >= other.y { self.y } else { other.y };
        Point2::new(x, y)
    }
}

///
/// Point-vector addition.
///
impl<T> std::ops::Add<Vec2<T>> for Point2<T>
where
    T: Numeric<T>,
{
    type Output = Point2<T>;
    fn add(self, v: Vec2<T>) -> Self::Output {
        Point2::new(self.x + v.x, self.y + v.y)
    }
}
