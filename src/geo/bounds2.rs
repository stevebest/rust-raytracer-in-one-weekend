use crate::geo::point2::Point2;
use crate::num_traits::Float;
use crate::num_traits::Numeric;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bounds2<T>
where
    T: Numeric<T>,
{
    pub min: Point2<T>,
    pub max: Point2<T>,
}

pub type Bounds2f = Bounds2<Float>;

impl<T> Bounds2<T>
where
    T: Numeric<T>,
{
    ///
    /// Constructs bounds from two opposing corners.
    ///
    /// ```
    /// use pbrt::geo::{Bounds2, Point2};
    ///
    /// let p1 = Point2::new(1.0, 0.0);
    /// let p2 = Point2::new(0.0, 2.0);
    /// let b = Bounds2::from_corners(p1, p2);
    ///
    /// assert_eq!(b.min, Point2::new(0.0, 0.0));
    /// assert_eq!(b.max, Point2::new(1.0, 2.0));
    /// ```
    pub fn from_corners(p1: Point2<T>, p2: Point2<T>) -> Bounds2<T>
    where
        T: PartialOrd,
    {
        let min = p1.min(p2);
        let max = p1.max(p2);
        Bounds2 { min, max }
    }

    ///
    /// Constructs bounds enclosing a single given point.
    ///
    /// ```
    /// use pbrt::geo::{Bounds2, Point2};
    ///
    /// let p = Point2::new(1.0, 0.0);
    /// let b = Bounds2::from_point(p);
    ///
    /// assert_eq!(b.min, p);
    /// assert_eq!(b.max, p);
    /// ```
    pub fn from_point(p: Point2<T>) -> Bounds2<T> {
        Bounds2 { min: p, max: p }
    }
}
