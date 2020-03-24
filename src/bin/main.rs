use pbrt::prelude::*;

use pbrt::geo::*;

fn to_color(v: Vec3f) -> Vec3<u8> {
    Vec3 {
        x: (v.x * 255.0) as u8,
        y: (v.y * 255.0) as u8,
        z: (v.z * 255.0) as u8,
    }
}

fn color(scene: &Scene, ray: &Ray) -> Vec3<u8> {
    let c = if let Some(hit) = scene.hit(ray, 0.0, std::f32::INFINITY) {
        (hit.n + Vec3f::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit = ray.direction().normalized();
        let t = (unit.y + 1.0) * 0.5;
        pbrt::geo::vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
    };

    to_color(c)
}

struct HitStruct {
    /// Time of hit
    pub t: Float,
    /// Point of hit
    pub p: Point3f,
    /// Normal to surface
    pub n: Vec3f,
}

trait Hit {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitStruct>;
}

struct Scene<'a> {
    objects: Vec<&'a dyn Hit>,
}

impl Hit for Scene<'_> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitStruct> {
        let mut t_max = t_max;
        let mut hit_struct = None;
        for obj in self.objects.iter() {
            if let Some(hit) = obj.hit(ray, t_min, t_max) {
                t_max = hit.t;
                hit_struct = Some(hit);
            }
        }
        hit_struct
    }
}

struct Sphere {
    center: Point3f,
    radius: Float,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitStruct> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().len_squared();
        let b = oc.dot(ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.eval(t);
                let n = (p - self.center) * self.radius.recip();
                return Some(HitStruct { t, p, n });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.eval(t);
                let n = (p - self.center) * self.radius.recip();
                return Some(HitStruct { t, p, n });
            }
        }

        return None;
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3f::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3f::new(4.0, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, 2.0, 0.0);
    let origin = Point3f::origin();

    let mut scene = Scene { objects: vec![] };

    let s1 = Sphere {
        center: Point3f::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let s2 = Sphere {
        center: Point3f::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    scene.objects.push(&s1);
    scene.objects.push(&s2);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let direction = lower_left_corner + horizontal * u + vertical * v;
            let ray = Ray::new(origin, direction);

            let color = color(&scene, &ray);

            println!("{} {} {}", color.x, color.y, color.z);
        }
    }
}
