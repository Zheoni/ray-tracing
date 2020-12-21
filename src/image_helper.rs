use crate::Clampable;
use image::Pixel;
use rayon::prelude::*;
use std::io::{Error, Write};
use std::ops::Mul;
use vec3::Vec3;

#[inline]
fn f64_subpixel_to_u8(p: f64) -> u8 {
    // sqrt => gamma = 2.0
    p.sqrt().clamp_(0.0, 0.999).mul(256.0).floor() as u8
}

pub trait Image {
    fn par_compute(
        width: usize,
        height: usize,
        tx: std::sync::mpsc::Sender<bool>,
        f: impl Fn(usize, usize) -> Vec3 + Sync,
    ) -> Self;
    fn write_as_plain_ppm(&self, file: &mut impl Write) -> Result<(), Error>;
}

impl Image for image::RgbImage {
    fn par_compute(
        width: usize,
        height: usize,
        tx: std::sync::mpsc::Sender<bool>,
        f: impl Fn(usize, usize) -> Vec3 + Sync,
    ) -> Self {
        let pixels: Vec<u8> = (0..height)
            .into_par_iter()
            .rev()
            .map_with(tx, |tx, j| {
                (0..width)
                    .map(|i| {
                        let p = f(i, j);
                        tx.send(true).unwrap();
                        p
                    })
                    .collect::<Vec<Vec3>>()
            })
            .flatten()
            .flat_map(|v| Vec::from(v.v))
            .map(f64_subpixel_to_u8)
            .collect();

        image::RgbImage::from_vec(width as u32, height as u32, pixels)
            .expect("Image could not be built from pixels")
    }

    fn write_as_plain_ppm(&self, file: &mut impl Write) -> Result<(), Error> {
        let header = format!("P3\n{} {}\n255\n", self.width(), self.height());
        file.write_all(&header.as_bytes())?;

        let buffer: String = self
            .pixels()
            .map(|p| {
                let chs = p.channels();
                format!("{} {} {}\n", chs[0], chs[1], chs[2])
            })
            .collect();

        file.write_all(&buffer.as_bytes())?;
        Ok(())
    }
}
