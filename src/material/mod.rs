//! Contains all the different types of materials
//! the ray tracer has.
//!
//! More materials can be created implementing
//! the [Material] trait. With this in mind, the [reflect] and [refract]
//! functions are exposed to be used if wanted.

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use std::ops::Neg;
use vec3::Vec3;

/// The [Material] trait has to be implemented for every material type.
/// It offers 2 methods used to determine how light iteracts with the material.
pub trait Material: Send + Sync {
    /// Checks if the light scatters when the material is hit,
    /// returning the light attenuation ([Vec3] interpreted as color) scattered
    /// and a new [Ray] with the new direction and origin.
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
    #[allow(unused)]
    /// Returns the light emitted by the material. It has a default implementation
    /// where the material does not emit light.
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

/// Returns the direction of a reflected ray of light on a hit.
/// # Arguments
/// * `v`: vector with the original direction
/// * `n`: surface normal of the hit point
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * (v.dot(n)) * (*n)
}

/// Returns the direction of a refracted ray of light when it crossed
/// from one medium to another.
/// # Arguments
/// * `uv`: vector with the original direction
/// * `n`: surface normal of the hit point
/// * `etai_over_etat`: index of refraction
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = uv.neg().dot(n).min(1.0);
    let r_out_perp = etai_over_etat * ((*uv) + cos_theta * (*n));
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * (*n);
    r_out_perp + r_out_parallel
}

mod lambertian;
pub use lambertian::*;
mod metal;
pub use metal::*;
mod dielectric;
pub use dielectric::*;
mod diffuse_light;
pub use diffuse_light::*;
mod isotropic;
pub use isotropic::*;
