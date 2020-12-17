use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::image::Image;
use crate::vec3::Vec3;
use crate::Config;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use pbr::ProgressBar;

pub struct RenderConfig {
    world: Arc<dyn Hittable>,
    camera: Arc<Camera>,
    image_width: usize,
    image_height: usize,
    aspect_ratio: f64,
    samples_per_pixel: usize,
    max_depth: u32,
    cpus: usize,
    print_debug: bool,
    show_progress_bar: bool,
}

impl RenderConfig {
    pub fn from(config: &Config, world: Arc<dyn Hittable>, camera: Arc<Camera>) -> Self {
        Self {
            world,
            camera,
            image_width: (config.image_height as f64 * config.aspect_ratio).floor() as usize,
            image_height: config.image_height,
            aspect_ratio: config.aspect_ratio,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
            cpus: config.cpus,
            print_debug: config.print_debug,
            show_progress_bar: config.show_progress_bar,
        }
    }
}

pub fn render(config: RenderConfig) -> (Image, Duration) {
    let image = Arc::new(Mutex::new(Image::new(
        config.image_width,
        config.image_height,
    )));

    if config.print_debug {
        eprintln!("Resolution: {}x{}", config.image_width, config.image_height);
        eprintln!("Aspect ratio: {}", config.aspect_ratio);
        eprintln!("SPP: {}", config.samples_per_pixel);
        eprintln!("Ray depth: {}", config.max_depth);
    }

    let mut handles = Vec::new();

    let config = Arc::new(config);
    let scanlines_counter = Arc::new(AtomicUsize::new(0));

    // Calculations on how to distribute the scanlines evenly
    let high = config.image_height / config.cpus
        + if config.image_height % config.cpus == 0 {
            0
        } else {
            1
        };
    let low = config.image_height / config.cpus;

    let n_high = if high == low {
        config.cpus
    } else {
        config.cpus - (config.cpus * high - config.image_height) / (high - low)
    };

    let mut pos = 0;

    eprintln!("Rendering with {} cores...", config.cpus);
    let start_instant = Instant::now();
    for t in 0..config.cpus {
        let cam = Arc::clone(&config.camera);
        let world = Arc::clone(&config.world);
        let image = Arc::clone(&image);
        let scanlines_counter = Arc::clone(&scanlines_counter);
        let config = Arc::clone(&config);

        let w = if t < n_high { high } else { low };
        let start = pos;
        let end = pos + w;
        pos += w;

        let handle = thread::spawn(move || {
            for j in start..end {
                for i in 0..config.image_width {
                    let mut pixel = Vec3::zero();
                    for _ in 0..config.samples_per_pixel {
                        let u =
                            (i as f64 + rand::random::<f64>()) / (config.image_width - 1) as f64;
                        let v =
                            (j as f64 + rand::random::<f64>()) / (config.image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel += r.ray_color(Arc::clone(&world), config.max_depth);
                    }
                    {
                        let mut image = image.lock().unwrap();
                        image[(i, j)] = pixel / config.samples_per_pixel as f64;
                    }
                }
                if config.show_progress_bar {
                    scanlines_counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    if config.show_progress_bar {
        let mut num = 0;
        let mut pb = ProgressBar::new(config.image_height as u64);
        while num < config.image_height {
            num = scanlines_counter.load(Ordering::Relaxed);
            pb.set(num as u64);
            thread::sleep(std::time::Duration::from_millis(200));
        }
        pb.finish();
    }

    for h in handles {
        h.join().unwrap();
    }

    let image = Arc::try_unwrap(image)
        .ok()
        .expect("Cannot own image from main thread")
        .into_inner()
        .expect("Cannot lock image in main thread");
    let elapsed = start_instant.elapsed();
    (image, elapsed)
}
