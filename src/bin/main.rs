use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use pbrt::camera::*;
use pbrt::color::*;
use pbrt::geo::*;
use pbrt::hit::{Hit, HitStruct};
use pbrt::material::*;
use pbrt::prelude::*;

fn tonemap(colors: &[LinearColor], (nx, ny): (usize, usize)) -> Vec<Rgba<u8>> {
    let mut pixels = Vec::<Rgba<u8>>::with_capacity(nx * ny);
    pixels.resize_with(nx * ny, Default::default);
    colors.iter().zip(pixels.iter_mut()).for_each(|(c, p)| {
        *p = c.to_rgba();
    });
    pixels
}

fn ray_color(scene: &Scene, ray: &Ray, limit: usize) -> LinearColor {
    // 1.0e-4 prevents shadow acne
    if let Some(hit) = scene.hit(ray, 1.0e-4, std::f32::INFINITY) {
        if limit == 0 {
            return Default::default();
        }
        let mut attenuation = vec3(0.0, 0.0, 0.0);
        if let Some(scattered) = hit.material.scatter(ray, &hit, &mut attenuation) {
            let c = ray_color(scene, &scattered, limit - 1);
            LinearColor::from_channels(
                c.r * attenuation.x,
                c.g * attenuation.y,
                c.b * attenuation.z,
                1.0,
            )
        } else {
            Default::default()
        }
    } else {
        let unit = ray.direction().normalized();
        let t = (unit.y + 1.0) * 0.5;

        // Sky
        // lerp(t, LinearColor::from_channels(1.0, 1.0, 1.0, 1.0), LinearColor::from_channels(0.5, 0.7, 1.0, 1.0))

        // Studio
        lerp(
            t,
            LinearColor::from_channels(0.0, 0.0, 0.0, 0.0),
            LinearColor::from_channels(1.0, 1.0, 1.0, 0.0),
        )

        // lerp(t, LinearColor::from_channels(0.7, 0.2, 0.1, 1.0), LinearColor::from_channels(0.5, 0.7, 1.0, 1.0))
        // lerp(t, LinearColor::from_channels(1.0, 1.0, 1.0, 1.0), LinearColor::from_channels(0.0, 0.0, 0.0, 1.0))
        // lerp(t, LinearColor::from_channels(0.0, 0.0, 0.0, 1.0), LinearColor::from_channels(0.5, 0.7, 1.0, 1.0))
    }
}

fn lerp<T>(t: Float, a: T, b: T) -> T
where
    T: std::ops::Mul<Float, Output = T> + std::ops::Add<Output = T>,
{
    a * (1.0 - t) + b * t
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

fn render(scene: &Scene, camera: &Camera, opt: RenderOptions) -> Vec<LinearColor> {
    use rand::prelude::*;
    use rayon::prelude::*;

    let pixels = (0..opt.ny)
        .into_par_iter()
        .rev()
        .map(|j| {
            let row: Vec<_> = (0..opt.nx)
                .into_par_iter()
                .map(|i| {
                    let mut rng = rand::thread_rng();
                    let mut color = LinearColor::default();
                    for _ in 0..opt.ns {
                        let u = ((i as f32) + rng.gen::<f32>()) / (opt.nx as f32);
                        let v = ((j as f32) + rng.gen::<f32>()) / (opt.ny as f32);
                        let ray = camera.get_ray(u, v);
                        color = color + ray_color(&scene, &ray, opt.n_max_bounce);
                    }
                    color = color * (1.0 / opt.ns as f32);

                    color
                })
                .collect();

            row
        })
        .collect::<Vec<_>>()
        .concat();

    pixels
}

fn write_image(
    filename: &str,
    pixels: &mut [Rgba<u8>],
    (nx, ny): (usize, usize),
) -> Result<(), std::io::Error> {
    let path = Path::new(&filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let pixel_data = std::mem::ManuallyDrop::new(unsafe {
        Vec::from_raw_parts(
            pixels.as_mut_ptr() as *mut u8,
            pixels.len() * 4,
            pixels.len() * 4,
        )
    });

    writer.write_image_data(&pixel_data).unwrap();

    Ok(())
}

#[derive(Copy, Clone)]
struct RenderOptions {
    nx: usize,
    ny: usize,
    ns: usize,
    n_max_bounce: usize,
}

impl RenderOptions {
    fn parse() -> RenderOptions {
        RenderOptions::from_args(std::env::args().collect())
    }
    fn from_args(args: Vec<String>) -> RenderOptions {
        fn arg<T: std::str::FromStr>(arg: Option<&String>, default: T) -> T {
            arg.unwrap_or(&"".into()).parse::<T>().unwrap_or(default)
        }

        let default = RenderOptions::default();

        RenderOptions {
            nx: arg(args.get(1), default.nx),
            ny: arg(args.get(2), default.ny),
            ns: arg(args.get(3), default.ns),
            n_max_bounce: 50,
        }
    }
}

impl Default for RenderOptions {
    fn default() -> RenderOptions {
        RenderOptions {
            nx: 40 * 16,
            ny: 40 * 9,
            ns: 8,
            n_max_bounce: 50,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let render_options = RenderOptions::parse();

    println!(
        "Rendering {}x{} at {} samples per pixels",
        render_options.nx, render_options.ny, render_options.ns
    );

    let mut scene = Scene {
        objects: Vec::new(),
    };

    use pbrt::shape::sphere::Sphere;

    // Earth
    let s1 = Sphere {
        center: Point3f::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &Metal {
            albedo: vec3(0.5, 0.5, 0.5),
            roughness: 0.3,
        },
        // material: &Lambertian {
        //     albedo: vec3(0.3, 0.3, 0.3),
        // },
    };
    // Rubber
    let s2 = Sphere {
        center: Point3f::new(0.0, 0.0, -1.6),
        radius: 0.5,
        material: &Lambertian {
            albedo: vec3(0.9, 0.1, 0.1),
        },
    };
    // Gold
    let s3 = Sphere {
        center: Point3f::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Metal {
            albedo: vec3(0.8, 0.6, 0.2),
            roughness: 0.3,
        },
    };
    // Silver
    let s4 = Sphere {
        center: Point3f::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Metal {
            albedo: vec3(0.8, 0.8, 0.8),
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
        aspect: render_options.nx as Float / render_options.ny as Float,
        // look_from: Point3f::new(1.0, 1.0, 1.0),
        look_from: Point3f::new(1.0, 1.5, 3.0),
        look_at: Point3f::new(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
    });

    let RenderOptions { nx, ny, .. } = render_options;

    let colors = render(&scene, &camera, render_options);

    let mut pixels = tonemap(&colors, (nx, ny));

    let filename = format!("img{}.png", 0);
    write_image(&filename, &mut pixels, (nx, ny))?;

    Ok(())
}
