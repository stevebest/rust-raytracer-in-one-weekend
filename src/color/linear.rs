use crate::num_traits::Float;

use super::rgba::Rgba;

#[derive(Copy, Clone, Debug)]
pub struct LinearColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
}

impl LinearColor {
    pub fn from_channels(r: Float, g: Float, b: Float, a: Float) -> LinearColor {
        LinearColor { r, g, b, a }
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.r) * 255.0) as u8,
            (gamma_encode(self.g) * 255.0) as u8,
            (gamma_encode(self.b) * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }
    pub fn from_rgba(rgba: Rgba<u8>) -> LinearColor {
        LinearColor {
            r: gamma_decode(rgba.r as f32 / 255.0),
            g: gamma_decode(rgba.r as f32 / 255.0),
            b: gamma_decode(rgba.r as f32 / 255.0),
            a: rgba.a as f32 / 255.0,
        }
    }
}

impl Default for LinearColor {
    fn default() -> LinearColor {
        LinearColor::from_channels(0.0, 0.0, 0.0, 1.0)
    }
}

impl std::ops::Mul<Float> for LinearColor {
    type Output = LinearColor;
    fn mul(self, s: Float) -> LinearColor {
        LinearColor::from_channels(self.r * s, self.g * s, self.b * s, self.a)
    }
}

impl std::ops::Add for LinearColor {
    type Output = LinearColor;
    fn add(self, rhs: LinearColor) -> LinearColor {
        LinearColor::from_channels(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a)
    }
}

const GAMMA: Float = 2.2;

fn gamma_encode(linear: Float) -> Float {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: Float) -> Float {
    encoded.powf(GAMMA)
}
