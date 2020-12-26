use super::*;
use crate::Clampable;
use image::io::Reader as ImageReader;

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
