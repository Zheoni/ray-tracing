mod aabb;
mod bvh;
mod camera;
mod cli;
mod constant_medium;
mod hittable;
mod image_helper;
mod material;
mod objects;
mod ray;
mod render;
mod scenes;
mod texture;

use camera::Camera;
use hittable::Hittable;
use image_helper::Image;
use render::*;

use std::fs::File;
use std::sync::Arc;

pub trait Clampable {
    #[inline]
    fn clamp_(self, min: Self, max: Self) -> Self
    where
        Self: PartialOrd + Sized,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Clampable for f64 {}

enum Output {
    File(String, image::ImageFormat),
    Stdout(image::ImageFormat),
}

pub struct Config {
    image_height: usize,
    aspect_ratio: f64,
    samples_per_pixel: usize,
    max_depth: u32,
    cpus: usize,
    print_debug: bool,
    output: Output,
    scene_name: String,
    avoid_bvh: bool,
    force_plain_ppm: bool,
}

fn main() -> Result<(), image::ImageError> {
    let config = cli::get_config();

    // Test that the file can be written
    if let Output::File(name, _) = &config.output {
        File::create(name)?;
    }

    // World
    let scene =
        scenes::get_scene_from_name(&config.scene_name).expect("Cannot build unknown scene");
    let world: Arc<dyn Hittable> = if config.avoid_bvh {
        Arc::new(scene.world)
    } else {
        Arc::new(bvh::BVHNode::from_scene(scene.world, 0.0, 1.0))
    };

    // Camera
    let cam = Arc::new(Camera::new(&scene.camera_config, config.aspect_ratio));

    // Render
    if config.print_debug {
        eprintln!("Scene: {}", config.scene_name);
        eprintln!("Aspect ratio: {}", config.aspect_ratio);
        eprintln!("SPP: {}", config.samples_per_pixel);
        eprintln!("Ray depth: {}", config.max_depth);
        eprintln!("Global BVH: {}", !config.avoid_bvh);
    }

    let (img, elapsed) = render::render(RenderConfig::from(
        &config,
        scene.background_color,
        world,
        cam,
    ));

    let (render_time, unit) = {
        let mut render_time = elapsed.as_secs_f64();
        let unit;
        if render_time > 60.0 {
            render_time /= 60.0;
            unit = "min"
        } else {
            unit = "sec"
        }
        (render_time, unit)
    };
    eprintln!("\nDone! Rendered in {:.3} {}", render_time, unit);

    // Saving
    eprintln!("Writing image...");

    if config.print_debug {
        eprintln!(
            "\tUsing \"{}\" as format",
            if config.force_plain_ppm {
                "plain ppm".to_string()
            } else {
                format!(
                    "{:?}",
                    match config.output {
                        Output::File(_, format) => format,
                        Output::Stdout(format) => format,
                    }
                )
            }
        );
        eprintln!(
            "\tWriting it to \"{}\"",
            match config.output {
                Output::File(ref name, _) => name,
                Output::Stdout(_) => "stdout",
            }
        );
    }

    if config.force_plain_ppm {
        let mut file: Box<dyn std::io::Write> = match config.output {
            Output::File(name, _) => Box::new(File::create(name).expect("Cannot open output file")),
            Output::Stdout(_) => Box::new(std::io::stdout()),
        };
        img.write_as_plain_ppm(&mut file)?;
    } else {
        let img = image::DynamicImage::ImageRgb8(img);
        match config.output {
            Output::File(name, format) => {
                let mut file = File::create(name).expect("Cannot open ouput file");
                img.write_to(&mut file, format)?;
            }
            Output::Stdout(format) => {
                img.write_to(&mut std::io::stdout(), format)?;
            }
        };
    }
    eprintln!("Image written!");

    Ok(())
}
