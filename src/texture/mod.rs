//! Contains the supported textures.
//!
//! More textures can be created implementing the [Texture] trait.

use vec3::Vec3;

/// Trait that the supported textures have to implement.
pub trait Texture: Sync + Send {
    /// Color of the texture in the surface coordinates (`u`, `v`)
    /// and the hit point `p`.
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
