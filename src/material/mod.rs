use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use std::ops::Neg;
use vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
    #[allow(unused)]
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * (v.dot(n)) * (*n)
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
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
