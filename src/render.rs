use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::image_helper::Image;
use crate::ray::Ray;
use image::RgbImage;
use rand::prelude::*;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use vec3::Vec3;

/// Configuration of a render call
pub struct RenderConfig {
    pub world: Box<dyn Hittable>,
    pub camera: Camera,
    pub background_color: Vec3,
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: u32,
    pub threads: usize,
    pub print_debug: bool,
}

/// Renders an image from a given [RenderConfig]. Returns the rendered
/// [RgbImage] and a [Duration] telling us how long the render took.
pub fn render(
    RenderConfig {
        image_width: width,
        image_height: height,
        samples_per_pixel: spp,
        world,
        max_bounces,
        camera,
        background_color: background,
        print_debug,
        threads,
    }: RenderConfig,
) -> (RgbImage, Duration) {
    if print_debug {
        eprintln!("Resolution: {}x{}", width, height);
    }

    eprintln!("Rendering with {} threads...", threads);

    let (tx, rx) = mpsc::channel::<bool>();

    let num_pixels = width * height;
    let mut pb = pbr::ProgressBar::on(std::io::stderr(), num_pixels as u64);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));
    pb.message("Pixels: ");
    // Progress bar thread
    let progress_thread = std::thread::spawn(move || {
        while rx.recv().is_ok() {
            pb.inc();
        }
        pb.finish();
    });

    let start_instant = Instant::now();
    // gives ownership of tx, therefore when function ends, tx is disconnected
    let image = RgbImage::par_compute(width, height, threads, tx, move |i, j| {
        let pixel: Vec3 = (0..spp)
            .map(|_| {
                let mut rng = thread_rng();
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = camera.get_ray(u, v);
                ray_color(r, &background, world.as_ref(), max_bounces)
            })
            .sum();
        pixel / spp as f64
    });
    let elapsed = start_instant.elapsed();

    progress_thread.join().expect("Progress thread panicked");

    (image, elapsed)
}

fn ray_color(r: Ray, background: &Vec3, world: &dyn Hittable, depth: u32) -> Vec3 {
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
