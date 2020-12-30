use super::*;
use perlin_noise::PNG;

/// Texture of noise similar to marble
///
/// This texture uses internally a Perlin Noise Generator
#[derive(Clone)]
pub struct NoiseTexture {
    noise: PNG,
    scale: f64,
}

impl NoiseTexture {
    /// Creates a new [NoiseTexture] and initializes its internal Perlin Noise Generator.
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
