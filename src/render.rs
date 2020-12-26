use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::image_helper::Image;
use crate::Config;
use crate::ray::Ray;
use image::RgbImage;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use vec3::Vec3;

pub struct RenderConfig {
    world: Box<dyn Hittable>,
    camera: Camera,
    background_color: Vec3,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: u32,
    cpus: usize,
    print_debug: bool,
}

impl RenderConfig {
    pub fn from(
        config: &Config,
        background_color: Vec3,
        world: Box<dyn Hittable>,
        camera: Camera,
    ) -> Self {
        Self {
            world,
            camera,
            background_color,
            image_width: (config.image_height as f64 * config.aspect_ratio).floor() as usize,
            image_height: config.image_height,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
            cpus: config.cpus,
            print_debug: config.print_debug,
        }
    }
}

pub fn render(
    RenderConfig {
        image_width: width,
        image_height: height,
        samples_per_pixel: spp,
        world,
        max_depth,
        camera,
        background_color: background,
        print_debug,
        cpus,
    }: RenderConfig,
) -> (RgbImage, Duration) {
    if print_debug {
        eprintln!("Resolution: {}x{}", width, height);
    }

    eprintln!("Rendering with {} cores...", cpus);

    let (tx, rx) = mpsc::channel::<bool>();

    let num_pixels = width * height;
    let mut pb = pbr::ProgressBar::new(num_pixels as u64);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));
    pb.message("Pixels: ");
    // Progress bar thread
    let progress_thread = std::thread::spawn(move || {
        while rx.recv().is_ok() {
            pb.inc();
        }
        pb.finish();
    });

    let world = Arc::new(world);

    let start_instant = Instant::now();
    // gives ownership of tx, therefore when function ends, tx is disconnected
    let image = RgbImage::par_compute(width, height, cpus, tx, move |i, j| {
        let pixel: Vec3 = (0..spp)
            .map(|_| {
                let mut rng = thread_rng();
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = camera.get_ray(u, v);
                ray_color(r, &background, &**world, max_depth)
            })
            .sum();
        pixel / spp as f64
    });
    let elapsed = start_instant.elapsed();

    progress_thread.join().expect("Progress thread panicked");

    (image, elapsed)
}

#[cfg(feature = "recursive-tracer")]
pub fn ray_color(r: Ray, background: &Vec3, world: &dyn Hittable, depth: u32) -> Vec3 {
    // If maximum number of rays
    if depth == 0 {
        return Vec3::zero();
    }

    // If hit with some object. The min hit distance is not 0 because
    // of course float precission. Not every ray will match exactly with 0.0
    if let Some(hit) = world.hit(&r, 0.001, f64::INFINITY) {
        // if hits something

        // Calculate the light emitted
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.point);

        if let Some((attenuation, scattered)) = hit.material.scatter(&r, &hit) {
            // if material scatters
            emitted + attenuation * ray_color(scattered, background, world, depth - 1)
        } else {
            // if it not, only emits
            emitted
        }
    } else {
        // if hits nothing, the background is visible
        *background
    }
}

#[cfg(not(feature = "recursive-tracer"))]
fn ray_color(mut ray: Ray, background: &Vec3, world: &dyn Hittable, max_bounces: u32) -> Vec3 {
    // Light accumulated in the in the ray after all the interactions
    let mut accum = Vec3::zero();
    // Total attenuation
    let mut strength = Vec3::one();

    let mut bounces = 0;

    // If hit with some object. The min hit distance is not 0 because
    // of float precission. Not every ray will match exactly with 0.0
    while let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
        // Add the light emmited in the hit
        accum += strength * hit.material.emitted(hit.u, hit.v, &hit.point);

        // if material scatters
        if let Some((attenuation, scattered)) = hit.material.scatter(&ray, &hit) {
            // change current ray
            ray = scattered;
            // update ray strength
            strength *= attenuation;
        } else {
            // if the ray is absorbed, no more calculations needed
            return accum;
        }

        if bounces == max_bounces {
            return accum;
        }

        bounces += 1;
    }

    *background
}
