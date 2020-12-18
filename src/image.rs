use std::io::{Error, Write};
use std::ops::{Index, IndexMut};
use vec3::Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn vec3_as_color_string(color: &Vec3) -> String {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Gamma=2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    let r = (256.0 * clamp(r, 0.0, 0.999)).floor() as u8;
    let g = (256.0 * clamp(g, 0.0, 0.999)).floor() as u8;
    let b = (256.0 * clamp(b, 0.0, 0.999)).floor() as u8;
    format!("{} {} {}\n", r, g, b)
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec3>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            pixels: vec![Vec3::zero(); height * width],
        }
    }

    pub fn write_as_ppm(&self, file: &mut impl Write) -> Result<(), Error> {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(&header.as_bytes())?;

        let mut buffer = String::with_capacity(12 * self.width * self.height);

        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let color_str = vec3_as_color_string(&self.pixels[i + j * self.width]);
                buffer.push_str(&color_str);
            }
        }
        file.write_all(&buffer.as_bytes())?;
        Ok(())
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Vec3;
    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.pixels[i.0 + i.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[i.0 + i.1 * self.width]
    }
}
