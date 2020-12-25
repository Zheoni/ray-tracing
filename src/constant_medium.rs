use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::SolidColor;
use crate::texture::Texture;
use vec3::Vec3;

pub struct ConstantMedium<H: Hittable, M: Material> {
    boundary: H,
    phase_function: M,
    neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    #[allow(unused)]
    pub fn new(boundary: H, d: f64, albedo: T) -> Self {
        Self {
            boundary,
            phase_function: Isotropic { albedo },
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<H: Hittable> ConstantMedium<H, Isotropic<SolidColor>> {
    pub fn from_color(boundary: H, d: f64, color: Vec3) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::from_color(color),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut hit2 = self.boundary.hit(r, hit1.t + 0.0001, f64::INFINITY)?;

        hit1.t = hit1.t.max(t_min);
        hit2.t = hit2.t.min(t_max);

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0;
        }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        let point = r.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0); // Some arbitrary value, and u, v
        Some(HitRecord::new(
            r,
            t,
            1.0,
            1.0,
            point,
            normal,
            &self.phase_function,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
