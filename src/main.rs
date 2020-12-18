mod aabb;
mod bvh;
mod camera;
mod hittable;
mod image;
mod material;
mod objects;
mod ray;
mod render;
mod scenes;
mod texture;
mod vec3;

use camera::Camera;
use render::*;

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
    scene_name: String,
}

fn get_config() -> Config {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let image_height: usize = args
        .value_of("image_height")
        .unwrap_or("1440")
        .parse()
        .expect("Invalid image height");

    let aspect_ratio = {
        let r = args.value_of("aspect_ratio").unwrap_or("16/9");
        let p: Vec<_> = r.split('/').map(str::parse::<usize>).collect();
        assert_eq!(p.len(), 2, "Invalid aspect ratio format");
        let w = p[0].as_ref().expect("Invalid aspect ratio width");
        let h = p[1].as_ref().expect("Invalid aspect ratio height");
        *w as f64 / *h as f64
    };

    // Antialias / noise
    let samples_per_pixel: usize = args
        .value_of("samples_per_pixel")
        .unwrap_or("500")
        .parse()
        .expect("Invalid spp");
    // Max recursive rays
    let max_depth: u32 = args
        .value_of("ray_depth")
        .unwrap_or("50")
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

    let scene_name = args.value_of("scene").unwrap_or("spheres").to_string();

    Config {
        image_height,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        cpus,
        show_progress_bar: true,
        print_debug,
        output,
        scene_name,
    }
}

fn main() -> Result<(), std::io::Error> {
    let config = get_config();

    // Test that the file can be written
    if let Output::File(name, _) = &config.output {
        File::create(name)?;
    }

    // World
    let (scene, camera_config) =
        scenes::gen_scene_from_name(&config).expect("Cannot build unknown scene");
    let scene_tree = bvh::BVHNode::from_scene(scene, 0.0, 1.0);
    let world = Arc::new(scene_tree);

    // Camera
    let cam = Arc::new(Camera::new(&camera_config));

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
