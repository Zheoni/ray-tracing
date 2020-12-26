use vec3::Vec3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

mod solid_color;
pub use solid_color::*;
mod checker_texture;
pub use checker_texture::*;
mod noise_texture;
pub use noise_texture::*;
mod image_texture;
pub use image_texture::*;
