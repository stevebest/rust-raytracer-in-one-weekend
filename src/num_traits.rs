/// Internal floating-point precision type.
pub type Float = f32;

pub const EPSILON: Float = 1.0e-6;

///
/// Number-like type.
///
pub trait Numeric<T>:
    Sized + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>
{
}

impl Numeric<f32> for f32 {}
impl Numeric<f64> for f64 {}
impl Numeric<isize> for isize {}

///
/// Reciprocal.
///
pub trait Recip {
    fn recip(self) -> Self;
}

macro_rules! impl_recip {
    ($t:ty) => {
        impl Recip for $t {
            fn recip(self) -> Self {
                1.0 / self
            }
        }
    };
}

impl_recip!(f32);
impl_recip!(f64);

///
/// Absolute value of a number.
///
pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ($t:ty) => {
        impl Abs for $t {
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

impl_abs!(f32);
impl_abs!(f64);
impl_abs!(isize);

///
/// Square root.
///
pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($t:ty) => {
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                self.sqrt()
            }
        }
    };
}

impl_sqrt!(f32);
impl_sqrt!(f64);

///
/// Zero - additive identity.
///
pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> Self {
                $v
            }
        }
    };
}

impl_zero!(f32, 0.0f32);
impl_zero!(f64, 0.0f64);
impl_zero!(isize, 0);

///
/// One - multiplicative identity
///
pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($t:ty, $v:expr) => {
        impl One for $t {
            fn one() -> Self {
                $v
            }
        }
    };
}

impl_one!(f32, 1.0f32);
impl_one!(f64, 1.0f64);
impl_one!(isize, 1);
