#![allow(clippy::many_single_char_names)]

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod constant_medium;
pub mod hittable;
pub mod image_helper;
pub mod material;
pub mod object;
pub mod ray;
pub mod render;
pub mod scenes;
pub mod texture;

pub trait Clampable {
    #[inline]
    fn clamp_(self, min: Self, max: Self) -> Self
    where
        Self: PartialOrd + Sized,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Clampable for f64 {}
