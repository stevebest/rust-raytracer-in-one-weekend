use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use pbrt::camera::*;
use pbrt::geo::*;
use pbrt::prelude::*;

fn to_color(v: Vec3f) -> Vec3<u8> {
    Vec3 {
        x: (v.x.sqrt() * 255.0) as u8,
        y: (v.y.sqrt() * 255.0) as u8,
        z: (v.z.sqrt() * 255.0) as u8,
    }
}

fn render(scene: &Scene, ray: &Ray) -> Vec3f {
    if let Some(hit) = scene.hit(ray, 0.0, std::f32::INFINITY) {
        let target = hit.p + hit.n + random_in_unit_sphere();
        render(scene, &Ray::new(hit.p, target - hit.p)) * 0.5
    } else {
        let unit = ray.direction().normalized();
        let t = (unit.y + 1.0) * 0.5;
        pbrt::geo::vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}

/// Unbiased random direction
fn random_in_unit_sphere() -> Vec3f {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3f::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 + Vec3f::new(-1.0, -1.0, -1.0);
        if v.len_squared() <= 1.0 {
            return v;
        }
    }
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
    let nx = 200; // image width, in pixels
    let ny = 100; // image height, in pixels
    let ns = 8; // number of samples per pixel

    let mut scene = Scene {
        objects: Vec::new(),
    };

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

    let camera = Camera::new();

    let mut pixels = Vec::<u8>::with_capacity(nx * ny * 4);

    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3f::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f32) + rng.gen::<f32>()) / (nx as f32);
                let v = ((j as f32) + rng.gen::<f32>()) / (ny as f32);
                let ray = camera.get_ray(u, v);
                col += render(&scene, &ray);
            }
            let col = to_color(col * (ns as f32).recip());

            pixels.push(col.x);
            pixels.push(col.y);
            pixels.push(col.z);
            pixels.push(255);
        }
    }

    let path = Path::new(r"img.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
}
