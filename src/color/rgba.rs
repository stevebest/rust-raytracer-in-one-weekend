#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    pub fn from_channels(r: T, g: T, b: T, a: T) -> Rgba<T> {
        Rgba { r, g, b, a }
    }
}
