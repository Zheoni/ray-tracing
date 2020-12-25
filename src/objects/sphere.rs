use super::*;

#[derive(Clone)]
pub struct Sphere<M: Material + Clone> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

#[inline]
pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    use std::f64::consts::{PI, TAU};
    use std::ops::Neg;
    let theta = p.y().neg().acos();
    let phi = p.z().neg().atan2(p.x()) + PI;

    let u = phi / (TAU);
    let v = theta / PI;
    (u, v)
}

impl<M: Material + Clone> Hittable for Sphere<M> {
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
        let (u, v) = get_sphere_uv(&outward_normal);
        let record = HitRecord::new(&r, root, u, v, hit_point, outward_normal, &self.material);

        Some(record)
    }

    #[inline]
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - Vec3::splat(self.radius),
            maximum: self.center + Vec3::splat(self.radius),
        })
    }
}
