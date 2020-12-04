# Ray Tracing in One Weekend in Rust

This is my [Ray Tracing in One Weekend](https://raytracing.github.io) implementation using Rust.

I also improve the render times by paralelizing the code and added some QOL thing to the program interface.

This is the render that it produces:

![Random spheres](./images/random_spheres.png)

More of them are in the [images](./images) folder.

## CLI

```
USAGE:
    ray-tracing [FLAGS] [OPTIONS]

FLAGS:
    -d               Prints configuration of the render.
    -h, --help       Prints help information
        --stdout     Returns the image via the standard output, not saving it to a file.
    -V, --version    Prints version information

OPTIONS:
        --aspect <ASPECT_RATIO>    Aspect ratio of the image. Format: <width>/<height>. [default: 16/9]
    -j <cores>                     Number of computing cores to use. Default is the number of physical cores of the
                                   computer.
        --resolution <HEIGHT>      Vertical resolution of the image. [default: 1440]
    -o <output>                    File to output to. [default: image.ppm]
        --raydepth <RAY_DEPTH>     Maximum ray depth. More depth, more reflects and refractions. [default: 50]
        --spp <SPP>                Samples per pixel. More samples, less noise. [default: 500]
```
