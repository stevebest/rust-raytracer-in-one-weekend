use crate::num_traits::Float;

use crate::geo::{Point3f, Ray, Vec3f};
use crate::material::Material;

// TODO: This should be called a Surface, or something. RTiaW calls it `hitable`.
pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitStruct>;
}

// TODO: `HitStruct` should be called `SurfaceInteraction`.
pub struct HitStruct<'a> {
    /// Time of hit
    pub t: Float,
    /// Point of hit
    pub p: Point3f,
    /// Normal to surface
    pub n: Vec3f,
    /// True if the incoming ray hit the front face of the surface
    pub front_face: bool,
    /// Material of a surface
    pub material: &'a dyn Material,
}

impl<'a> HitStruct<'a> {
    pub fn new(
        t: Float,
        p: Point3f,
        ray: &Ray,
        outward_normal: Vec3f,
        material: &'a dyn Material,
    ) -> HitStruct<'a> {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitStruct {
            t,
            p,
            n,
            front_face,
            material,
        }
    }
}
