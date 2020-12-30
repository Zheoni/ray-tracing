use ray_tracing::bvh;
use ray_tracing::camera::Camera;
use ray_tracing::hittable::Hittable;
use ray_tracing::image_helper::Image;
use ray_tracing::render::*;
use ray_tracing::scenes;
use vec3::Vec3;

use std::fs::File;

enum Output {
    File(String, image::ImageFormat),
    Stdout(image::ImageFormat),
}

pub struct Config {
    image_height: usize,
    aspect_ratio: f64,
    samples_per_pixel: usize,
    max_bounces: u32,
    threads: usize,
    print_debug: bool,
    output: Output,
    scene_name: String,
    avoid_bvh: bool,
    force_plain_ppm: bool,
}

impl Config {
    pub fn build_render_config(
        &self,
        background_color: Vec3,
        world: Box<dyn Hittable>,
        camera: Camera,
    ) -> RenderConfig {
        RenderConfig {
            world,
            camera,
            background_color,
            image_width: (self.image_height as f64 * self.aspect_ratio).floor() as usize,
            image_height: self.image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            threads: self.threads,
            print_debug: self.print_debug,
        }
    }
}

pub fn get_config() -> Config {
    use clap::Arg;
    use image::ImageFormat;

    let app = clap::App::new("Ray tracing renderer")
    .author("Zheoni <zheoni@outlook.es>")
    .version("0.1.0")
    .about("Ray Tracing in a Weekend implementation in Rust")
    .arg(Arg::with_name("image_height")
        .long("resolution")
        .help("Vertical resolution of the image")
        .value_name("HEIGHT")
        .takes_value(true))
    .arg(Arg::with_name("aspect_ratio")
        .long("aspect")
        .help("Aspect ratio of the image. Format: <width>/<height>  e.g. \"16/9\"")
        .value_name("ASPECT_RATIO")
        .takes_value(true))
    .arg(Arg::with_name("samples_per_pixel")
        .long("spp")
        .help("Samples per pixel. More samples, less noise but more computation.")
        .value_name("SPP")
        .takes_value(true))
    .arg(Arg::with_name("max_bounces")
        .long("maxbounces")
        .help("Maximum depth of the ray tracing algorithm. More depth, more reflects and refractions but more computation.")
        .value_name("MAX_BOUNCES")
        .takes_value(true))
    .arg(Arg::with_name("threads")
        .long("threads")
        .short("j")
        .help("Number of worker threds to use. Defaults to the number of physical cores available.")
        .takes_value(true))
    .arg(Arg::with_name("output")
        .long("output")
        .short("o")
        .help("File to output to.")
        .takes_value(true))
    .arg(Arg::with_name("stdout")
        .long("stdout")
        .help("Returns the image via the standard output, not saving it to a file.")
        .conflicts_with("output"))
    .arg(Arg::with_name("format")
        .long("format")
        .short("F")
        .help("Explicitly select image format, if not given it's inferred from output file extension.")
        .takes_value(true))
    .arg(Arg::with_name("plain_ppm")
        .long("plain_ppm")
        .help("Use plain ppm format, enconding the image into an ASCII ppm file.")
        .conflicts_with("format"))
    .arg(Arg::with_name("debug")
        .long("debug")
        .short("d")
        .help("Increases the logging level."))
    .arg(Arg::with_name("avoid_bvh")
        .long("avoid_bvh")
        .help("Avoid to build a BVH with all the objects. May be faster to render a simple scene."))
    .arg(Arg::with_name("scene")
        .takes_value(true)
        .required(true)
        .possible_values(&scenes::get_scenes()));
    let args = app.get_matches();

    let image_height: usize = args
        .value_of("image_height")
        .unwrap_or("1080")
        .parse()
        .expect("Invalid image height");

    let aspect_ratio = {
        let r = args.value_of("aspect_ratio").unwrap_or("1/1");
        let p: Vec<_> = r.split('/').map(str::parse::<usize>).collect();
        assert_eq!(p.len(), 2, "Invalid aspect ratio format");
        let w = p[0].as_ref().expect("Invalid aspect ratio width");
        let h = p[1].as_ref().expect("Invalid aspect ratio height");
        *w as f64 / *h as f64
    };

    // Antialias / noise
    let samples_per_pixel: usize = args
        .value_of("samples_per_pixel")
        .unwrap_or("3000")
        .parse()
        .expect("Invalid spp");
    // Max recursive rays
    let max_bounces: u32 = args
        .value_of("ray_depth")
        .unwrap_or("50")
        .parse()
        .expect("Invalid ray depth");

    let threads: usize = if let Some(threads) = args.value_of("threads") {
        threads.parse().expect("Invalid number of threads")
    } else {
        num_cpus::get_physical()
    };

    let print_debug = args.is_present("debug");

    let extension = args.value_of("format");
    let output = if args.is_present("stdout") {
        let extension = extension.unwrap_or("ppm");
        let format = ImageFormat::from_extension(extension).expect("Unsuported image format");
        Output::Stdout(format)
    } else {
        let filename: String = args.value_of("output").unwrap_or("image.ppm").to_string();
        let format = if let Some(extension) = extension {
            ImageFormat::from_extension(extension)
        } else {
            ImageFormat::from_path(filename.clone()).ok()
        };
        let format = format.expect("Unsuported image format");
        Output::File(filename, format)
    };

    let scene_name = args.value_of("scene").unwrap_or("cornell_box").to_string();

    let avoid_bvh = args.is_present("avoid_bvh");
    let force_plain_ppm = args.is_present("plain_ppm");

    Config {
        image_height,
        aspect_ratio,
        samples_per_pixel,
        max_bounces,
        threads,
        print_debug,
        output,
        scene_name,
        avoid_bvh,
        force_plain_ppm,
    }
}

fn main() -> Result<(), image::ImageError> {
    let config = get_config();

    // Test that the file can be written
    if let Output::File(name, _) = &config.output {
        File::create(name)?;
    }

    // World
    let scene =
        scenes::get_scene_from_name(&config.scene_name).expect("Cannot build unknown scene");
    let world: Box<dyn Hittable> = if config.avoid_bvh {
        Box::new(scene.world)
    } else {
        Box::new(bvh::BVH::from_scene(scene.world, 0.0, 1.0))
    };

    // Camera
    let cam = Camera::new(&scene.camera_config, config.aspect_ratio);

    // Render
    if config.print_debug {
        eprintln!("Scene: {}", config.scene_name);
        eprintln!("Aspect ratio: {}", config.aspect_ratio);
        eprintln!("SPP: {}", config.samples_per_pixel);
        eprintln!("Max bounces: {}", config.max_bounces);
        eprintln!("Global BVH: {}", !config.avoid_bvh);
    }

    let (img, elapsed) = render(config.build_render_config(scene.background_color, world, cam));

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
