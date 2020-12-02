mod camera;
mod hittable;
mod image;
mod material;
mod objects;
mod ray;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use image::Image;
use vec3::*;

use std::rc::Rc;

use rand::Rng;

fn gen_random_scene() -> HittableList {
    use material::*;
    use objects::sphere::Sphere;

    let mut world = HittableList::default();

    // Add the ground
    world.add(Rc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Rc::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
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
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    Rc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    Rc::new(Metal { albedo, fuzz })
                } else {
                    // glass
                    Rc::new(Dielectric {
                        index_refraction: 1.5,
                    })
                };
                world.add(Rc::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }))
            }
        }
    }

    // Big glass boi
    world.add(Rc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric {
            index_refraction: 1.5,
        }),
    }));

    // Big diffuse boi
    world.add(Rc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    }));

    // Big metal boi
    world.add(Rc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    world
}

fn main() -> Result<(), std::io::Error> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_height: usize = 1200;
    let image_width: usize = (image_height as f64 * aspect_ratio).floor() as usize;

    let mut image = Image::new(image_width, image_height);
    
    let samples_per_pixel: usize = 1; // Antialias (100 es un buen numero)
    let max_depth = 50; // Max recursive rays

    // World
    let world = gen_random_scene();

    // Camera
    let lookfrom = Vec3::new(12.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>5}", j);
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
                pixel += r.ray_color(&world, max_depth);
            }
            image[(i, j)] = pixel / samples_per_pixel as f64;
        }
    }
    eprintln!("\nDone!");

    // Saving
    eprintln!("Writing image...");
    let mut file = std::fs::File::create("image.ppm")?;
    image.write_as_ppm(&mut file)?;
    eprintln!("Image written!");

    Ok(())
}
