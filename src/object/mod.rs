//! Contains all the diferent geometric objects and trasformations to
//! those objects supported.
//!
//! Objects implement the [Hittable] trait have a [Material].
//! Transformations owns some [Hittable] and also implements the [Hittable]
//! trait by themselfs.

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::ray::Ray;
use vec3::{Axis, Vec3};

mod sphere;
pub use sphere::*;
mod moving_sphere;
pub use moving_sphere::*;
mod aarect;
pub use aarect::*;
mod block;
pub use block::*;
mod rotate;
pub use rotate::*;
mod translate;
pub use translate::*;
