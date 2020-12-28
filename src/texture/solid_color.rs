use super::*;

/// Texture of a solid color
#[derive(Clone)]
pub struct SolidColor {
    /// Color of the texture
    pub color: Vec3,
}

impl SolidColor {
    /// Creates a new [SolidColor] texture with the given RGB values
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
