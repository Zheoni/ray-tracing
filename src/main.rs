mod camera;
mod hittable;
mod image;
mod material;
mod objects;
mod ray;
mod render;
mod scenes;
mod vec3;

use camera::Camera;
use render::*;
use vec3::*;

use std::fs::File;
use std::sync::Arc;

#[macro_use]
extern crate clap;
use clap::App;

enum Format {
    PPM,
}

enum Output {
    File(String, Format),
    Stdout(Format),
}

pub struct Config {
    image_height: usize,
    aspect_ratio: f64,
    samples_per_pixel: usize,
    max_depth: u32,
    cpus: usize,
    show_progress_bar: bool,
    print_debug: bool,
    output: Output,
}

fn get_config() -> Config {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let image_height: usize = args
        .value_of("image_height")
        .unwrap()
        .parse()
        .expect("Invalid image height");

    let aspect_ratio = {
        let r = args.value_of("aspect_ratio").unwrap();
        let p: Vec<_> = r.split('/').map(str::parse::<usize>).collect();
        assert_eq!(p.len(), 2, "Invalid aspect ratio format");
        let w = p[0].as_ref().expect("Invalid aspect ratio width");
        let h = p[1].as_ref().expect("Invalid aspect ratio height");
        *w as f64 / *h as f64
    };

    // Antialias / noise
    let samples_per_pixel: usize = args
        .value_of("samples_per_pixel")
        .unwrap()
        .parse()
        .expect("Invalid spp");
    // Max recursive rays
    let max_depth: u32 = args
        .value_of("ray_depth")
        .unwrap()
        .parse()
        .expect("Invalid ray depth");

    let cpus: usize = if let Some(cpus) = args.value_of("cores") {
        cpus.parse().expect("Invalid number of cores")
    } else {
        num_cpus::get_physical()
    };

    let print_debug = args.is_present("debug");

    let format = Format::PPM;
    let output = if args.is_present("stdout") {
        Output::Stdout(format)
    } else {
        let filename: String = args.value_of("output").unwrap_or("image.ppm").to_string();
        Output::File(filename, format)
    };

    Config {
        image_height,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        cpus,
        show_progress_bar: true,
        print_debug,
        output,
    }
}

fn main() -> Result<(), std::io::Error> {
    let config = get_config();

    // Test that the file can be written
    if let Output::File(name, _) = &config.output {
        File::create(name)?;
    }

    // World
    let world = Arc::new(scenes::randon_spheres());

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
        config.aspect_ratio,
        aperture,
        dist_to_focus,
    ));

    // Render
    let (image, elapsed) = render::render(RenderConfig::from(&config, world, cam));

    let (render_time, tag) = {
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
    eprintln!("\nDone! Rendered in {:.3} {}", render_time, tag);

    // Saving
    eprintln!("Writing image...");
    let mut file: Box<dyn std::io::Write> = match config.output {
        Output::File(name, _) => Box::new(File::create(name).expect("Cannot open output file")),
        Output::Stdout(_) => Box::new(std::io::stdout()),
    };

    image.write_as_ppm(&mut file)?;
    eprintln!("Image written!");

    Ok(())
}
