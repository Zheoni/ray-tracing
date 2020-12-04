mod camera;
mod hittable;
mod image;
mod material;
mod objects;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList};
use image::Image;
use vec3::*;

use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use rand::Rng;
#[macro_use]
extern crate clap;
use clap::App;
use pbr::ProgressBar;

fn gen_random_scene() -> HittableList {
    use material::*;
    use objects::sphere::Sphere;

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Add the ground
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                (a as f64) + 0.9 * rand::random::<f64>(),
                0.2,
                (b as f64) + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian { albedo },
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);

                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal { albedo, fuzz },
                    }));
                } else {
                    // glass
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric {
                            index_refraction: 1.5,
                        },
                    }));
                }
            }
        }
    }

    // Big glass boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric {
            index_refraction: 1.5,
        },
    }));

    // Big diffuse boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    }));

    // Big metal boi
    objects.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    }));

    HittableList { objects }
}

fn main() -> Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yaml");
    let config = App::from_yaml(yaml).get_matches();

    let mut file: Box<dyn std::io::Write> = if config.is_present("stdout") {
        Box::new(std::io::stdout())
    } else {
        let output = config.value_of("output").unwrap();
        Box::new(File::create(output)?)
    };

    // Image
    let aspect_ratio = {
        let r = config.value_of("aspect_ratio").unwrap();
        let p: Vec<_> = r.split('/').map(str::parse::<usize>).collect();
        assert_eq!(p.len(), 2, "Invalid aspect ratio format");
        let w = p[0].as_ref().expect("Invalid aspect ratio width");
        let h = p[1].as_ref().expect("Invalid aspect ratio height");
        *w as f64 / *h as f64
    };
    let image_height: usize = config
        .value_of("image_height")
        .unwrap()
        .parse()
        .expect("Invalid image height");
    let image_width: usize = (image_height as f64 * aspect_ratio).floor() as usize;

    let image = Arc::new(Mutex::new(Image::new(image_width, image_height)));

    // Antialias / noise
    let samples_per_pixel: usize = config
        .value_of("samples_per_pixel")
        .unwrap()
        .parse()
        .expect("Invalid spp");
    // Max recursive rays
    let max_depth: u32 = config
        .value_of("ray_depth")
        .unwrap()
        .parse()
        .expect("Invalid ray depth");

    // World
    let world = Arc::new(gen_random_scene());

    // Camera
    let lookfrom = Vec3::new(12.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let aperture = 0.1;

    let cam = Arc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    ));

    // Render
    let cpus: usize = if let Some(cpus) = config.value_of("cores") {
        cpus.parse().expect("Invalid number of cores")
    } else {
        num_cpus::get_physical()
    };

    let mut handles = Vec::new();

    let scanlines_counter = Arc::new(AtomicUsize::new(image_height));

    let start_instant = Instant::now();

    if config.is_present("debug") {
        eprintln!("Resolution: {}x{}", image_width, image_height);
        eprintln!("Aspect ratio: {}", aspect_ratio);
        eprintln!("SPP: {}", samples_per_pixel);
        eprintln!("Ray depth: {}", max_depth);
    }

    eprintln!("Rendering with {} cores...", cpus);
    for t in 0..cpus {
        let cam = Arc::clone(&cam);
        let world = Arc::clone(&world);
        let image = Arc::clone(&image);
        let scanlines_counter = Arc::clone(&scanlines_counter);

        let w = image_height / cpus;
        let start = t * w;
        let end = if t == cpus - 1 {
            image_height
        } else {
            (t + 1) * w
        };
        let handle = thread::spawn(move || {
            for j in start..end {
                for i in 0..image_width {
                    let mut pixel = Vec3::zero();
                    for _ in 0..samples_per_pixel {
                        let (u, v) = if samples_per_pixel > 1 {
                            let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                            let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                            (u, v)
                        } else {
                            let u = (i as f64) / (image_width - 1) as f64;
                            let v = (j as f64) / (image_height - 1) as f64;
                            (u, v)
                        };
                        let r = cam.get_ray(u, v);
                        pixel += r.ray_color(world.clone(), max_depth);
                    }
                    {
                        let mut image = image.lock().unwrap();
                        image[(i, j)] = pixel / samples_per_pixel as f64;
                    }
                }
                scanlines_counter.fetch_sub(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    let mut num = image_height;
    let mut pb = ProgressBar::new(image_height as u64);
    while num > 0 {
        num = scanlines_counter.load(Ordering::Relaxed);
        pb.set((image_height - num) as u64);
        thread::sleep(std::time::Duration::from_millis(200));
    }
    pb.finish();

    for h in handles {
        h.join().unwrap();
    }

    let (render_time, tag) = {
        let mut render_time = start_instant.elapsed().as_secs_f64();
        let tag;
        if render_time > 60.0 {
            render_time /= 60.0;
            tag = "min"
        } else {
            tag = "sec"
        }
        (render_time, tag)
    };
    eprintln!("\nDone! Rendered in {:.3} {}", render_time, tag);

    // Saving
    eprintln!("Writing image...");
    {
        let image = image.lock().unwrap();
        image.write_as_ppm(&mut file)?;
    }
    eprintln!("Image written!");

    Ok(())
}
