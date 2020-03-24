use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use pbrt::camera::*;
use pbrt::geo::*;
use pbrt::hit::{Hit, HitStruct};
use pbrt::prelude::*;

fn to_color(v: Vec3f) -> Vec3<u8> {
    Vec3 {
        x: (v.x.sqrt() * 255.0) as u8,
        y: (v.y.sqrt() * 255.0) as u8,
        z: (v.z.sqrt() * 255.0) as u8,
    }
}

fn render(scene: &Scene, ray: &Ray, limit: usize) -> Vec3f {
    if let Some(hit) = scene.hit(ray, 0.0, std::f32::INFINITY) {
        let mut attenuation = Vec3f::new(0.0, 0.0, 0.0);
        if let Some(scattered) = hit.material.scatter(ray, &hit, &mut attenuation) {
            let c = render(scene, &scattered, limit - 1);
            Vec3f::new(
                c.x * attenuation.x,
                c.y * attenuation.y,
                c.z * attenuation.z,
            )
        } else {
            Vec3f::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit = ray.direction().normalized();
        let t = (unit.y + 1.0) * 0.5;
        pbrt::geo::vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
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

fn main() {
    let nx = 200; // image width, in pixels
    let ny = 100; // image height, in pixels
    let ns = 128; // number of samples per pixel
    let n_max_bounce = 50; // max number of bounces

    let mut scene = Scene {
        objects: Vec::new(),
    };

    use pbrt::material::lambertian::Lambertian;
    use pbrt::shape::sphere::Sphere;

    let green_mat = Lambertian {
        albedo: Vec3f::new(0.1, 0.8, 0.2),
    };

    let red_mat = Lambertian {
        albedo: Vec3f::new(0.9, 0.1, 0.1),
    };

    let s1 = Sphere {
        center: Point3f::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: &red_mat,
    };
    let s2 = Sphere {
        center: Point3f::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &green_mat,
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
                col += render(&scene, &ray, n_max_bounce);
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
