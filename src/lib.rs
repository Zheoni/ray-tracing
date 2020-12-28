#![allow(clippy::many_single_char_names)]
//! Ray Tracing in One Weekend book series implemented in Rust.
//!
//! This crate is intended to be a CLI executable. However, this
//! lib is exposed to be used if wanted.

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

/// This trait attaches the [`clamp_`](Self::clamp_) method to a type.
///
/// [`clamp_`](Self::clamp_) is the only method it has and may be useful while `clamp`
/// is experimental. It has an underscore to avoid future conflicts.
pub trait Clampable {
    /// Restricts the value of `self` to be in [`min`, `max`]. It offers a
    /// default implementation if the type implements the [PartialOrd] and
    /// [Sized] traits.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::Clampable;
    /// assert_eq!((2.0).clamp_(0.0, 3.0), 2.0);
    /// assert_eq!((4.0).clamp_(0.0, 3.0), 3.0);
    /// assert_eq!((-3.0).clamp_(0.0, 3.0), 0.0);
    /// ```
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
