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
    // 1.0e-4 prevents shadow acne
    if let Some(hit) = scene.hit(ray, 1.0e-4, std::f32::INFINITY) {
        if limit == 0 {
            return Vec3f::new(0.0, 0.0, 0.0);
        }
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

        // Sky
        // pbrt::geo::vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)

        // Studio
        pbrt::geo::vec3::lerp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), t)

        // pbrt::geo::vec3::lerp(Vec3::new(0.7, 0.2, 0.1), Vec3::new(0.5, 0.7, 1.0), t)
        // pbrt::geo::vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), t)
        // pbrt::geo::vec3::lerp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.7, 1.0), t)
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

fn write_image(
    filename: &str,
    pixels: &[u8],
    (nx, ny): (usize, usize),
) -> Result<(), std::io::Error> {
    let path = Path::new(&filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let res = 40;
    let nx = 16 * res; // image width, in pixels
    let ny = 9 * res; // image height, in pixels
    let ns = 8; // number of samples per pixel
    let n_max_bounce = 50; // max number of bounces

    let mut scene = Scene {
        objects: Vec::new(),
    };

    use pbrt::material::dielectric::Dielectric;
    use pbrt::material::lambertian::Lambertian;
    use pbrt::material::metal::Metal;

    use pbrt::shape::sphere::Sphere;

    // Earth
    let s1 = Sphere {
        center: Point3f::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &Metal {
            albedo: Vec3f::new(0.5, 0.5, 0.5),
            roughness: 0.3,
        },
        // material: &Lambertian {
        //     albedo: Vec3f::new(0.3, 0.3, 0.3),
        // },
    };
    // Rubber
    let s2 = Sphere {
        center: Point3f::new(0.0, 0.0, -1.6),
        radius: 0.5,
        material: &Lambertian {
            albedo: Vec3f::new(0.9, 0.1, 0.1),
        },
    };
    // Gold
    let s3 = Sphere {
        center: Point3f::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Metal {
            albedo: Vec3f::new(0.8, 0.6, 0.2),
            roughness: 0.3,
        },
    };
    // Silver
    let s4 = Sphere {
        center: Point3f::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Metal {
            albedo: Vec3f::new(0.8, 0.8, 0.8),
            roughness: 0.0,
        },
    };
    // Glass
    let s5 = Sphere {
        center: Point3f::new(0.0, 0.0, -0.5),
        radius: 0.5,
        material: &Dielectric {
            refraction_index: 1.33333,
        },
    };

    scene.objects.push(&s1);
    scene.objects.push(&s2);
    scene.objects.push(&s3);
    scene.objects.push(&s4);
    scene.objects.push(&s5);

    let camera = Camera::from_spec(CameraSpec {
        vfov: 60.0,
        aspect: nx as Float / ny as Float,
        // look_from: Point3f::new(1.0, 1.0, 1.0),
        look_from: Point3f::new(1.0, 1.5, 3.0),
        look_at: Point3f::new(0.0, 0.0, -1.0),
        up: Vec3f::new(0.0, 1.0, 0.0),
    });

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

    let filename = format!("img{}.png", 0);
    write_image(&filename, &pixels, (nx, ny))?;

    Ok(())
}
