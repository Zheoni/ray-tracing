use crate::Clampable;
use image::Pixel;
use std::io::{Error, Write};
use std::ops::Mul;
use std::sync::{mpsc, Arc};
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
        threads: usize,
        tx: mpsc::Sender<bool>,
        f: impl Fn(usize, usize) -> Vec3 + Sync + Send + 'static,
    ) -> Self;
    fn write_as_plain_ppm(&self, file: &mut impl Write) -> Result<(), Error>;
}

fn compute_sanlines(
    scanlines: std::ops::Range<usize>,
    width: usize,
    progress_tx: mpsc::Sender<bool>,
    f: Arc<impl Fn(usize, usize) -> Vec3>,
) -> Vec<u8> {
    scanlines
        .rev()
        .flat_map(|j| {
            (0..width)
                .map(|i| {
                    let p = f(i, j);
                    progress_tx.send(true).unwrap();
                    p
                })
                .collect::<Vec<Vec3>>()
        })
        .flat_map(|v| Vec::from(v.v))
        .map(f64_subpixel_to_u8)
        .collect()
}

impl Image for image::RgbImage {
    fn par_compute(
        width: usize,
        height: usize,
        threads: usize,
        tx: mpsc::Sender<bool>,
        f: impl Fn(usize, usize) -> Vec3 + Sync + Send + 'static,
    ) -> Self {
        // Calculations on how to distribute the scanlines evenly
        let low = height / threads;
        let high = low + if height % threads == 0 { 0 } else { 1 };

        let n_high = if high == low {
            threads
        } else {
            threads - (threads * high - height) / (high - low)
        };

        let mut pos = 0;

        let mut handles = Vec::new();

        let f = Arc::new(f);

        for t in 0..threads {
            let w = if t < n_high { high } else { low };
            let start = pos;
            let end = pos + w;
            pos += w;

            let tx = tx.clone();
            let f = Arc::clone(&f);

            let handle = std::thread::spawn(move || compute_sanlines(start..end, width, tx, f));
            handles.push(handle);
        }

        let pixels = handles
            .into_iter()
            .rev()
            .flat_map(|h| h.join().unwrap())
            .collect::<Vec<u8>>();

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
