use crate::num_traits::Float;

///
/// A 4x4 matrix.
///
/// ```
/// use pbrt::geo::Mat4;
///
/// let m = Mat4::identity();
///
/// assert_eq!(m[(0, 0)], 1.0);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    m: [[Float; 4]; 4],
}

impl Mat4 {
    ///
    /// Creates an identity matrix.
    ///
    pub fn identity() -> Mat4 {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Mat4 { m }
    }
}

impl std::ops::Index<(usize, usize)> for Mat4 {
    type Output = Float;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.m[r][c]
    }
}
