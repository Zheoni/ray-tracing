use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::image_helper::Image;
use crate::Config;
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
                r.ray_color(&background, &**world, max_depth)
            })
            .sum();
        pixel / spp as f64
    });
    let elapsed = start_instant.elapsed();

    progress_thread.join().expect("Progress thread panicked");

    (image, elapsed)
}
