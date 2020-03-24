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

    /// Material of a surface
    pub material: &'a dyn Material,
}
