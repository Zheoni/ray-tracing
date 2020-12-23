use crate::Clampable;
use image::io::Reader as ImageReader;
use perlin_noise::PNG;
use vec3::Vec3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    #[allow(unused)]
    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    odd: T1,
    even: T2,
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn from_colors(odd: Vec3, even: Vec3) -> Self {
        Self {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let p_10 = p.scale(10.0);
        let sines = p_10.x().sin() * p_10.y().sin() * p_10.z().sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: PNG,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: PNG::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::one() * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turbulence(p)).sin())
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    img: image::RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = ImageReader::open(filename)?.decode()?.into_rgb8();
        Ok(Self { img })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = u.clamp_(0.0, 1.0);
        let v = 1.0 - v.clamp_(0.0, 1.0);

        let width = self.img.width();
        let height = self.img.height();

        let i = {
            let i = (u * (width as f64)) as u32;
            if i < width {
                i
            } else {
                width - 1
            }
        };
        let j = {
            let j = (v * (height as f64)) as u32;
            if j < height {
                j
            } else {
                height - 1
            }
        };

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);

        Vec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
    }
}
