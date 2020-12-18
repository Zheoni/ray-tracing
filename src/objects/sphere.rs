use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<M>,
}

impl<M: Material> Sphere<M> {
    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        use std::f64::consts::PI;
        let theta = -p.y().acos();
        let phi = -p.z().atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = {
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }
            root
        };

        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let (u, v) = Self::get_sphere_uv(&outward_normal);
        let record = HitRecord::new(
            &r,
            root,
            u,
            v,
            hit_point,
            outward_normal,
            self.material.as_ref(),
        );

        Some(record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - Vec3::splat(self.radius),
            maximum: self.center + Vec3::splat(self.radius),
        })
    }
}
