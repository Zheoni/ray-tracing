use crate::scenes::get_scenes;
use crate::Config;
use crate::Output;

fn get_app() -> clap::App<'static, 'static> {
    use clap::Arg;
    clap::App::new("Ray tracing renderer")
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
		.arg(Arg::with_name("ray_depth")
			.long("raydepth")
			.help("Maximum ray depth. More depth, more reflects and refractions but more computation.")
			.value_name("RAY_DEPTH")
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
			.possible_values(&get_scenes()))
}

pub fn get_config() -> Config {
    use image::ImageFormat;

    let app = get_app();
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
        max_depth,
        cpus,
        print_debug,
        output,
        scene_name,
        avoid_bvh,
        force_plain_ppm,
    }
}
