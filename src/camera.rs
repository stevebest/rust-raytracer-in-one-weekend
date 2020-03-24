use crate::geo::{Point3f, Ray, Vec3f};
use crate::prelude::*;

pub struct Camera {
    origin: Point3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Point3f::origin(),
            lower_left_corner: Vec3f::new(-2.0, -1.0, -1.0),
            horizontal: Vec3f::new(4.0, 0.0, 0.0),
            vertical: Vec3f::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray::new(self.origin, direction)
    }
}
