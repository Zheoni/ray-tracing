use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use vec3::Vec3;

pub struct MovingSphere<M: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_now = self.center(r.time);
        let oc = r.origin - center_now;
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
        let outward_normal = (hit_point - center_now) / self.radius;
        let (u, v) = super::sphere::Sphere::<M>::get_sphere_uv(&outward_normal);
        let record = HitRecord::new(&r, root, u, v, hit_point, outward_normal, &self.material);

        Some(record)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB {
            minimum: self.center(time0) - Vec3::splat(self.radius),
            maximum: self.center(time0) + Vec3::splat(self.radius),
        };
        let box1 = AABB {
            minimum: self.center(time1) - Vec3::splat(self.radius),
            maximum: self.center(time1) + Vec3::splat(self.radius),
        };
        Some(surrounding_box(&box0, &box1))
    }
}
