use crate::geo::*;
use crate::num_traits::*;

/// 3-dimensional axis-aligned bounding box (AABB)
#[derive(Copy, Clone, Debug)]
pub struct Bounds3<T>
where
    T: Numeric<T>,
{
    pub min: Point3<T>,
    pub max: Point3<T>,
}

pub type Bounds3f = Bounds3<Float>;

impl<T> Bounds3<T>
where
    T: Numeric<T>,
{
    ///
    /// Constructs bounds from two opposing corners.
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let p1 = point3(1.0, 0.0, -1.0);
    /// let p2 = point3(0.0, 2.0, 5.0);
    /// let b = Bounds3::from_corners(p1, p2);
    ///
    /// assert_eq!(b.min, point3(0.0, 0.0, -1.0));
    /// assert_eq!(b.max, point3(1.0, 2.0, 5.0));
    /// ```
    pub fn from_corners(p1: Point3<T>, p2: Point3<T>) -> Bounds3<T>
    where
        T: PartialOrd,
    {
        let min = p1.min(p2);
        let max = p1.max(p2);
        Bounds3 { min, max }
    }

    /// Constructs bounds enclosing a single point.
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let p = point3(1.0, -2.0, 3.0);
    /// let b = Bounds3::from_point(p);
    ///
    /// assert_eq!(b.min, p);
    /// assert_eq!(b.max, p);
    /// ```
    pub fn from_point(p: Point3<T>) -> Bounds3<T> {
        Bounds3 { min: p, max: p }
    }

    /// Construct an AABB enclosing two given AABBs.
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let b1 = Bounds3::from_point(point3(-1.0, 2.0, 3.0));
    /// let b2 = Bounds3::from_point(point3(2.0, -4.0, 2.0));
    ///
    /// let u = Bounds3::union(&b1, &b2);
    /// assert_eq!(u.min, point3(-1.0, -4.0, 2.0));
    /// assert_eq!(u.max, point3(2.0, 2.0, 3.0));
    /// ```
    pub fn union(b1: &Bounds3<T>, b2: &Bounds3<T>) -> Bounds3<T>
    where
        T: PartialOrd,
    {
        let (min, max) = (b1.min.min(b2.min), b1.max.max(b2.max));
        Bounds3 { min, max }
    }

    /// Surface area of the bounding box.
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let b = Bounds3::from_corners(point3(1.0, -1.0, 3.0), point3(2.0, 1.0, 0.0));
    /// assert_eq!(b.area(), 22.0); // 2 * (3 + 6 + 9)
    /// ```
    pub fn area(&self) -> T
    where
        T: Numeric<T>,
    {
        let Vec3 { x, y, z } = self.diagonal();
        let t = x * y + y * z + z * x;
        t + t
    }

    /// Main diagonal of the box, spanning from the min to max corner.
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let b = Bounds3::from_corners(point3(1.0, -1.0, 3.0), point3(2.0, 1.0, 0.0));
    /// assert_eq!(b.diagonal(), vec3(1.0, 2.0, 3.0));
    /// ```
    pub fn diagonal(&self) -> Vec3<T> {
        self.max - self.min
    }
}

impl Bounds3f {
    /// Quickly finds if the ray hits the AABB, in a given time interval.
    pub fn hit(&self, ray: &Ray, (t_min, t_max): (Float, Float)) -> bool {
        hit_naive(self, ray, (t_min, t_max))
    }

    /// Center point of the box
    ///
    /// ```
    /// use pbrt::geo::*;
    ///
    /// let b = Bounds3::from_corners(point3(1.0, -1.0, 3.0), point3(2.0, 1.0, 0.0));
    /// assert_eq!(b.centroid(), point3(1.5, 0.0, 1.5));
    /// ```
    pub fn centroid(&self) -> Point3f {
        Point3::lerp(0.5, self.min, self.max)
    }
}

fn hit_naive(b: &Bounds3f, r: &Ray, (t_min, t_max): (Float, Float)) -> bool {
    // for (int a = 0; a < 3; a++) {
    //     auto t0 = ffmin((_min[a] - r.origin()[a]) / r.direction()[a],
    //                     (_max[a] - r.origin()[a]) / r.direction()[a]);
    //     auto t1 = ffmax((_min[a] - r.origin()[a]) / r.direction()[a],
    //                     (_max[a] - r.origin()[a]) / r.direction()[a]);
    //     tmin = ffmax(t0, tmin);
    //     tmax = ffmin(t1, tmax);
    //     if (tmax <= tmin)
    //         return false;
    // }
    // return true;

    // TODO: Vectorize AABB-ray intersection finder (SIMD)

    let (o, d) = r.origin_and_direction();

    let inv = d.x.recip();
    let (u, v) = ((b.min.x - o.x) * inv, (b.max.x - o.x) * inv);
    let (t0, t1) = min_max(u, v);
    let (t_min, t_max) = (max(t0, t_min), min(t1, t_max));
    if t_max <= t_min {
        return false;
    }

    let inv = d.y.recip();
    let (u, v) = ((b.min.y - o.y) * inv, (b.max.y - o.y) * inv);
    let (t0, t1) = min_max(u, v);
    let (t_min, t_max) = (max(t0, t_min), min(t1, t_max));
    if t_max <= t_min {
        return false;
    }

    let inv = d.z.recip();
    let (u, v) = ((b.min.z - o.z) * inv, (b.max.z - o.z) * inv);
    let (t0, t1) = min_max(u, v);
    let (t_min, t_max) = (max(t0, t_min), min(t1, t_max));
    if t_max <= t_min {
        return false;
    }

    true
}

#[inline]
fn min_max(u: Float, v: Float) -> (Float, Float) {
    if u < v {
        (u, v)
    } else {
        (v, u)
    }
}
