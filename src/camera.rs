use crate::prelude::*;

use crate::geo::{Point3f, Ray, Vec3f};

pub struct CameraSpec {
    /// Vertical field of view angle, in degrees.
    pub vfov: Float,

    /// Aspect ratio (width / height).
    pub aspect: Float,

    /// Focal point position.
    pub look_from: Point3f,

    /// Target of focus.
    pub look_at: Point3f,

    /// 'Up' vector.
    pub up: Vec3f,
}

pub struct Camera {
    origin: Point3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
}

impl Camera {
    pub fn from_spec(spec: CameraSpec) -> Camera {
        let origin = spec.look_from;

        let w = (spec.look_from - spec.look_at).normalized();
        let u = Vec3f::cross(&spec.up, &w).normalized();
        let v = Vec3f::cross(&w, &u);

        let theta = degrees_to_radians(spec.vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = spec.aspect * half_height;

        let lower_left_corner = (origin - Point3f::origin()) - u * half_width - v * half_height - w;
        let horizontal = u * (2.0 * half_width);
        let vertical = v * (2.0 * half_height);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v
            - (self.origin - Point3f::origin());
        Ray::new(self.origin, direction)
    }
}

fn degrees_to_radians(degrees: Float) -> Float {
    degrees / 360.0 * std::f32::consts::PI
}
